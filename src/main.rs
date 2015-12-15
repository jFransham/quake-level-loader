#![feature(test)]
#![feature(iter_arith)]
#![feature(path_ext)]

#[macro_use]
extern crate nom;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate itertools;
extern crate image;
extern crate glium;

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

pub const SIMPLE_DM5: &'static [u8] = include_bytes!(
    "../assets/simple-dm5.bsp"
);
pub const TRESPASS: &'static [u8] = include_bytes!(
    "../assets/trespass.bsp"
);
pub const WATER_GIANT: &'static [u8] = include_bytes!(
    "../assets/casdm9v1.bsp"
);

fn main() {
    use glium::DisplayBuild;

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let map = get_map(&display);
    println!("{:?}", map.get_visible_set_at([0.0, 0.0, 0.0]).len());
}

fn get_map<T: glium::backend::Facade>(f: &T) -> bsp_transform::Bsp {
    let mut builder =
        TextureBuilder::new(
            vec!["assets/trespass"],
            f
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
