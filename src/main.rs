#![cfg_attr(test, feature(test))]
#![feature(iter_arith)]
#![feature(time2)]
#![feature(fnbox)]
#![feature(step_by)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]

#[macro_use]
extern crate nom;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate itertools;
extern crate image;
extern crate nalgebra;
#[macro_use]
extern crate glium;
extern crate eventual;
extern crate rand;

#[macro_use]
mod macros;
mod directory_header;
mod helpers;
mod bsp;
mod raw_bsp;
mod raw_bsp_parsers;
mod lazy;
mod texture_flags;
mod texture;

use nom::IResult::*;
use raw_bsp_parsers::*;
use texture::*;
use texture_flags::*;
use bsp::*;
use std::rc::Rc;
use glium::{
    Program,
    IndexBuffer,
    VertexBuffer,
    Surface,
};
use glium::index::PrimitiveType;
use glium::backend::Facade;
use glium::backend::glutin_backend::GlutinFacade;
use glium::texture::Texture2d;

pub static SIMPLE_DM5: &'static str = "assets/simple-dm5.bsp";
pub static TRESPASS: &'static str = "assets/trespass.bsp";
pub static WATER_GIANT: &'static str = "assets/casdm9v1.bsp";

type Matrix = [[f32; 4]; 4];

/*
 * fn get_indices_from_leaves(leaves: &[Rc<Leaf>]) -> Vec<u16> {
    leaves.iter()
        .flat_map(|l| get_indices_from_faces(&l.faces))
        .collect::<Vec<_>>()
}
*/

fn get_indices_from_faces(faces: &[&Face]) -> Vec<u16> {
    faces.iter()
        .flat_map(|f|
            if let FaceRenderType::Mesh(ref v) = f.render_type {
                v.clone().into_iter()
            } else {
                vec![].into_iter()
            }
        )
        .map(|i| i as u16)
        .collect::<Vec<_>>()
}

static VERTEX_SHADER_SRC: &'static str = r#"
    #version 140

    in vec3 position;
    in vec2 surface_coords;
    in vec2 lightmap_coords;
    in vec3 normal;
    in vec4 color;

    uniform mat4 u_View;
    uniform mat4 u_Proj;

    smooth out vec2 tex_coords;

    void main() {
        gl_Position = u_Proj * u_View * vec4(position, 1.0);
        tex_coords = surface_coords;
    }
"#;

static FRAGMENT_SHADER_SRC: &'static str = r#"
    #version 140

    smooth in vec2 tex_coords;
    out vec4 color;

    uniform sampler2D u_Texture;

    void main() {
        color = texture(u_Texture, vec2(1 - tex_coords.x, 1 - tex_coords.y));
    }
"#;

