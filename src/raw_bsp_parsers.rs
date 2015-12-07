use nom::{IResult,le_u32,le_i32,le_u8};
use nom::Err;
use nom::IResult::*;
use directory_header::*;
use helpers::*;
use raw_bsp::*;
use texture_flags::*;
use std::str::from_utf8;

named! {
    parse_entity_value <EntityValue>,
    alt!(
        chain!(
                       tag!(b"*")    ~
            reference: parse_str_int ,
            || { EntityValue::ModelRef(reference) }
        ) |
        chain!(
                       tag!(b"t")    ~
            reference: parse_str_int ,
            || { EntityValue::TargetRef(reference) }
        ) |
        chain!(
            first: parse_str_int        ~
                   mandatory_whitespace ~
            secnd: parse_str_int        ~
                   mandatory_whitespace ~
            third: parse_str_int        ~
                   whitespace ,
            || { EntityValue::IVec3([first, secnd, third]) }
        ) |
        chain!(
            first: parse_str_float      ~
                   mandatory_whitespace ~
            secnd: parse_str_float      ~
                   mandatory_whitespace ~
            third: parse_str_float      ~
                   whitespace  ,
            || { EntityValue::Vec3([first, secnd, third]) }
        ) |
        chain!(
            first: parse_str_int        ~
                   mandatory_whitespace ~
            secnd: parse_str_int        ~
                   whitespace ,
            || { EntityValue::IVec2([first, secnd]) }
        ) |
        chain!(
            first: parse_str_float      ~
                   mandatory_whitespace ~
            secnd: parse_str_float      ~
                   whitespace ,
            || { EntityValue::Vec2([first, secnd]) }
        ) |
        map!(parse_str_int, EntityValue::Int) |
        map!(parse_str_float, EntityValue::Float) |
        map!(take_s_until!("\""), EntityValue::Text)
    )
}

fn parse_entity(i: &[u8]) -> IResult<&[u8], Entity> {
    use nom::{Err, ErrorKind};

    let (rest, keyvals) = itry!(
        chain!(i,
                    whitespace ~
                    char!('{')  ~
                    whitespace  ~
            params: many0!(
                chain!(
                           char!('\"')        ~
                    name:  take_s_until!("\"")  ~
                           char!('\"')        ~
                           whitespace         ~
                           char!('\"')        ~
                    value: parse_entity_value ~
                           char!('\"')        ~
                           whitespace         ,
                    || { (name, value) }
                )
            )                   ~
                    char!('}')  ,
            || {
                params
            }
        )
    );

    fn byte_arrays_and_val_to_entity(
        v: Vec<(String, EntityValue)>
    ) -> Entity {
        Entity {
            parameters: v.into_iter().collect()
        }
    }

    Done(
        rest,
        byte_arrays_and_val_to_entity(keyvals)
    )
}

named! {
    parse_texture<Texture>,
    chain!(
        name:          take_s!(64) ~
        surface_flags: le_u32      ~
        content_flags: le_u32      ,
        || {
            Texture {
                name: name,
                surface_flags: SurfaceFlags::from_bits_truncate(
                    surface_flags
                ),
                content_flags: ContentFlags::from_bits_truncate(
                    content_flags
                ),
            }
        }
    )
}

named! {
    parse_plane<Plane>,
    chain!(
        normal:   parse_vec3 ~
        distance: le_f32     ,
        || {
            Plane {
                normal: normal,
                distance: distance,
            }
        }
    )
}

named! {
    parse_raw_node<RawNode>,
    chain!(
        plane:   le_i32       ~
        children: pair!(
            le_i32,
            le_i32
        )                     ~
        min:     parse_ivec3  ~
        max:     parse_ivec3  ,
        || {
            RawNode {
                plane: plane,
                children: children,
                min: min,
                max: max,
            }
        }
    )
}

named! {
    parse_vertex<Vertex>,
    chain!(
        position:        parse_vec3     ~
        surface_coords:  parse_vec2     ~
        lightmap_coords: parse_vec2     ~
        normal:          parse_vec3     ~
        color:           take_exact!(4) ,
        || {
            Vertex {
                position: position,
                surface_coords: surface_coords,
                lightmap_coords: lightmap_coords,
                normal: normal,
                color: color,
            }
        }
    )
}

