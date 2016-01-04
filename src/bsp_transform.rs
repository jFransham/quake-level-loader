use std::mem::replace;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use glium::backend::Facade;
use texture_flags::*;
use texture::*;
use helpers::*;
use raw_bsp::*;

// TODO: IMPORTANT HOLY SHIT 𝘋𝘖 𝘕𝘖𝘛 𝘍𝘖𝘙𝘎𝘌𝘛!
//       Remove all uses of [] & use try!(_.get(_)) instead

// TODO: Make this return &Leaf instead of Rc<Leaf>
pub struct Bsp {
    // For caching/etc. needs keep this separate and store indexes only
    vertices: Vec<Vertex>,
    root: Rc<NonTerminal>,
}

impl Bsp {
    pub fn get_visible_set_at(&self, point: Vec3) -> Vec<Rc<Leaf>> {
        self.get_terminal_at(point).map_or(
            vec![],
            |t| self.get_visible_set_of(t)
        )
    }

    pub fn get_vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    fn new(
        root: NonTerminal,
        verts: Vec<Vertex>
    ) -> Bsp {
        Bsp {
            vertices: verts,
            root: Rc::new(root),
        }
    }

    fn get_surfaces_between(
        &self, bounds: (Vec3, Vec3)
    ) -> Vec<&Surface> {
        unimplemented!()
    }

    fn get_terminal_at(&self, point: Vec3) -> Option<Rc<Leaf>> {
        let mut current = self.root.clone();
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
                // TODO: make this convert p to an int instead of bounds to
                //       f32? Would mean that precision is not lost as min,
                //       max => 2^23
                |(&p, &min, &max)| p < min as f32 || p > max as f32
            ) {
                return None;
            }

