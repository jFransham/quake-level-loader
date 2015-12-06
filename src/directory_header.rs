use nom::{GetOutput,IResult,le_i32};

pub struct DirectoryEntry {
    pub offset: i32,
    pub size: i32,
}

pub struct DirectoryHeader {
    pub version: i32,
    pub entities: DirectoryEntry,
    pub textures: DirectoryEntry,
    pub planes: DirectoryEntry,
    pub nodes: DirectoryEntry,
    pub leaves: DirectoryEntry,
    pub leaf_faces: DirectoryEntry,
    pub leaf_brushes: DirectoryEntry,
    pub models: DirectoryEntry,
    pub brushes: DirectoryEntry,
    pub brush_sides: DirectoryEntry,
    pub vertices: DirectoryEntry,
    pub mesh_vertices: DirectoryEntry,
    pub effects: DirectoryEntry,
    pub faces: DirectoryEntry,
    pub light_maps: DirectoryEntry,
    pub light_volumes: DirectoryEntry,
    pub visibility_data: DirectoryEntry,
}

pub fn directory_entry(i: &[u8]) -> IResult<&[u8], DirectoryEntry> {
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

pub fn directory_header(i: &[u8]) -> IResult<&[u8], DirectoryHeader> {
    chain!(i,
                         tag!("IBSP")    ~
        version:         le_i32          ~
        entities:        directory_entry ~
        textures:        directory_entry ~
        planes:          directory_entry ~
        nodes:           directory_entry ~
        leaves:          directory_entry ~
        leaf_faces:      directory_entry ~
        leaf_brushes:    directory_entry ~
        models:          directory_entry ~
        brushes:         directory_entry ~
        brush_sides:     directory_entry ~
        vertices:        directory_entry ~
        mesh_vertices:      directory_entry ~
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
    )
}
