#[macro_use]
extern crate nom;
extern crate byteorder;
extern crate lazysort;

use nom::{GetOutput,IResult,le_i32};
use nom::Err;
use nom::IResult::*;
use std::str::from_utf8;
use lazysort::SortedBy;

/*********************************
 * All numbers are little-endian *
 *********************************/

type Vec3 = [f32; 3];
type IVec3 = [i32; 3];
type IVec2 = [i32; 2];
type Rgb = [u8; 3];
type Rgba = [u8; 4];

#[derive(Debug)]
struct RawBsp {
    header: DirectoryHeader,
    entities: String,
    textures: Vec<RawTexture>,
    planes: Vec<Plane>,
    nodes: Vec<RawNode>,
    leafs: Vec<RawLeaf>,
    leaf_faces: Vec<RawLeafFace>,
    leaf_brushes: Vec<RawLeafBrush>,
    models: Vec<RawModel>,
    brushes: Vec<RawBrush>,
    brush_sides: Vec<RawBrushSide>,
    vertices: Vec<Vertex>,
    mesh_vertices: Vec<RawMeshVert>,
    effects: Vec<RawEffect>,
    faces: Vec<RawFace>,
    light_maps: Vec<Lightmap>,
    light_volumes: Vec<LightVolume>,
    visibility_data: Vec<RawVisibilityData>,
}

#[derive(Debug)]
struct RawTexture {
    name: [u8; 64],
    flags: i32,
    contents: i32,
}

struct Texture {
    name: String,
    flags: i32,
    contents: i32,
}

#[derive(Debug)]
struct Plane {
    normal: Vec3,
    distance: f32,
}

#[derive(Debug)]
struct RawNode {
    plane: i32,
    children: (i32, i32),
    min: IVec3,
    max: IVec3,
}

#[derive(Debug)]
struct RawLeaf {
    visdata_cluster: i32,
    areaportal_area: i32,
    min: IVec3,
    max: IVec3,
    first_leaf_face: i32,
    num_leaf_faces: i32,
    first_leaf_brush: i32,
    num_leaf_faces: i32,
}

#[derive(Debug)]
struct RawLeafFace {
    index: i32,
}

#[derive(Debug)]
struct RawLeafBrush {
    index: i32,
}

#[derive(Debug)]
struct RawModel {
    min: Vec3,
    max: Vec3,
    first_face: i32,
    num_faces: i32,
    first_brush: i32,
    num_brushes: i32,
}

#[derive(Debug)]
struct RawBrush {
    first_brush_side: i32,
    num_brush_sides: i32,
    texture_index: i32,
}

#[derive(Debug)]
struct RawBrushSide {
    plane_index: i32,
    texture_index:i32,
}

#[derive(Debug)]
struct Vertex {
    position: Vec3,
    surface_coords: [f32; 2],
    lightmap_coords: [f32; 2],
    normal: Vec3,
    color: [u8; 4],
}

#[derive(Debug)]
struct RawMeshVert {
    offset: i32,
}

#[derive(Debug)]
struct RawEffect {
    name: [u8; 64],
    brush_index: i32,
    // padding: i32,
}

// 32 bits (4 bytes)
#[derive(Debug)]
enum FaceType {
    Polygon,   // = 1
    Patch,     // = 2
    Mesh,      // = 3
    Billboard, // = 4
}

#[derive(Debug)]
struct RawFace {
    texture_index: i32,
    effect_index: i32,
    face_type: FaceType,
    first_vertex: i32,
    num_vertexes: i32,
    first_mesh_vertex: i32,
    num_mesh_vertices: i32,
    lightmap_index: i32,
    lightmap_start: IVec2,
    lightmap_size: IVec2,
    lightmap_origin: Vec3,
    lightmap_vecs: (Vec3, Vec3),
    normal: Vec3,
    size: IVec2,
}

#[derive(Debug)]
struct Lightmap {
    colors: [[Rgb; 128]; 128]
}

