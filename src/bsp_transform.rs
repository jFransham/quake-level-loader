use std::mem::replace;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::ops::Deref;
use glium::backend::Facade;
use texture_flags::*;
use texture::*;
use helpers::*;
use raw_bsp::*;

// TODO: IMPORTANT HOLY SHIT 𝘋𝘖 𝘕𝘖𝘛 𝘍𝘖𝘙𝘎𝘌𝘛!
//       Remove all uses of [] & use try!(_.get(_)) instead

pub struct Bsp {
    node_owner: Vec<Rc<NonTerminal>>,
    leaf_owner: Vec<Rc<Leaf>>,
    // For caching/etc. needs keep this separate and store indexes only
    pub vertices: Vec<Vertex>,
    root: BspTreeNode,
}

impl Bsp {
    pub fn get_visible_set_at(&self, point: Vec3) -> Vec<Rc<Leaf>> {
        self.get_terminal_at(point).map_or(
            vec![],
            |t| self.get_visible_set_of(t)
        )
    }

    fn new(
        node_owner: Vec<Rc<NonTerminal>>,
        leaf_owner: Vec<Rc<Leaf>>,
        verts: Vec<Vertex>
    ) -> Bsp {
        let root = BspTreeNode::NonTerminal(Rc::downgrade(&node_owner[0]));

        Bsp {
            node_owner: node_owner,
            leaf_owner: leaf_owner,
            vertices: verts,
            root: root,
        }
    }

    fn get_collision_planes_between(
        &self, bounds: (Vec3, Vec3)
    ) -> Vec<&Plane> {
        unimplemented!()
    }

    fn get_terminal_at(&self, point: Vec3) -> Option<Rc<Leaf>> {
        let mut current = match self.root {
            BspTreeNode::NonTerminal(ref node_pntr) =>
                if let Some(n) = node_pntr.upgrade() {
                    n
                } else {
                    return None
                },
            BspTreeNode::Leaf(ref leaf_pntr) =>
                return leaf_pntr.upgrade(),
            BspTreeNode::Empty => return None,
        };

        loop {
            let dot = point.iter()
                .zip(current.plane.normal.iter())
                .map(|(a, b)| a * b)
                .sum::<f32>();

            if izip!(
                point.iter(),
                current.bounds.0.iter(),
                current.bounds.1.iter()
            ).any(
                |(&p, &min, &max)| p < min as f32 || p > max as f32
            ) {
                return None;
            }

            let tmp;
            {
                let child =
                    if dot < current.plane.distance {
                        current.back.borrow()
                    } else {
                        current.front.borrow()
                    };
                tmp = match *child {
                    BspTreeNode::NonTerminal(ref node_pntr) =>
                        if let Some(n) = node_pntr.upgrade() {
                                n
                            } else {
                                return None
                            },
                    BspTreeNode::Leaf(ref leaf_pntr) =>
                        return leaf_pntr.upgrade(),
                    BspTreeNode::Empty => return None,
                };
            }

            current = tmp;
        }
    }

    fn get_visible_set_of(&self, leaf: Rc<Leaf>) -> Vec<Rc<Leaf>> {
        let mut out = leaf.visdata.borrow().iter()
            .filter_map(|i| i.upgrade())
            .collect::<Vec<_>>();
        out.push(leaf);
        out
    }
}

enum BspTreeNode {
    NonTerminal(Weak<NonTerminal>),
    Leaf(Weak<Leaf>),
    Empty,
}

struct NonTerminal {
    plane: Plane,
    bounds: (IVec3, IVec3),
    front: RefCell<BspTreeNode>,
    back: RefCell<BspTreeNode>,
}

#[derive(Debug)]
pub struct Leaf {
    cluster: usize,
    visdata: RefCell<Vec<Weak<Leaf>>>,
    pub faces: Vec<Face>,
    pub brushes: Vec<Brush>,
}

#[derive(Debug)]
pub struct Face {
    texture: Rc<Texture>,
    render_type: FaceRenderType,
}

#[derive(Debug)]
struct Surface {
    plane: Plane,
    surface_flags: SurfaceFlags,
}
#[derive(Debug)]
pub struct Brush {
    surfaces: Vec<Surface>,
    content_flags: ContentFlags,
}

#[derive(Debug)]
enum FaceRenderType {
    Patch(Vec<Vec3>),
    Mesh(Vec<usize>),
    Billboard(usize),
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
    mesh_verts: &[RawMeshVertex],
    textures: &[Rc<Texture>]
) -> Face {
    Face {
        texture: textures[face.texture_index as usize].clone(),
        render_type: match face.face_type {
            FaceType::Polygon   =>
                FaceRenderType::Mesh(
                    {
                        let start = face.first_vertex as usize;
                        let end = start + face.num_vertices as usize;
                        start..end
                    }.into_iter()
                    .collect::<Vec<_>>()
                ),
            FaceType::Mesh      =>
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
            FaceType::Patch     =>
                FaceRenderType::Patch(
                    vec![] // TODO: make this work
                ),
            FaceType::Billboard =>
                FaceRenderType::Billboard(
                    0 // TODO: support things proper-like
                ),
        },
    }
}