named! {
    parse_raw_leaf<RawLeaf>,
    chain!(
        visdata_cluster:  le_i32      ~
        areaportal_area:  le_i32      ~
        min:              parse_ivec3 ~
        max:              parse_ivec3 ~
        first_leaf_face:  le_i32      ~
        num_leaf_faces:   le_i32      ~
        first_leaf_brush: le_i32      ~
        num_leaf_brushes: le_i32      ,
        || {
            RawLeaf {
                visdata_cluster: visdata_cluster,
                areaportal_area: areaportal_area,
                min: min,
                max: max,
                first_leaf_face: first_leaf_face,
                num_leaf_faces: num_leaf_faces,
                first_leaf_brush: first_leaf_brush,
                num_leaf_brushes: num_leaf_brushes,
            }
        }
    )
}

named! {
    parse_raw_leaf_face<RawLeafFace>,
    chain!(
        index: le_i32 ,
        || {
            RawLeafFace {
                index: index,
            }
        }
    )
}

named! {
    parse_raw_leaf_brush<RawLeafBrush>,
    chain!(
        index: le_i32 ,
        || {
            RawLeafBrush {
                index: index,
            }
        }
    )
}

named! {
    parse_raw_model<RawModel>,
    chain!(
        min:         parse_vec3 ~
        max:         parse_vec3 ~
        first_face:  le_i32     ~
        num_faces:   le_i32     ~
        first_brush: le_i32     ~
        num_brushes: le_i32     ,
        || {
            RawModel {
                min: min,
                max: max,
                first_face: first_face,
                num_faces: num_faces,
                first_brush: first_brush,
                num_brushes: num_brushes,
            }
        }
    )
}

named! {
    parse_raw_brush<RawBrush>,
    chain!(
        first_brush_side: le_i32 ~
        num_brush_sides:  le_i32 ~
        texture_index:    le_i32 ,
        || {
            RawBrush {
                first_brush_side: first_brush_side,
                num_brush_sides: num_brush_sides,
                texture_index: texture_index,
            }
        }
    )
}

named! {
    parse_raw_brush_side<RawBrushSide>,
    chain!(
        plane_index:   le_i32 ~
        texture_index: le_i32 ,
        || {
            RawBrushSide {
                plane_index: plane_index,
                texture_index: texture_index,
            }
        }
    )
}

named! {
    parse_raw_mesh_vertex<RawMeshVertex>,
    chain!(
        offset: le_i32 ,
        || {
            RawMeshVertex {
                offset: offset,
            }
        }
    )
}

named! {
    parse_raw_effect<RawEffect>,
    chain!(
        name:        take_s!(64) ~
        brush_index: le_i32      ~
                     take!(4)    ,
        || {
            RawEffect {
                name: name,
                brush_index: brush_index,
            }
        }
    )
}

fn parse_face_type(i: &[u8]) -> IResult<&[u8], FaceType> {
    use nom::ErrorKind;
    let (rest, t) = itry!(le_i32(i));
    match t {
        1 => Done(rest, FaceType::Polygon),
        2 => Done(rest, FaceType::Patch),
        3 => Done(rest, FaceType::Mesh),
        4 => Done(rest, FaceType::Billboard),
        _ => Error(Err::Code(ErrorKind::Custom(0))),
    }
}

named! {
    parse_raw_face<RawFace>,
    chain!(
        texture_index:     le_i32          ~
        effect_index:      le_i32          ~
        face_type:         parse_face_type ~
        first_vertex:      le_i32          ~
        num_vertexes:      le_i32          ~
        first_mesh_vertex: le_i32          ~
        num_mesh_vertices: le_i32          ~
        lightmap_index:    le_i32          ~
        lightmap_start:    parse_ivec2     ~
        lightmap_size:     parse_ivec2     ~
        lightmap_origin:   parse_vec3      ~
        lightmap_vecs:     pair!(
            parse_vec3,
            parse_vec3
        )                                  ~
        normal:            parse_vec3      ~
        size:              parse_ivec2     ,
        || {
            RawFace {
                texture_index: texture_index,
                effect_index: effect_index,
                face_type: face_type,
                first_vertex: first_vertex,
                num_vertexes: num_vertexes,
                first_mesh_vertex: first_mesh_vertex,
                num_mesh_vertices: num_mesh_vertices,
                lightmap_index: lightmap_index,
                lightmap_start: lightmap_start,
                lightmap_size: lightmap_size,
                lightmap_origin: lightmap_origin,
                lightmap_vecs: lightmap_vecs,
                normal: normal,
                size: size,
            }
        }
    )
}