#[derive(Debug)]
struct RotationDirection {
    phi: u8,
    theta: u8,
}

#[derive(Debug)]
struct LightVolume {
    ambient: Rgb,
    directional: Rgb,
    direction: RotationDirection,
}

#[derive(Debug)]
struct RawVisibilityData {
    num_vectors: i32,
    sizeof_vector: i32,
    raw_bytes: Vec<u8>,
}

#[derive(Debug)]
struct DirectoryEntry {
    offset: i32,
    size: i32,
}

#[derive(Debug)]
struct DirectoryHeader {
    version: i32,
    entities: DirectoryEntry,
    textures: DirectoryEntry,
    planes: DirectoryEntry,
    nodes: DirectoryEntry,
    leafs: DirectoryEntry,
    leaf_faces: DirectoryEntry,
    leaf_brushes: DirectoryEntry,
    models: DirectoryEntry,
    brushes: DirectoryEntry,
    brush_sides: DirectoryEntry,
    vertexes: DirectoryEntry,
    mesh_verts: DirectoryEntry,
    effects: DirectoryEntry,
    faces: DirectoryEntry,
    light_maps: DirectoryEntry,
    light_volumes: DirectoryEntry,
    visibility_data: DirectoryEntry,
}

fn directory_entry(i: &[u8]) -> IResult<&[u8], DirectoryEntry> {
    chain!(i,
        offset: le_i32 ~
        size:   le_i32 ,
        || {
            DirectoryEntry {
                offset: offset,
                size:   size,
            }
        }
    )
}

fn directory_header(i: &[u8]) -> IResult<&[u8], DirectoryHeader> {
    chain!(i,
                         tag!("IBSP")    ~
        version:         le_i32          ~
        entities:        directory_entry ~
        textures:        directory_entry ~
        planes:          directory_entry ~
        nodes:           directory_entry ~
        leafs:           directory_entry ~
        leaf_faces:      directory_entry ~
        leaf_brushes:    directory_entry ~
        models:          directory_entry ~
        brushes:         directory_entry ~
        brush_sides:     directory_entry ~
        vertexes:        directory_entry ~
        mesh_verts:      directory_entry ~
        effects:         directory_entry ~
        faces:           directory_entry ~
        light_maps:      directory_entry ~
        light_volumes:   directory_entry ~
        visibility_data: directory_entry ,
        || {
            DirectoryHeader {
                version:   version,
                entities:  entities,
                textures: textures,
                planes: planes,
                nodes: nodes,
                leafs: leafs,
                leaf_faces: leaf_faces,
                leaf_brushes: leaf_brushes,
                models: models,
                brushes: brushes,
                brush_sides: brush_sides,
                vertexes: vertexes,
                mesh_verts: mesh_verts,
                effects: effects,
                faces: faces,
                light_maps: light_maps,
                light_volumes: light_volumes,
                visibility_data: visibility_data,
            }
        }
    )
}

fn parse_vec<T>(
    input: &[u8],
    fun: fn(&[u8]) -> IResult<&[u8], T>,
    count: i32
) -> IResult<&[u8], Vec<T>> {
    let output = Vec::with_capacity(count);
    let mut bytes: &[u8] = input;
    for i in 0..count {
        match fun(bytes) {
            Done(rest, result) => {
                bytes = rest;
                output.push(result);
            },
            Error(e) =>           { return Error(e) },
            Incomplete(needed) => { return Incomplete(needed) },
        }
    }

    Done(bytes, output)
}

fn bsp_parse(i: &[u8]) -> IResult<&[u8], Bsp> {
    match directory_header(i) {
        Done(rest, header) => {

        },
        Error(e) =>           Error(e),
        Incomplete(needed) => Incomplete(needed),
    }
}

fn main() {
    let b = include_bytes!(
        "../assets/simple-dm5.bsp"
    );

    match directory_header(b) {
        Done(_, out) => println!("{:?}", out),
        _            => { },
    }
}
