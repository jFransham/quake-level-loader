#![cfg_attr(test, feature(test))]
#![feature(iter_arith)]
#![feature(path_ext)]

#[macro_use]
extern crate nom;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate itertools;
extern crate image;
#[macro_use]
extern crate glium;
extern crate eventual;

#[macro_use]
mod macros;
mod bsp_transform;
mod directory_header;
mod helpers;
mod raw_bsp;
mod raw_bsp_parsers;
mod texture_flags;
mod texture;

use nom::IResult::*;
use raw_bsp_parsers::*;
use texture::*;
use bsp_transform::*;
use std::rc::Rc;

pub const SIMPLE_DM5: &'static [u8] = include_bytes!(
    "../assets/simple-dm5.bsp"
);
pub const TRESPASS: &'static [u8] = include_bytes!(
    "../assets/trespass.bsp"
);
pub const WATER_GIANT: &'static [u8] = include_bytes!(
    "../assets/casdm9v1.bsp"
);

fn get_mesh_verts(leaves: &[Rc<Leaf>]) -> Vec<u16> {
    leaves.iter()
        .flat_map(|l|
            l.faces.iter()
                .flat_map(|f|
                    if let FaceRenderType::Mesh(ref v) = f.render_type {
                        v.clone().into_iter()
                    }
                    else {
                        vec![].into_iter()
                    }
                )
                .map(|i| i as u16)
        )
        .collect::<Vec<_>>()
}

fn main() {
    use glium::{DisplayBuild, VertexBuffer, IndexBuffer, Program, Surface};
    use glium::index::PrimitiveType;
    use glium::glutin::WindowBuilder;

    let vertex_shader_src = r#"
        #version 140

        in vec3 position;

        void main() {
            gl_Position = vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let display = WindowBuilder::new()
        .build_glium()
        .unwrap();
    let map = get_map(&display);
    let vbuffer = VertexBuffer::new(&display, map.get_vertices()).unwrap();
    let ibuffer = IndexBuffer::new(
        &display,
        PrimitiveType::TrianglesList,
        &get_mesh_verts(&map.get_visible_set_at([0.0, 0.0, 0.0])),
    ).unwrap();
    let program = Program::from_source(
        &display,
        vertex_shader_src,
        fragment_shader_src,
        None
    ).unwrap();
    println!("{}", ibuffer.len());

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(
            &vbuffer, &ibuffer, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}

fn get_map<T: glium::backend::Facade>(f: &T) -> bsp_transform::Bsp {
    use eventual::*;

    let mut builder =
        TextureBuilder::new(
            vec!["assets/trespass"],
            f,
            Some("textures/common/missing".into())
        );
    match parse_raw_bsp(WATER_GIANT) {
        Done(_, bsp)  => {
            // Ignore entities for now
            bsp_transform::build_bsp(
                bsp,
                &mut builder
            ).1
        },
        Incomplete(n) => panic!("Incomplete: {:?}", n),
        Error(_)      => panic!("Failed :("),
    }
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
                raw_bsp_parsers::parse_raw_bsp(super::SIMPLE_DM5).is_done()
            )
        });
    }

    #[bench]
    pub fn bench_complex(b: &mut Bencher) {
        b.iter(|| {
            assert!(
                raw_bsp_parsers::parse_raw_bsp(super::TRESPASS).is_done()
            )
        });
    }

    #[bench]
    pub fn bench_huge(b: &mut Bencher) {
        b.iter(|| {
            assert!(
                raw_bsp_parsers::parse_raw_bsp(super::WATER_GIANT).is_done()
            )
        });
    }
}