            let tmp = {
                let child =
                    if dot < current.plane.distance {
                        current.back.borrow()
                    } else {
                        current.front.borrow()
                    };
                match *child {
                    BspTreeNode::NonTerminal(ref node_pntr) =>
                        node_pntr.clone(),
                    BspTreeNode::Leaf(ref leaf_pntr) =>
                        return Some(leaf_pntr.clone()),
                    BspTreeNode::Empty => return None,
                }
            };

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

#[derive(Debug)]
enum BspTreeNode {
    NonTerminal(Rc<NonTerminal>),
    Leaf(Rc<Leaf>),
    Empty,
}

#[derive(Debug)]
struct NonTerminal {
    plane: Plane,
    bounds: (IVec3, IVec3),
    front: RefCell<BspTreeNode>,
    back: RefCell<BspTreeNode>,
}

// TODO: Make this thread-safe. RefCell is only referenced on initialisation
//       so it is safe to send, but the Weak is non-atomic.
#[derive(Debug)]
pub struct Leaf {
    cluster: isize,
    visdata: RefCell<Vec<Weak<Leaf>>>,
    pub faces: Vec<Face>,
    pub brushes: Vec<Brush>,
}

#[derive(Debug)]
pub struct Face {
    pub texture: Texture,
    pub render_type: FaceRenderType,
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
pub enum FaceRenderType {
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
    textures: &[Texture]
) -> Face {
    Face {
        texture: textures[face.texture_index as usize].clone(),
        render_type: match face.face_type {
            FaceType::Mesh | FaceType::Polygon =>
                FaceRenderType::Mesh(
                    {
                        let start = face.first_mesh_vertex as usize;
                        let end = start + face.num_mesh_vertices as usize;
                        start..end
                    }.map(|i| &mesh_verts[i])
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
    textures: &[Texture],
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
        .group_by(|l| l.visdata_cluster);

    let mut out = clusters.map(|(cluster, ref group)| {
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

        let (faces, brushes) = group.iter()
            .flat_map(get_faces)
            .map(|lf| lf.index)
            .sorted_by(|a, b| a.cmp(b))
            .into_iter()
            .unique()
            .map(|i| &faces[i as usize])
            .map(|f| build_face(f, mesh_verts, textures))
            .zip(
                group.iter()
                    .flat_map(get_brushes)
                    .map(|lb| lb.index)
                    .sorted_by(|a, b| a.cmp(b))
                    .into_iter()
                    .unique()
                    .map(|i| &brushes[i as usize])
                    .map(|b| build_brush(
                            b,
                            brush_sides,
                            planes,
                            raw_textures,
                        )
                    )
            )
            .unzip();

        Rc::new(
            Leaf {
                cluster: cluster as isize,
                visdata: RefCell::new(vec![]),
                faces: faces,
                brushes: brushes,
            }
        )
    })
    .collect::<Vec<_>>();

    let out_of_map = if out[0].cluster < 0 {
        println!("Some out-of-map leaves");
        Some(out.remove(0))
    } else {
        println!("No out-of-map leaves");
        None
    };

    for l in &out {
        *l.visdata.borrow_mut() = get_indices(
            &visibility_data.raw_bytes[{
                let start = (l.cluster as usize) * visibility_data.sizeof_vector as usize;
                let end = start + visibility_data.sizeof_vector as usize;
                start..end
            }]
        ).into_iter()
            .map(|i| {
                debug_assert_eq!(i as isize, out[i].cluster);
                Rc::downgrade(&out[i])
            })
            .chain(out_of_map.clone().map(|l| Rc::downgrade(&l)).into_iter())
            .collect::<Vec<_>>()
    }

    if let Some(o) = out_of_map {
        out.push(o);
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
        let leaf_index = (-i - 1) as usize;
        let visdata_cluster = raw_leaves[leaf_index].visdata_cluster;
        let leaf_pntr = if visdata_cluster >= 0 {
            leaves.get(visdata_cluster as usize)
        } else {
            None
        };

        if let Some(pntr) = leaf_pntr {
            debug_assert_eq!(pntr.cluster as i32, visdata_cluster);
            BspTreeNode::Leaf(pntr.clone())
        } else {
            BspTreeNode::Empty
        }
    } else {
        BspTreeNode::NonTerminal(nodes[i as usize].clone())
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
) -> Result<Vec<Texture>, ()> {
    let texdata = raw.iter()
        .map(|r| r.path.clone())
        .collect::<Vec<_>>();
    let flags = raw.iter()
        .map(|r| r.surface_flags);
    let mut out = vec![];
    for (opt, f) in
        builder.load_async(texdata).into_iter()
            .zip(flags)
    {
        if let Some(tex) = opt {
            out.push(tex.create_texture(f));
        } else {
            return Err(());
        }
    }
    Ok(out)
}

pub fn build_bsp<'a, T: Facade>(
    mut raw: RawBsp,
    texture_builder: &mut TextureBuilder<T>
) -> (Vec<Entity>, Bsp) {
    let vertices = replace(&mut raw.vertices, vec![]);
    let ents = replace(&mut raw.entities, vec![]);
    let root = {
        println!("Start loading textures");
        let tex = build_textures(
                &raw.textures,
                texture_builder
            ).expect("Missing texture is missing");
        println!("End loading textures");
        println!("Start building leaves");
        let leaves = build_leaves(&mut raw, &tex);
        println!("End building leaves");
        println!("Start building nodes");
        let nodes = build_nodes(&mut raw, &leaves);
        println!("End building nodes");
        nodes[0].clone()
    };

    (
        ents,
        Bsp::new(
            Rc::try_unwrap(root).expect("Bsp has circular nodegraph"),
            vertices
        )
    )
}

#[cfg(test)]
mod test {
    extern crate test;

    use self::test::Bencher;
    use super::{build_textures, build_textures_sync};
    use texture::*;
    use texture_flags::*;
    use raw_bsp::*;
    use glium::{self, DisplayBuild};

    #[bench]
    pub fn bench_texture_loader(b: &mut Bencher) {
        let raw = vec![
            RawTexture {
                path: "textures/phdm5/metb".into(),
                surface_flags: SurfaceFlags::empty(),
                content_flags: ContentFlags::empty(),
            },
            RawTexture {
                path: "textures/phdm5/metb".into(),
                surface_flags: SurfaceFlags::empty(),
                content_flags: ContentFlags::empty(),
            },
            RawTexture {
                path: "textures/phdm5/brick1a".into(),
                surface_flags: SurfaceFlags::empty(),
                content_flags: ContentFlags::empty(),
            },
            RawTexture {
                path: "not/a/real/texture".into(),
                surface_flags: SurfaceFlags::empty(),
                content_flags: ContentFlags::empty(),
            },
        ];
        let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
        let ref mut builder =
            TextureBuilder::new(
                vec!["assets/trespass"],
                &display
            );
        b.iter(|| {
            build_textures(
                &raw,
                builder
            ).or_else(|_|
                build_textures_sync(
                    &raw,
                    builder
                )
            ).expect("Missing texture is missing");
        });
    }
}