fn parse_visibility_data(i: &[u8]) -> IResult<&[u8], RawVisibilityData> {
    let (rest, (num_vectors, sizeof_vector)) = itry!(
        chain! {i,
            num_vectors:   le_i32 ~
            sizeof_vector: le_i32 ,
            || { (num_vectors, sizeof_vector) }
        }
    );

    let size = num_vectors * sizeof_vector;
    let (rest, raw) = itry!(take!(rest, size));

    Done(rest, RawVisibilityData {
        num_vectors: num_vectors,
        sizeof_vector: sizeof_vector,
        raw_bytes: raw.into_iter().map(|&a| a).collect::<Vec<_>>(),
    })
}

fn parse_lightmap(i: &[u8]) -> IResult<&[u8], Lightmap> {
    let mut out = [[[0u8, 0u8, 0u8]; 128]; 128];
    let mut rest = i;
    for x in 0..128 {
        for y in 0..128 {
            let (a, b) = itry!(take_exact!(rest, 3));
            rest = a;
            out[y][x] = b;
        }
    }
    Done(rest, Lightmap { colors: out })
}

named! {
    parse_rotation_direction<RotationDirection>,
    chain!(
        phi:   le_u8 ~
        theta: le_u8 ,
        || {
            RotationDirection {
                phi: phi,
                theta: theta,
            }
        }
    )
}

named! {
    parse_light_volume<LightVolume>,
    chain!(
        ambient:     take_exact!(3)           ~
        directional: take_exact!(3)           ~
        direction:   parse_rotation_direction ,
        || {
            LightVolume {
                ambient: ambient,
                directional: directional,
                direction: direction,
            }
        }
    )
}

pub fn parse_raw_bsp(i: &[u8]) -> IResult<&[u8], RawBsp> {
    let (_, header) = itry!(directory_header(i));
    let (_, entities) = many1_from_header!(i,
        header.entities,
        parse_entity,
        Entity
    );
    let (_, textures) = get_from_header!(i,
        header.textures,
        parse_texture,
        Texture,
        72
    );
    let (_, planes) = get_from_header!(i,
        header.planes,
        parse_plane,
        Plane
    );
    let (_, nodes) = get_from_header!(i,
        header.nodes,
        parse_raw_node,
        RawNode
    );
    let (_, leaves) = get_from_header!(i,
        header.leaves,
        parse_raw_leaf,
        RawLeaf
    );
    let (_, leaf_faces) = get_from_header!(i,
        header.leaf_faces,
        parse_raw_leaf_face,
        RawLeafFace
    );
    let (_, leaf_brushes) = get_from_header!(i,
        header.leaf_brushes,
        parse_raw_leaf_brush,
        RawLeafBrush
    );
    let (_, models) = get_from_header!(i,
        header.models,
        parse_raw_model,
        RawModel
    );
    let (_, brushes) = get_from_header!(i,
        header.brushes,
        parse_raw_brush,
        RawBrush
    );
    let (_, brush_sides) = get_from_header!(i,
        header.brush_sides,
        parse_raw_brush_side,
        RawBrushSide
    );
    let (_, vertices) = get_from_header!(i,
        header.vertices,
        parse_vertex,
        Vertex
    );
    let (_, mesh_vertices) = get_from_header!(i,
        header.mesh_vertices,
        parse_raw_mesh_vertex,
        RawMeshVertex
    );
    let (_, effects) = get_from_header!(i,
        header.effects,
        parse_raw_effect,
        RawEffect,
        72
    );
    let (_, faces) = get_from_header!(i,
        header.faces,
        parse_raw_face,
        RawFace
    );
    let (_, light_maps) = get_from_header!(i,
        header.light_maps,
        parse_lightmap,
        Lightmap
    );
    let (_, light_volumes) = get_from_header!(i,
        header.light_volumes,
        parse_light_volume,
        LightVolume
    );
    let (_, visibility_data) = many1_from_header!(i,
        header.visibility_data,
        parse_visibility_data,
        RawVisibilityData
    );

    Done(&[], RawBsp {
        header: header,
        entities: entities,
        textures: textures,
        planes: planes,
        nodes: nodes,
        leaves: leaves,
        leaf_faces: leaf_faces,
        leaf_brushes: leaf_brushes,
        models: models,
        brushes: brushes,
        brush_sides: brush_sides,
        vertices: vertices,
        mesh_vertices: mesh_vertices,
        effects: effects,
        faces: faces,
        light_maps: light_maps,
        light_volumes: light_volumes,
        visibility_data: visibility_data,
    })
}
