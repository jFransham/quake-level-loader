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
use raw_bsp::*;
use super::{
    Bsp,
    BspTreeNode,
    NonTerminal,
    Leaf,
    Face,
    Model,
    Surface,
    Brush,
    PatchData,
    FaceRenderType,
    MIN_PATCH_SUBDIVISION_LEVELS,
    MAX_PATCH_SUBDIVISION_LEVELS,
};
use nalgebra;

// TODO: IMPORTANT HOLY SHIT 𝘋𝘖 𝘕𝘖𝘛 𝘍𝘖𝘙𝘎𝘌𝘛!
//       Remove all uses of [] & use try!(_.get(_)) instead

fn get_tessellate_fn(
    control_points: Rc<[Vertex; 9]>,
    level: usize
) -> Box<FnBox() -> (Vec<Vertex>, Vec<u16>)> {
    let v_per_side = level + 1;
    let float_lvl = level as f32;

    fn nvec3(v: &Vec3) -> nalgebra::Vec3<f32> {
        nalgebra::Vec3::new(v[0], v[1], v[2])
    }

    Box::new(move || {
        println!("Running: {}", level);
        let vertices = (0..v_per_side).map(|i| {
            let a = (i as f32) / float_lvl;
            let b = 1.0 - a;

            nvec3(&control_points[0].position) * (b * b) +
                nvec3(&control_points[3].position) * (2.0 * b * a) +
                nvec3(&control_points[6].position) * (a * a)
        }).chain(
            (1..v_per_side).flat_map(|i| {
                let a = (i as f32) / float_lvl;
                let b = 1.0 - a;

                let temp = to_3_arr! {
                    (0..3).map(|i| {
                        let k = 3usize * i;
                        nvec3(&control_points[k].position) * (b*b) +
                            nvec3(&control_points[k + 1].position) * (2.0*b*a) +
                            nvec3(&control_points[k + 2].position) * (a*a)
                    })
                };

                (0..v_per_side).map(|j| {
                    let a = (j as f32) / float_lvl;
                    let b = 1.0 - a;

                    temp[0] * (b * b) +
                        temp[1] * (2.0 * b * a) +
                        temp[2] * (a * a)
                }).collect::<Vec<_>>()
            })
        ).map(|v|
            Vertex {
                position: [v.x, v.y, v.z],
                ..control_points[0].clone()
            }
        ).collect::<Vec<_>>();

        let indices = (0..level-1)
            .step_by(v_per_side)
            .cartesian_product(0..v_per_side)
            .flat_map(|(row, col)|
                vec![
                    row + col + 2 * v_per_side,
                    row + col + v_per_side,
                    row + col,
                ]
            )
            .map(|i| i as u16)
            .collect::<Vec<_>>();

        (vertices, indices)
    })
}

fn get_patch_data(control_points: [Vertex; 9]) -> PatchData {
    let count_ctrl = Rc::new(control_points);

    let subs = to_8_arr! {
        (MIN_PATCH_SUBDIVISION_LEVELS..MAX_PATCH_SUBDIVISION_LEVELS).map(|i|
            Lazy::new(get_tessellate_fn(count_ctrl.clone(), i))
        )
    };

    PatchData(subs)
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
    verts: &[Vertex],
    textures: &[Texture],
    lightmaps: &[Rc<Texture2d>]
) -> Face {
    Face {
        texture: textures[face.texture_index as usize].clone(),
        lightmap: if face.lightmap_index < 0 {
            None
        } else {
            Some(lightmaps[face.lightmap_index as usize].clone())
        },
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
                    (0..(face.size[0] - 1) as usize / 2)
                        .cartesian_product(0..(face.size[1] - 1) as usize / 2)
                        .map(|(x, y)| {
                            to_9_arr! {
                                get_offset_grid(x, y, face.size[0] as usize)
                                    .into_iter()
                                    .map(|i| &verts[*i])
                                    .map(|v| v.clone())
                                    .collect::<Vec<_>>()
                            }
                        })
                        .map(get_patch_data)
                        .collect::<Vec<_>>()
                ),
            FaceType::Billboard =>
                FaceRenderType::Billboard(
                    0 // TODO: support things proper-like
                ),
        },
    }
}

