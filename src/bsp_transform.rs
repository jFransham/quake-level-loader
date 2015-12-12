use std::path::Path;
use std::mem::{replace, size_of};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use glium::texture::Texture2d;
use texture_flags::*;
use helpers::*;
use raw_bsp::*;

// TODO: IMPORTANT HOLY SHIT 𝘋𝘖 𝘕𝘖𝘛 𝘍𝘖𝘙𝘎𝘌𝘛!
//       Remove all uses of [] & use try!(_.get(_)) instead

// TODO: implement so this acts like a Vec<Rc<T>>
//       Extract to crate?

pub struct Bsp {
    node_owner: Vec<NonTerminal>,
    leaf_owner: Vec<Leaf>,
    // For caching/etc. needs keep this separate and store indexes only
    vertices: Vec<Vertex>,
    root: BspTreeNode,
}

impl Bsp {
    pub fn get_visible_set_at(&self, point: Vec3) -> Vec<&Leaf> {
        self.get_visible_set_of(self.get_terminal_at(point))
    }

    fn new(
        node_owner: Vec<NonTerminal>,
        leaf_owner: Vec<Leaf>,
        verts: Vec<Vertex>
    ) -> Bsp {
        Bsp {
            node_owner: node_owner,
            leaf_owner: leaf_owner,
            vertices: verts,
            root: BspTreeNode::NonTerminal(0)
        }
    }

    fn get_collision_planes_between(
        &self, bounds: (Vec3, Vec3)
    ) -> Vec<&Plane> {
        unimplemented!()
    }

    fn get_terminal_at(&self, point: Vec3) -> &Leaf {
        let mut current = &self.root;
        while let &BspTreeNode::NonTerminal(node_index) = current {
            let node = &self.node_owner[node_index];
            let dot = point.iter()
                .zip(node.plane.normal.iter())
                .map(|(a, b)| a * b)
                .sum::<f32>();
            current =
                if dot < node.plane.distance {
                    &node.back
                } else {
                    &node.front
                }
        }

        if let &BspTreeNode::Leaf(leaf_index) = current {
            &self.leaf_owner[leaf_index]
        } else {
            unreachable!()
        }
    }

    fn get_visible_set_of<'a>(&'a self, leaf: &'a Leaf) -> Vec<&'a Leaf> {
        let mut out = leaf.visdata.iter()
            .map(|&i| &self.leaf_owner[i])
            .collect::<Vec<_>>();
        out.push(leaf);
        out
    }
}

enum BspTreeNode {
    NonTerminal(usize),
    Leaf(usize),
}

struct NonTerminal {
    plane: Plane,
    bounds: (IVec3, IVec3),
    front: BspTreeNode,
    back: BspTreeNode,
}

pub struct Leaf {
    visdata: Vec<usize>,
    faces: Vec<Face>,
    brushes: Vec<Brush>,
}

pub struct Face {
    texture: Rc<Texture>,
    render_type: FaceRenderType,
}

struct Surface {
    plane: Plane,
    surface_flags: SurfaceFlags,
}

pub struct Texture {
    hash: u64,
    texture: Texture2d,
    surface_flags: SurfaceFlags,
}

