use std::mem::replace;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::boxed::FnBox;
use std::iter::IntoIterator;
use glium::backend::Facade;
use glium::texture::Texture2d;
use lazy::Lazy;
use itertools::*;
use texture_flags::*;
use texture::*;
use helpers::*;
use raw_bsp::RawBsp;

use nalgebra;

mod transform;

pub use self::transform::build_bsp as from_raw;
pub use raw_bsp::{Vertex, Plane};

const MIN_PATCH_SUBDIVISION_LEVELS: usize = 12;
const MAX_PATCH_SUBDIVISION_LEVELS: usize = 20;

// TODO: Make this return &Leaf instead of Rc<Leaf>
pub struct Bsp {
    // For caching/etc. needs keep this separate and store indexes only
    vertices: Vec<Vertex>,
    world: Model,
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

    pub fn get_world(&self) -> &[Face] {
        &self.world.faces
    }

    pub fn get_bounds(&self) -> (IVec3, IVec3) {
        self.root.bounds.clone()
    }

    fn new(
        root: NonTerminal,
        world: Model,
        verts: Vec<Vertex>
    ) -> Bsp {
        Bsp {
            vertices: verts,
            world: world,
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
            .filter_map(|i| {
                let rc = i.upgrade();
                debug_assert!(rc.is_some());
                rc
            })
            .collect::<Vec<_>>();
        out.push(leaf);
        out
    }
}

enum BspTreeNode {
    NonTerminal(Rc<NonTerminal>),
    Leaf(Rc<Leaf>),
    Empty,
}

struct NonTerminal {
    plane: Plane,
    bounds: (IVec3, IVec3),
    front: RefCell<BspTreeNode>,
    back: RefCell<BspTreeNode>,
}

// TODO: Make this thread-safe. RefCell is only referenced on initialisation
//       so it is safe to send, but the Weak is non-atomic.
pub struct Leaf {
    cluster: isize,
    visdata: RefCell<Vec<Weak<Leaf>>>,
    pub faces: Vec<Face>,
    pub brushes: Vec<Brush>,
}

pub struct Face {
    pub texture: Texture,
    pub lightmap: Option<Rc<Texture2d>>,
    pub render_type: FaceRenderType,
}

pub struct Model {
    pub min: Vec3,
    pub max: Vec3,
    pub faces: Vec<Face>,
    pub brushes: Vec<Brush>,
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

pub struct PatchData(
    [
        Lazy<(Vec<Vertex>, Vec<u16>)>;
        MAX_PATCH_SUBDIVISION_LEVELS - MIN_PATCH_SUBDIVISION_LEVELS
    ]
);

impl PatchData {
    pub fn tessellate(&self, level: usize) -> (&[Vertex], &[u16]) {
        let (ref vs, ref is) = *self.0[level];
        (vs, is)
    }
}

pub enum FaceRenderType {
    Patch(Vec<PatchData>),
    Mesh(Vec<usize>),
    Billboard(usize),
}