fn build_model(
    mdl: &RawModel,
    faces: &[RawFace],
    mesh_verts: &[RawMeshVertex],
    verts: &[Vertex],
    textures: &[Texture],
    lightmaps: &[Rc<Texture2d>]
) -> Model {
    let faces = {
        let start = mdl.first_face as usize;
        let end = start + mdl.num_faces as usize;
        start..end
    }
    .map(|i| &faces[i])
    .map(|f| build_face(f, mesh_verts, verts, textures, lightmaps))
    .collect::<Vec<_>>();

    Model {
        min: mdl.min,
        max: mdl.max,
        faces: faces,
        brushes: vec![], // TODO: unimplemented
    }
}

fn get_offset_grid(x: usize, y: usize, width: usize) -> [usize; 9] {
    let (x, y) = (x*2, y*2);

    [
        x + y * width, x + y * width + 1, x + y * width + 2,
        x + (y+1) * width, x + (y+1) * width + 1, x + (y+1) * width + 2,
        x + (y+2) * width, x + (y+2) * width + 1, x + (y+2) * width + 2,
    ]
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

fn build_leaves(
    raw: &RawBsp,
    textures: &[Texture],
    lightmaps: &[Rc<Texture2d>]
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
    let verts = &raw.vertices;
    let clusters = raw.leaves.iter()
        .sorted_by(|a, b|
            a.visdata_cluster.cmp(&b.visdata_cluster)
        )
        .into_iter()
        .group_by(|l| l.visdata_cluster);

    let mut out = clusters.map(|(cluster, ref group)| {
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
            .sorted_by(i32::cmp)
            .into_iter()
            .unique()
            .map(|i| &faces[i as usize])
            .map(|f| build_face(f, mesh_verts, verts, textures, lightmaps))
            .zip(
                group.iter()
                    .flat_map(get_brushes)
                    .map(|lb| lb.index)
                    .sorted_by(i32::cmp)
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

    if out[0].cluster < 0 {
        println!("Some out-of-map leaves");
        out.remove(0);
    } else {
        println!("No out-of-map leaves");
    }

    for l in &out {
        *l.visdata.borrow_mut() = get_indices(
            &visibility_data.raw_bytes[{
                let start = (l.cluster as usize) *
                    visibility_data.sizeof_vector as usize;
                let end = start + visibility_data.sizeof_vector as usize;
                start..end
            }]
        ).into_iter()
            .map(|i| {
                debug_assert_eq!(i as isize, out[i].cluster);
                Rc::downgrade(&out[i])
            })
            .collect::<Vec<_>>();
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

        if visdata_cluster >= 0 {
            let pntr = &leaves[visdata_cluster as usize];
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

fn build_lightmaps<T: Facade>(
    builder: &mut TextureBuilder<T>,
    lightmaps: Vec<Lightmap>
) -> Vec<Rc<Texture2d>> {
    lightmaps.into_iter()
        .map(|l|
            l.colors
        )
        .map(|c|
            c.into_iter().map(|r|
                r.into_iter().map(|rgb|
                    (rgb[0], rgb[1], rgb[2])
                )
                .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>()
        )
        .map(|d|
             Rc::new(builder.create_raw(d).unwrap())
        )
        .collect()
}

pub fn build_bsp<T: Facade>(
    mut raw: RawBsp,
    texture_builder: &mut TextureBuilder<T>
) -> (Vec<Entity>, Bsp) {
    let tex = build_textures(
        &raw.textures,
        texture_builder
    ).expect("Missing texture is missing");

    let raw_lightmaps = replace(&mut raw.light_maps, vec![]);
    let lightmaps = build_lightmaps(texture_builder, raw_lightmaps);

    let root = {
        let leaves = build_leaves(&mut raw, &tex, &lightmaps);
        let mut nodes = build_nodes(&mut raw, &leaves);
        nodes.remove(0)
    };

    let vertices = replace(&mut raw.vertices, vec![]);
    let ents = replace(&mut raw.entities, vec![]);

    let world = build_model(
        &raw.models[0],
        &raw.faces,
        &raw.mesh_vertices,
        &vertices,
        &tex,
        &lightmaps
    );

    (
        ents,
        Bsp::new(
            Rc::try_unwrap(root).unwrap_or_else(|_|
                panic!("Bsp has circular nodegraph")
            ),
            world,
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
