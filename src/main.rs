#![feature(test)]
#![feature(drain)]
#![feature(iter_arith)]

#[macro_use]
extern crate nom;
#[macro_use]
extern crate bitflags;
extern crate itertools;
extern crate glium;

#[macro_use]
mod macros;
mod bsp_transform;
mod directory_header;
mod helpers;
mod raw_bsp;
mod raw_bsp_parsers;
mod texture_flags;

use itertools::*;
use nom::IResult::*;
use raw_bsp::*;
use raw_bsp_parsers::*;

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
    match parse_raw_bsp(SIMPLE_DM5) {
        Done(_, bsp)  => println!(
            "{:#?}",
            bsp.brushes
                .iter()
                .map(|b| (b, b.texture_index))
                .sorted_by(|&a, &b| a.1.cmp(&b.1))
                .into_iter()
                .unique_by(|b| b.1)
                .map(|(b, i)|
                     (
                         bsp.textures.get(i as usize).unwrap(),
                         bsp.brush_sides[
                             {
                                 let start = b.first_brush_side as usize;
                                 let end   = start + b.num_brush_sides as usize;
                                 start..end
                             }
                         ].iter().map(|s|
                             bsp.textures.get(
                                 s.texture_index as usize
                             ).unwrap()
                         ).collect::<Vec<_>>(),
                     )
                ).collect::<Vec<_>>()
        ),
        Incomplete(n) => println!("Incomplete: {:?}", n),
        Error(_)      => println!("Failed :("),
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
