use std::path::{Path,PathBuf};
use std::fs::PathExt;
use std::mem::replace;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use glium::texture::Texture2d;
use glium::backend::Facade;
use image;
use texture_flags::*;
use helpers::*;
use raw_bsp::*;

// TODO: IMPORTANT HOLY SHIT 𝘋𝘖 𝘕𝘖𝘛 𝘍𝘖𝘙𝘎𝘌𝘛!
//       Remove all uses of [] & use try!(_.get(_)) instead

pub struct Bsp {
    node_owner: Vec<NonTerminal>,
    leaf_owner: Vec<Leaf>,
    // For caching/etc. needs keep this separate and store indexes only
    vertices: Vec<Vertex>,
    root: BspTreeNode,
}

impl Bsp {
    pub fn get_visible_set_at(&self, point: Vec3) -> Vec<&Leaf> {
        self.get_terminal_at(point).map_or(
            vec![],
            |t| self.get_visible_set_of(t)
        )
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

    fn get_terminal_at(&self, point: Vec3) -> Option<&Leaf> {
        let mut current = &self.root;
        while let &BspTreeNode::NonTerminal(node_index) = current {
            let node = &self.node_owner[node_index];
            let dot = point.iter()
                .zip(node.plane.normal.iter())
                .map(|(a, b)| a * b)
                .sum::<f32>();

            if izip!(
                point.iter(),
                node.bounds.0.iter(),
                node.bounds.1.iter()
            ).any(
                |(&p, &min, &max)| p < min as f32 || p > max as f32
            ) {
                return None;
            }

            current =
                if dot < node.plane.distance {
                    &node.back
                } else {
                    &node.front
                }
        }

        if let &BspTreeNode::Leaf(leaf_index) = current {
            Some(&self.leaf_owner[leaf_index])
        } else {
            None
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
    Empty,
}

struct NonTerminal {
    plane: Plane,
    bounds: (IVec3, IVec3),
    front: BspTreeNode,
    back: BspTreeNode,
}

#[derive(Debug)]
pub struct Leaf {
    cluster: usize,
    visdata: Vec<usize>,
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
pub struct Texture {
    hash: u64,
    texture: Texture2d,
    surface_flags: SurfaceFlags,
}

// TODO: make this take a root directory
pub struct TextureBuilder<'a, T: Facade + 'a> {
    root: PathBuf,
    facade: &'a T,
    cache: Vec<Weak<Texture>>,
}

impl<'a, T: Facade + 'a> TextureBuilder<'a, T> {
    pub fn new<A: Into<PathBuf>>(a: A, facade: &'a T) -> TextureBuilder<'a, T> {
        TextureBuilder { root: a.into(), facade: facade, cache: vec![] }
    }

    fn get_real_path_and_ext(
        &self,
        path: &String
    ) -> Option<(image::ImageFormat, PathBuf)> {
        use image::ImageFormat;
        use image::ImageFormat::*;

        fn get_extensions(i: &ImageFormat) -> &'static [&'static str] {
            static PNG_EXT:  [&'static str; 1] = ["png"];
            static JPEG_EXT: [&'static str; 2] = ["jpeg", "jpg"];
            static GIF_EXT:  [&'static str; 1] = ["gif"];
            static WEBP_EXT: [&'static str; 1] = ["webp"];
            static PPM_EXT:  [&'static str; 1] = ["ppm"];
            static TIFF_EXT: [&'static str; 1] = ["tiff"];
            static TGA_EXT:  [&'static str; 1] = ["tga"];
            static BMP_EXT:  [&'static str; 1] = ["bmp"];
            static ICO_EXT:  [&'static str; 1] = ["ico"];

            match *i {
                PNG  => &PNG_EXT,
                JPEG => &JPEG_EXT,
                GIF  => &GIF_EXT,
                WEBP => &WEBP_EXT,
                PPM  => &PPM_EXT,
                TIFF => &TIFF_EXT,
                TGA  => &TGA_EXT,
                BMP  => &BMP_EXT,
                ICO  => &ICO_EXT,
            }
        }

        let root: PathBuf = self.root.join(path);
        let file_name: String =
            if let Some(Some(f)) = root.file_name().map(|o| o.to_str()) {
                f.into()
            } else {
                return None
            };
        for ex in [PNG, JPEG, GIF, WEBP, PPM, TIFF, TGA, BMP, ICO].into_iter() {
            let extensions = get_extensions(&ex);

            for str_ex in extensions {
                use std::env;

                let out = root.with_file_name(format!("{}.{}", file_name, str_ex));

                if out.is_file() { return Some((*ex, out.to_path_buf())); }
            }
        }

        None
    }

    pub fn load(
        &mut self, path: &String, surface_flags: SurfaceFlags
    ) -> Option<Rc<Texture>> {
        use std::io::BufReader;
        use std::fs::File;
        use glium::texture::RawImage2d;
        use image;

        let str_hash = get_string_hash(path);
        if let Some(t) = self.cache.iter()
            .filter_map(|weak| weak.upgrade())
            .find(
                |t| t.hash == str_hash
            )
        {
            return Some(t);
        }

        let (ext, real_path) =
            if let Some(tup) = self.get_real_path_and_ext(path) {
                tup
            } else {
                println!("{} not found", path);
                return None
            };

        let mut f = if let Ok(a) = File::open(&real_path) {
                a
            } else {
                println!("Cannot open {:?}", &real_path);
                return None
            };
        let mut reader = BufReader::new(f);

        let raw = if let Ok(a) = image::load(
                reader,
                ext
            ) {
                a.to_rgba()
            } else {
                println!("Cannot interpret {:?}", &real_path);
                return None
            };
        let image_dimensions = raw.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(
                raw.into_raw(), image_dimensions
            );
        Texture2d::new(self.facade, image).ok()
            .map(|t| {
                let out = Rc::new(
                    Texture {
                        hash: get_string_hash(path),
                        texture: t,
                        surface_flags: surface_flags
                    }
                );
                self.cache.push(Rc::downgrade(&out));
                out
            })
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool { self.hash == other.hash }
}

impl Eq for Texture {}

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
                FaceRenderType::Patch(
                    vec![] // TODO: make this work
                ),
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
    clusters.iter().filter_map(|&(cluster, ref group)| {
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
            .map(|i| &faces[i.index as usize])
            .map(|f| build_face(f, mesh_verts, textures))
        };
        let get_brushes = |leaf: &&RawLeaf| {
            leaf_brushes[{
                let start = leaf.first_leaf_brush as usize;
                let end = start + leaf.num_leaf_brushes as usize;
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
        };

        let faces = group.iter().flat_map(get_faces).collect::<Vec<_>>();
        let brushes = group.iter().flat_map(get_brushes).collect::<Vec<_>>();

        Some(Leaf {
            cluster: cluster as usize,
            visdata: get_indices(
                    &visibility_data.raw_bytes[{
                        let start = (cluster *
                                     visibility_data.sizeof_vector) as usize;
                        let end = start +
                            visibility_data.sizeof_vector as usize;
                        start..end
                    }]).into_iter()
                .flat_map(|i|
                          clusters.iter()
                          .enumerate()
                          .filter(|&(cluster, _)|
                              cluster == i
                          )
                          .map(|(index, _)| index)
                          .collect::<Vec<_>>()
                         )
                .collect::<Vec<_>>(),
            faces: faces,
            brushes: brushes,
        })
    })
    .collect::<Vec<_>>()
}

fn get_bsp_tree_node(
    i: i32, raw_leaves: &Vec<RawLeaf>, leaves: &Vec<Leaf>
) -> BspTreeNode {
    if i < 0 {
        let leaf_index = (-i) as usize;
        let actual_index = leaves.iter()
            .enumerate()
            .find(|&(_, l)|
                raw_leaves[leaf_index].visdata_cluster as usize == l.cluster
            )
            .map(|o| o.0);
        if let Some(index) = actual_index {
            BspTreeNode::Leaf(index)
        } else {
            BspTreeNode::Empty
        }
    } else {
        BspTreeNode::NonTerminal(i as usize)
    }
}

fn build_nodes(raw: &mut RawBsp, leaves: &Vec<Leaf>) -> Vec<NonTerminal> {
    let planes = &raw.planes;
    raw.nodes.iter()
        .map(|n|
            NonTerminal {
                plane: planes[n.plane_index as usize].clone(),
                bounds: (n.min.clone(), n.max.clone()),
                front: get_bsp_tree_node(
                    n.children_indices.0,
                    &raw.leaves,
                    leaves
                ),
                back: get_bsp_tree_node(
                    n.children_indices.1,
                    &raw.leaves,
                    leaves
                ),
            }
        )
        .collect::<Vec<_>>()
}

fn build_textures<T: Facade>(
    raw: &Vec<RawTexture>,
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
         // TODO: Replace this with 𝘱𝘳𝘰𝘱𝘦𝘳 missingno texture
         builder.load(
             &"textures/phdm5/metb_seam".into(),
             raw.surface_flags.clone()
             )
        )
    .ok_or(())
}

fn get_string_hash(s: &String) -> u64 {
    use std::hash::{SipHasher, Hash, Hasher};

    let mut hasher = SipHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
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