fn build_brush(
    brush: &RawBrush,
    brush_sides: &[RawBrushSide],
    planes: &[Plane],
    raw_textures: &[RawTexture],
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
    textures: &[Rc<Texture>],
) -> Vec<Rc<Leaf>> {
    use itertools::*;

    let faces = &raw.faces;
    let leaf_brushes = &raw.leaf_brushes;
    let leaf_faces = &raw.leaf_faces;
    let brushes = &raw.brushes;
    let brush_sides = &raw.brush_sides;
    let planes = &raw.planes;
    let raw_textures = &raw.textures;
    let visibility_data = &raw.visibility_data;
    let mesh_verts = &raw.mesh_vertices;
    let clusters = raw.leaves.iter()
                    .sorted_by(|a, b|
                        a.visdata_cluster.cmp(&b.visdata_cluster)
                    )
                    .into_iter()
                    .group_by(|l| l.visdata_cluster)
                    .collect::<Vec<_>>();
    let out = clusters.iter().filter_map(|&(cluster, ref group)| {
        if cluster < 0 {
            return None
        }

        // make a closure to not have to deal with iterator adaptor types
        let get_faces = |leaf: &&RawLeaf| {
            leaf_faces[{
                let start = leaf.first_leaf_face as usize;
                let end = start + leaf.num_leaf_faces as usize;
                start..end
            }].iter()
        };
        let get_brushes = |leaf: &&RawLeaf| {
            leaf_brushes[{
                let start = leaf.first_leaf_brush as usize;
                let end = start + leaf.num_leaf_brushes as usize;
                start..end
            }].iter()
        };

        let faces = group.iter()
            .flat_map(get_faces)
            .map(|lf| lf.index)
            .unique()
            .map(|i| &faces[i as usize])
            .map(|f| build_face(f, mesh_verts, textures))
            .collect::<Vec<_>>();
        let brushes = group.iter()
            .flat_map(get_brushes)
            .map(|lb| lb.index)
            .unique()
            .map(|i| &brushes[i as usize])
            .map(|b| build_brush(
                    b,
                    brush_sides,
                    planes,
                    raw_textures,
                )
            )
        .collect::<Vec<_>>();

        Some(Rc::new(Leaf {
            cluster: cluster as usize,
            visdata: RefCell::new(vec![]),
            faces: faces,
            brushes: brushes,
        }))
    })
    .collect::<Vec<_>>();

    for l in &out {
        *l.visdata.borrow_mut() = get_indices(
            &visibility_data.raw_bytes[{
                let start = l.cluster *
                    visibility_data.sizeof_vector as usize;
                let end = start +
                    visibility_data.sizeof_vector as usize;
                start..end
            }]
        ).into_iter()
            .flat_map(|i|
                out.iter()
                .filter(|l|
                    l.cluster == i
                )
                .map(|leaf|
                    Rc::downgrade(&leaf)
                )
                .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>()
    }

    out
}

fn get_bsp_tree_node(
    i: i32,
    raw_leaves: &[RawLeaf],
    leaves: &[Rc<Leaf>],
    nodes: &[Rc<NonTerminal>]
) -> BspTreeNode {
    if i < 0 {
        let leaf_index = (-i) as usize;
        let leaf_pntr = leaves.iter()
            .find(|l|
                l.cluster == raw_leaves[leaf_index].visdata_cluster as usize
            )
            .map(|o| Rc::downgrade(o));
        if let Some(pntr) = leaf_pntr {
            BspTreeNode::Leaf(pntr)
        } else {
            BspTreeNode::Empty
        }
    } else {
        BspTreeNode::NonTerminal(Rc::downgrade(&nodes[i as usize]))
    }
}

fn build_nodes(raw: &mut RawBsp, leaves: &[Rc<Leaf>]) -> Vec<Rc<NonTerminal>> {
    let planes = &raw.planes;
    let out = raw.nodes.iter()
        .map(|n|
            Rc::new(NonTerminal {
                plane: planes[n.plane_index as usize].clone(),
                bounds: (n.min.clone(), n.max.clone()),
                front: RefCell::new(BspTreeNode::Empty),
                back: RefCell::new(BspTreeNode::Empty),
            })
        )
        .collect::<Vec<_>>();

    for (n, r) in out.iter().zip(raw.nodes.iter()) {
        *n.front.borrow_mut() = get_bsp_tree_node(
            r.children_indices.0,
            &raw.leaves,
            leaves,
            &out
        );
        *n.back.borrow_mut() = get_bsp_tree_node(
            r.children_indices.1,
            &raw.leaves,
            leaves,
            &out
        );
    }

    out
}

fn build_textures<T: Facade>(
    raw: &[RawTexture],
    builder: &mut TextureBuilder<T>
) -> Result<Vec<Rc<Texture>>, ()> {
    let mut out = Vec::with_capacity(raw.len());
    for res in raw.iter().map(|r| get_texture(r, builder)) {
        try!(res.map(|t| out.push(t)));
    }
    Ok(out)
}

fn get_texture<T: Facade>(
    raw: &RawTexture,
    builder: &mut TextureBuilder<T>
) -> Result<Rc<Texture>, ()> {
    builder.load(
        &raw.path,
        raw.surface_flags.clone()
        )
    .or_else(||
         builder.load(
             &"textures/common/missing",
             raw.surface_flags.clone()
             )
        )
    .ok_or(())
}

pub fn build_bsp<'a, T: Facade>(
    mut raw: RawBsp,
    texture_builder: &mut TextureBuilder<T>
) -> (Vec<Entity>, Bsp) {
    let tex = build_textures(
            &raw.textures,
            texture_builder
        ).expect("Invalid map");
    let ents = replace(&mut raw.entities, vec![]);
    let vertices = replace(&mut raw.vertices, vec![]);
    let leaves = build_leaves(&mut raw, &tex);
    let nodes = build_nodes(&mut raw, &leaves);

    (
        ents,
        Bsp::new(
            nodes,
            leaves,
            vertices
        )
    )
}