fn view_matrix(
    position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]
) -> Matrix {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0]
    ];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0]
    ];

    let p = [
        -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]
    ];

    [
        [s[0], u[0], f[0], 0.0],
        [s[1], u[1], f[1], 0.0],
        [s[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

fn main() {
    use std::time::*;
    use glium::glutin::{Event, WindowBuilder};
    use glium::DisplayBuild;
    use nalgebra::{Vec3, PerspMat3, Rot3};
    use rand::Rng;

    let display = WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium()
        .unwrap();
    let map = get_map(&display);
    let map_vertices = map.get_vertices();
    println!("Vertices size: {}b", map_vertices.len() * std::mem::size_of::<raw_bsp::Vertex>());
    let vbuffer = VertexBuffer::new(&display, map_vertices).unwrap();
    let program = Program::from_source(
        &display,
        VERTEX_SHADER_SRC,
        FRAGMENT_SHADER_SRC,
        None
    ).unwrap();

    let (mut x, mut y, mut z, mut rotxy, mut rotz) = (
        0f32, 200f32, -100f32, 3.141592 / 2.0, 0f32
    );
    let mut rng = rand::thread_rng();
    while map.get_visible_set_at([x, y, z]).len() == 0 {
        use rand::distributions::{IndependentSample, Range};
        let range = Range::new(-5000.0f32, 5000.0f32);
        x = range.ind_sample(&mut rng);
        y = range.ind_sample(&mut rng);
        z = range.ind_sample(&mut rng);
    }

    let mut vel = 0.0;
    let mut acc = 0.001;

    let start = Instant::now();
    let mut last = start.clone();
    let mut period = 0u64;

    loop {
        let now = Instant::now();
        let elapsed = now.duration_from_earlier(last);
        /*
        println!(
            "{}",
            1_000_000_000.0 / (
                elapsed.as_secs() * 1_000_000_000 +
                    elapsed.subsec_nanos() as u64
            ) as f64
        );
        */
        last = now;

        let total_elapsed = now.duration_from_earlier(start);
        let elapsed_nano =
            total_elapsed.as_secs() * 1_000_000_000 +
            total_elapsed.subsec_nanos() as u64;

        let perspective = {
            let (width, height) = (800, 600);
            let aspect_ratio = height as f32 / width as f32;

            let fov = 3.141592 / 2.0;
            let zfar = 8192.0 * 8.0;
            let znear = 0.1;

            PerspMat3::new(aspect_ratio, fov, znear, zfar)
        };

        let v: Matrix = view_matrix(
                &[x, y, z],
                (
                    Rot3::new_with_euler_angles(0.0, rotz, rotxy) *
                        Vec3::new(0.0, -1.0, 0.0)
                ).as_ref(),
                (
                    Rot3::new_with_euler_angles(0.0, rotz, rotxy) *
                        Vec3::new(0.0, 0.0, 1.0)
                ).as_ref()
            );
        let p: Matrix = perspective
            .to_mat()
            .as_ref()
            .clone();
        let uniforms = uniform! {
            u_View: v,
            u_Proj: p,
        };

        let leaves = map.get_visible_set_at([x, y, z]);
        let faces = leaves.iter()
            .flat_map(|l| &l.faces)
            .chain(
                map.get_world().into_iter()
            )
            .collect::<Vec<_>>();

        render(
            &display,
            &program,
            p,
            v,
            &vbuffer,
            faces
        );

        rotxy = elapsed_nano as f32 * 0.0000000002 %
            (std::f32::consts::PI * 2.0);

        let now_period = elapsed_nano / 10_000_000_000;
        if period != now_period {
            loop {
                use rand::distributions::{IndependentSample, Range};
                let (mins, maxs) = map.get_bounds();
                let (range_x, range_y, range_z) = (
                    Range::new(mins[0] as f32, maxs[0] as f32),
                    Range::new(mins[1] as f32, maxs[1] as f32),
                    Range::new(mins[2] as f32, maxs[2] as f32),
                );
                x = range_x.ind_sample(&mut rng);
                y = range_y.ind_sample(&mut rng);
                z = range_z.ind_sample(&mut rng);

                if map.get_visible_set_at([x, y, z]).len() != 0 { break; }
            }
            period = now_period;
        }

        for ev in display.poll_events() {
            match ev {
                Event::Closed => return,
                _ => ()
            }
        }
    }
}

fn render<'a, I: IntoIterator<Item=&'a Face>>(
    display: &GlutinFacade,
    program: &Program,
    projection: Matrix,
    view: Matrix,
    vbuffer: &VertexBuffer<Vertex>,
    iter: I
) {
    use itertools::*;
    use std::sync::Arc;

    struct ArcCmp<T>(Arc<T>);
    impl<T> PartialEq<ArcCmp<T>> for ArcCmp<T> {
        fn eq(&self, other: &ArcCmp<T>) -> bool {
            (self.0.as_ref() as *const _) == (other.0.as_ref() as *const _)
        }
    }

    let buffers = iter.into_iter()
        .sorted_by(|a, b|
            (a.texture.texture.as_ref() as *const _).cmp(
                &(b.texture.texture.as_ref() as *const _)
            )
        )
        .into_iter()
        .filter(|f| f.texture.surface_flags.should_draw())
        .group_by(|f| ArcCmp(f.texture.texture.clone()))
        .into_iter()
        .map(|(ArcCmp(tex), faces)|
            (
                tex,
                IndexBuffer::new(
                    display,
                    PrimitiveType::TrianglesList,
                    &get_indices_from_faces(&faces)
                ).unwrap()
            )
        );

    let mut target = display.draw();

    target.clear_depth(1.0);

    for (t, b) in buffers {
        let uniforms = uniform! {
            u_View: view,
            u_Proj: projection,
            u_Texture: &*t,
        };

        render_one(
            &mut target,
            program,
            uniforms,
            vbuffer,
            &b
        );
    }

    target.finish().unwrap();
}

fn render_one<U: glium::uniforms::Uniforms, S: glium::Surface>(
    s: &mut S,
    program: &Program,
    uniforms: U,
    vbuffer: &VertexBuffer<Vertex>,
    ibuffer: &IndexBuffer<u16>
) -> Result<(), glium::DrawError> {
    use glium::draw_parameters::*;

    s.draw(
        vbuffer, ibuffer, program, &uniforms,
        &DrawParameters {
            backface_culling: BackfaceCullingMode::CullCounterClockwise,
            smooth: Some(Smooth::Nicest),
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        }
    )
}

fn get_map<T: Facade>(f: &T) -> Bsp {
    let mut builder =
        TextureBuilder::new(
            vec!["assets", "assets/trespass", "assets/Casdm9v1"],
            f,
            Some("textures/common/missing".into())
        );

    match parse_raw_bsp(&load_all("assets/q3dm11.bsp")) {
        Done(e, bsp)  => {
            // Ignore entities for now
            let b = from_raw(
                bsp,
                &mut builder
            );
            b.1
        },
        Incomplete(n) => panic!("Incomplete: {:?}", n),
        Error(_)      => panic!("Failed :("),
    }
}

fn load_all(p: &str) -> Vec<u8> {
    use std::fs::File;
    use std::io::Read;

    let mut buf = vec![];
    let mut f = File::open(p).unwrap();
    f.read_to_end(&mut buf);
    buf
}

#[cfg(test)]
mod test_main {
    extern crate test;

    use self::test::Bencher;
    use super::raw_bsp_parsers;

    #[bench]
    pub fn bench_simple(b: &mut Bencher) {
        b.iter(|| {
            assert!(
                raw_bsp_parsers::parse_raw_bsp(super::load_all(super::SIMPLE_DM5)).is_done()
            )
        });
    }

    #[bench]
    pub fn bench_complex(b: &mut Bencher) {
        b.iter(|| {
            assert!(
                raw_bsp_parsers::parse_raw_bsp(super::load_all(super::TRESPASS)).is_done()
            )
        });
    }

    #[bench]
    pub fn bench_huge(b: &mut Bencher) {
        b.iter(|| {
            assert!(
                raw_bsp_parsers::parse_raw_bsp(super::load_all(super::WATER_GIANT)).is_done()
            )
        });
    }
}

