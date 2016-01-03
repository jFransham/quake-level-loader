use helpers::*;
use directory_header::*;
use std::collections::HashMap;
use texture_flags::*;

#[derive(Debug)]
pub struct Entity {
    pub parameters: HashMap<String, EntityValue>,
}

#[derive(Debug)]
pub enum EntityValue {
    Text(String),
    Vec3(Vec3),
    IVec3(IVec3),
    Vec2(Vec2),
    IVec2(IVec2),
    Float(f32),
    Int(i32),
    ModelRef(i32),
    TargetRef(i32),
}

pub struct RawBsp {
    pub header: DirectoryHeader,
    pub entities: Vec<Entity>,
    pub textures: Vec<RawTexture>,
    pub planes: Vec<Plane>,
    pub nodes: Vec<RawNode>,
    pub leaves: Vec<RawLeaf>,
    pub leaf_faces: Vec<RawLeafFace>,
    pub leaf_brushes: Vec<RawLeafBrush>,
    pub models: Vec<RawModel>,
    pub brushes: Vec<RawBrush>,
    pub brush_sides: Vec<RawBrushSide>,
    pub vertices: Vec<Vertex>,
    pub mesh_vertices: Vec<RawMeshVertex>,
    pub effects: Vec<RawEffect>,
    pub faces: Vec<RawFace>,
    pub light_maps: Vec<Lightmap>,
    pub light_volumes: Vec<LightVolume>,
    pub visibility_data: RawVisibilityData,
}

#[derive(Debug, Clone)]
pub struct RawTexture {
    pub path: String,
    pub surface_flags: SurfaceFlags,
    pub content_flags: ContentFlags,
}

#[derive(Debug, Clone)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: f32,
}

#[derive(Debug)]
pub struct RawNode {
    pub plane_index: i32,
    pub children_indices: (i32, i32),
    pub min: IVec3,
    pub max: IVec3,
}

#[derive(Debug)]
pub struct RawLeaf {
    pub visdata_cluster: i32,
    pub areaportal_area: i32,
    pub min: IVec3,
    pub max: IVec3,
    pub first_leaf_face: i32,
    pub num_leaf_faces: i32,
    pub first_leaf_brush: i32,
    pub num_leaf_brushes: i32,
}

#[derive(Debug)]
pub struct RawLeafFace {
    pub index: i32,
}

#[derive(Debug)]
pub struct RawLeafBrush {
    pub index: i32,
}

#[derive(Debug)]
pub struct RawModel {
    pub min: Vec3,
    pub max: Vec3,
    pub first_face: i32,
    pub num_faces: i32,
    pub first_brush: i32,
    pub num_brushes: i32,
}

#[derive(Debug)]
pub struct RawBrush {
    pub first_brush_side: i32,
    pub num_brush_sides: i32,
    pub texture_index: i32,
}

#[derive(Debug)]
pub struct RawBrushSide {
    pub plane_index: i32,
    pub texture_index:i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub surface_coords: Vec2,
    pub lightmap_coords: Vec2,
    pub normal: Vec3,
    pub color: [u8; 4],
}

implement_vertex! {
    Vertex,
    position,
    surface_coords,
    lightmap_coords,
    normal,
    color,
}

#[derive(Debug)]
pub struct RawMeshVertex {
    pub offset: i32,
}

#[derive(Debug)]
pub struct RawEffect {
    pub name: String,
    pub brush_index: i32,
    // padding: i32,
}

#[derive(Debug, Clone)]
// 32 bits (4 bytes)
pub enum FaceType {
    Polygon,   // = 1
    Patch,     // = 2
    Mesh,      // = 3
    Billboard, // = 4
}

#[derive(Debug, Clone)]
pub struct RawFace {
    pub texture_index: i32,
    pub effect_index: i32,
    pub face_type: FaceType,
    pub first_vertex: i32,
    pub num_vertices: i32,
    pub first_mesh_vertex: i32,
    pub num_mesh_vertices: i32,
    pub lightmap_index: i32,
    pub lightmap_start: IVec2,
    pub lightmap_size: IVec2,
    pub lightmap_origin: Vec3,
    pub lightmap_vecs: (Vec3, Vec3),
    pub normal: Vec3,
    pub size: IVec2,
}

pub struct Lightmap {
    pub colors: [[Rgb; 128]; 128]
}

#[derive(Debug)]
pub struct RotationDirection {
    pub phi: u8,
    pub theta: u8,
}

#[derive(Debug)]
pub struct LightVolume {
    pub ambient: Rgb,
    pub directional: Rgb,
    pub direction: RotationDirection,
}

#[derive(Debug, Default)]
pub struct RawVisibilityData {
    pub num_vectors: i32,
    pub sizeof_vector: i32,
    pub raw_bytes: Vec<u8>,
}