impl Texture {
    pub fn load<T: ?Sized + AsRef<Path>>(path: &T) -> Option<Texture> {
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool { self.hash == other.hash }
}

impl Eq for Texture {}

pub struct Brush {
    surfaces: Vec<Surface>,
    content_flags: ContentFlags,
}

enum FaceRenderType {
    Patch(Vec<Vec3>),
    Mesh(Vec<usize>),
    Billboard(usize),
}

struct BspBuilder {
    bsp: RawBsp,
}

fn get_indices(visdata: &[u8]) -> Vec<usize> {
    (0..visdata.len()*8).into_iter()
        .filter(|i|
            visdata[*i as usize / 8] & (1 << (*i % 8)) != 0
        )
        .collect::<Vec<_>>()
}

fn build_face(
    face: &RawFace,
    mesh_verts: &Vec<RawMeshVertex>,
    textures: &Vec<Rc<Texture>>
) -> Face {
    Face {
        texture: textures[face.texture_index as usize].clone(),
        render_type: match &face.face_type {
            &FaceType::Polygon   =>
                FaceRenderType::Mesh(
                    {
                        let start = face.first_vertex as usize;
                        let end = start + face.num_vertices as usize;
                        start..end
                    }.into_iter()
                    .collect::<Vec<_>>()
                ),
            &FaceType::Mesh      =>
                FaceRenderType::Mesh(
                    {
                        let start = face.first_mesh_vertex as usize;
                        let end = face.num_mesh_vertices as usize;
                        start..end
                    }.into_iter()
                    .map(|i| &mesh_verts[i])
                    .map(|v| v.offset + face.first_vertex)
                    .map(|i| i as usize)
                    .collect::<Vec<_>>()
                ),
            &FaceType::Patch     =>
                unimplemented!(),
            &FaceType::Billboard =>
                FaceRenderType::Billboard(
                    0 // TODO: support things proper-like
                ),
        },
    }
}

fn build_brush(
    brush: &RawBrush,
    brush_sides: &Vec<RawBrushSide>,
    planes: &Vec<Plane>,
    raw_textures: &Vec<RawTexture>,
) -> Brush {
    Brush {
        surfaces: brush_sides[{
                let start = brush.first_brush_side as usize;
                let end = start + brush.num_brush_sides as usize;
                start..end
            }].iter()
            .map(|s| (
                planes[s.plane_index as usize].clone(),
                raw_textures[s.texture_index as usize].surface_flags,
            ))
            .map(|(p, f)| Surface {
                plane: p,
                surface_flags: f,
            })
            .collect::<Vec<_>>(),
        content_flags: raw_textures[
            brush.texture_index as usize
        ].content_flags,
    }
}

fn build_leaves<'a>(
    raw: &mut RawBsp,
    textures: &Vec<Rc<Texture>>,
) -> Vec<Leaf> {
    let faces = &raw.faces;
    let leaf_brushes = &raw.leaf_brushes;
    let leaf_faces = &raw.leaf_faces;
    let brushes = &raw.brushes;
    let brush_sides = &raw.brush_sides;
    let planes = &raw.planes;
    let raw_textures = &raw.textures;
    let visibility_data = &raw.visibility_data;
    let mesh_verts = &raw.mesh_vertices;
    raw.leaves.drain(..)
        .map(|l| {
            let faces = leaf_faces[{
                    let start = l.first_leaf_face as usize;
                    let end = start + l.num_leaf_faces as usize;
                    start..end
                }].iter()
                .map(|i| &faces[i.index as usize])
                .map(|f| build_face(f, mesh_verts, textures))
                .collect::<Vec<_>>();
            let brushes = leaf_brushes[{
                    let start = l.first_leaf_brush as usize;
                    let end = start + l.num_leaf_brushes as usize;
                    start..end
                }].iter()
                .map(|i| &brushes[i.index as usize])
                .map(|b| build_brush(
                        b,
                        brush_sides,
                        planes,
                        raw_textures,
                    )
                )
                .collect::<Vec<_>>();

            Leaf {
                visdata: get_indices(
                    &visibility_data.raw_bytes[{
                        let start = (l.visdata_cluster *
                            visibility_data.sizeof_vector) as usize;
                        let end = start +
                            visibility_data.sizeof_vector as usize;
                        start..end
                    }]
                ),
                faces: faces,
                brushes: brushes,
            }
        })
        .collect::<Vec<_>>()
}

fn get_bsp_tree_node(i: i32) -> BspTreeNode {
    if i < 0 {
        BspTreeNode::Leaf((-i) as usize)
    } else {
        BspTreeNode::NonTerminal(i as usize)
    }
}

fn build_nodes(raw: &mut RawBsp) -> Vec<NonTerminal> {
    let planes = &raw.planes;
    raw.nodes.drain(..)
        .map(|n|
            NonTerminal {
                plane: planes[n.plane_index as usize].clone(),
                bounds: (n.min.clone(), n.max.clone()),
                front: get_bsp_tree_node(n.children_indices.0),
                back: get_bsp_tree_node(n.children_indices.1),
            }
        )
        .collect::<Vec<_>>()
}

fn build_textures(
    raw: &Vec<RawTexture>, cache: &mut Vec<Weak<Texture>>
) -> Result<Vec<Rc<Texture>>, ()> {
    let out = Vec::with_capacity(raw.len());
    for res in raw.iter().map(|r| get_texture(r, cache)) {
        try!(res.map(|t| out.push(t)));
    }
    Ok(out)
}

// TODO: Load textures
fn get_texture(
    raw: &RawTexture, cache: &mut Vec<Weak<Texture>>
) -> Result<Rc<Texture>, ()> {
    let str_hash = get_string_hash(&raw.path);
    if let Some(t) = cache.iter()
        .filter_map(|weak| weak.upgrade())
        .find(
            |t| t.hash == str_hash
    ) {
        Ok(t)
    } else {
        let out: Rc<Texture> = Rc::new(try!(Texture::load(&raw.path).ok_or(())));
        cache.push(Rc::downgrade(&out));
        Ok(out)
    }
}

fn get_string_hash(s: &String) -> u64 {
    use std::hash::{SipHasher, Hash, Hasher};

    let mut hasher = SipHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

pub fn build_bsp<'a>(
    mut raw: RawBsp, texture_cache: &mut Vec<Weak<Texture>>
) -> (Vec<Entity>, Bsp) {
    let tex = build_textures(
            &raw.textures,
            texture_cache
        ).expect("Invalid map");
    let ents = replace(&mut raw.entities, vec![]);
    let vertices = replace(&mut raw.vertices, vec![]);

    (
        ents,
        Bsp::new(
            build_nodes(&mut raw),
            build_leaves(&mut raw, &tex),
            vertices
        )
    )
}
