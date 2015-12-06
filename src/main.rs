#![feature(box_syntax)]

#[macro_use]
extern crate nom;
extern crate byteorder;
extern crate lazysort;

#[macro_use]
mod macros;
mod raw_bsp;
mod directory_header;
mod helpers;

use nom::{GetOutput,IResult,le_i32,le_u8};
use nom::Err;
use nom::IResult::*;
use directory_header::*;
use helpers::*;
use raw_bsp::*;
use std::mem;
use std::mem::size_of;
use std::str::from_utf8;
use lazysort::SortedBy;

/*********************************
 * All numbers are little-endian *
 *********************************/

fn parse_texture(i: &[u8]) -> IResult<&[u8], Texture> {
    chain! {i,
        name:     take_s!(64) ~
        flags:    le_i32      ~
        contents: le_i32      ,
        || {
            Texture {
                name: name,
                flags: flags,
                contents: contents,
            }
        }
    }
}

fn parse_effect(i: &[u8]) -> IResult<&[u8], RawEffect> {
    chain! {i,
        name:        take_s!(64) ~
        brush_index: le_i32      ~
                     take!(4)    ,
        || {
            RawEffect {
                name: name,
                brush_index: brush_index,
            }
        }
    }
}

fn parse_face_type(i: &[u8]) -> IResult<&[u8], FaceType> {
    let (rest, t) = itry!(le_i32(i));
    match t {
        1 => Done(rest, FaceType::Polygon),
        2 => Done(rest, FaceType::Patch),
        3 => Done(rest, FaceType::Mesh),
        4 => Done(rest, FaceType::Billboard),
        _ => Error(Err::Code(nom::ErrorKind::Custom(0))),
    }
}

fn parse_face(i: &[u8]) -> IResult<&[u8], RawFace> {
    chain! {i,
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
    }
}

fn parse_visibility_data(i: &[u8]) -> IResult<&[u8], RawVisibilityData> {
    let (rest, (num_vectors, sizeof_vector)) = itry!(
        chain! {i,
            num_vectors:   le_i32 ~
            sizeof_vector: le_i32 ,
            || { (num_vectors, sizeof_vector) }
        }
    );

    let (rest, raw) = itry!(take!(rest, num_vectors * sizeof_vector));

    Done(rest, RawVisibilityData {
        num_vectors: num_vectors,
        sizeof_vector: sizeof_vector,
        raw_bytes: raw.into_iter().map(|&a| a).collect::<Vec<_>>(),
    })
}

fn raw_parse<T>() -> Box<Fn(&[u8]) -> IResult<&[u8], T>> {
    box |i: &[u8]| {
        let size = mem::size_of::<T>();
        let (rest, data) = itry!(take!(i, size));
        Done(
            rest,
            unsafe { mem::transmute::<_, T>(data) }
        )
    }
}

fn parse_bsp(i: &[u8]) -> IResult<&[u8], RawBsp> {
    let (_, header) = itry!(directory_header(i));
    let (_, entities) = {
        let start = header.entities.offset as usize;
        let end = start + header.entities.size as usize;
        itry!(
            take_s!(&i[start..end], header.entities.size)
        )
    };
    let (_, textures) = get_from_header!(i,
        header.textures,
        box parse_texture,
        Texture
    );
    let (_, planes) = get_from_header!(i,
        header.planes,
        raw_parse::<Plane>(),
        Plane
    );
    let (_, nodes) = get_from_header!(i,
        header.nodes,
        raw_parse::<RawNode>(),
        RawNode
    );
    let (_, leaves) = get_from_header!(i,
        header.leaves,
        raw_parse::<RawLeaf>(),
        RawLeaf
    );
    let (_, leaf_faces) = get_from_header!(i,
        header.leaf_faces,
        raw_parse::<RawLeafFace>(),
        RawLeafFace
    );
    let (_, leaf_brushes) = get_from_header!(i,
        header.leaf_brushes,
        raw_parse::<RawLeafBrush>(),
        RawLeafBrush
    );
    let (_, models) = get_from_header!(i,
        header.models,
        raw_parse::<RawModel>(),
        RawModel
    );
    let (_, brushes) = get_from_header!(i,
        header.brushes,
        raw_parse::<RawBrush>(),
        RawBrush
    );
    let (_, brush_sides) = get_from_header!(i,
        header.brush_sides,
        raw_parse::<RawBrushSide>(),
        RawBrushSide
    );
    let (_, vertices) = get_from_header!(i,
        header.vertices,
        raw_parse::<Vertex>(),
        Vertex
    );
    let (_, mesh_vertices) = get_from_header!(i,
        header.mesh_vertices,
        raw_parse::<RawMeshVert>(),
        RawMeshVert
    );
    let (_, effects) = get_from_header!(i,
        header.effects,
        box parse_effect,
        RawEffect
    );
    let (_, faces) = get_from_header!(i,
        header.faces,
        box parse_face,
        RawFace
    );
    let (_, light_maps) = get_from_header!(i,
        header.light_maps,
        raw_parse::<Lightmap>(),
        Lightmap
    );
    let (_, light_volumes) = get_from_header!(i,
        header.light_volumes,
        raw_parse::<LightVolume>(),
        LightVolume
    );
    let (_, visibility_data) = get_from_header!(i,
        header.visibility_data,
        box parse_visibility_data,
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

fn main() {
    let b = include_bytes!(
        "../assets/simple-dm5.bsp"
    );
}
