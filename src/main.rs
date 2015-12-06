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
use std::mem::size_of;
use std::str::from_utf8;
use lazysort::SortedBy;

/*********************************
 * All numbers are little-endian *
 *********************************/

named! {
    parse_texture <Texture>,
    chain!(
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
    )
}

named! {
    parse_plane <Plane>,
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
    parse_raw_node <RawNode>,
    chain!(
        plane:   le_i32       ~
        child_a: le_i32       ~
        child_b: le_i32       ~
        min:     parse_ivec3  ~
        max:     parse_ivec3  ,
        || {
            RawNode {
                plane: plane,
                children: (child_a, child_b),
                min: min,
                max: max,
            }
        }
    )
}

named! {
    parse_vertex <Vertex>,
    chain!(
        position:        parse_vec3 ~
        surface_coords:  parse_vec2 ~
        lightmap_coords: parse_vec2 ~
        normal:          parse_vec3 ~
        c0:              le_u8      ~
        c1:              le_u8      ~
        c2:              le_u8      ~
        c3:              le_u8      ,
        || {
            Vertex {
                position: position,
                surface_coords: surface_coords,
                lightmap_coords: lightmap_coords,
                normal: normal,
                color: [c0, c1, c2, c3],
            }
        }
    )
}

named! {
    parse_leaf <RawLeaf>,
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
    parse_leaf_face <RawLeafFace>,
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
    parse_leaf_brush <RawLeafBrush>,
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
    parse_model <RawModel>,
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
    parse_brush <RawBrush>,
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
    parse_brush_side <RawBrushSide>,
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
    parse_mesh_vertex <RawMeshVert>,
    chain!(
        offset: le_i32 ,
        || {
            RawMeshVert {
                offset: offset,
            }
        }
    )
}

named! {
    parse_effect <RawEffect>,
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

fn raw_parse<T>() -> T {
    chain!(
        data: take!(mem::size_of<T>()) ,
        || {
            unsafe { mem::transmute(data) }
        }
    )
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
    let textures = get_from_header!(i,
        header.textures,
        parse_texture,
        Texture
    );
    let planes = get_from_header!(i,
        header.planes,
        parse_plane,
        Plane
    );
    let nodes = get_from_header!(i,
        header.nodes,
        parse_raw_node,
        RawNode
    );
    let leaves = get_from_header!(i,
        header.leaves,
        parse_leaf,
        RawLeaf
    );
    let leaf_faces = get_from_header!(i,
        header.leaf_faces,
        parse_leaf_face,
        RawLeafFace
    );
    let leaf_brushes = get_from_header!(i,
        header.leaf_brushes,
        parse_leaf_brush,
        RawLeafBrush
    );
    let models = get_from_header!(i,
        header.models,
        parse_model,
        RawModel
    )
    let brushes = get_from_header!(i,
        header.brushes,
        parse_brush,
        RawBrush
    )
    let brush_sides = get_from_header!(i,
        header.brush_sides,
        parse_brush_side,
        RawBrushSide
    )
    let (_, vertices) = get_from_header!(i,
        header.vertices,
        parse_vertex,
        Vertex
    );
    /*
    mesh_vertices: Vec<RawMeshVert>,
    effects: Vec<RawEffect>,
    faces: Vec<RawFace>,
    light_maps: Vec<Lightmap>,
    light_volumes: Vec<LightVolume>,
    visibility_data: Vec<RawVisibilityData>,
    */

    RawBsp {
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
    }
}

fn main() {
    let b = include_bytes!(
        "../assets/simple-dm5.bsp"
    );
}
