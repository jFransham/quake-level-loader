#![feature(test)]

#[macro_use]
extern crate nom;
#[macro_use]
extern crate bitflags;

#[macro_use]
mod macros;
mod raw_bsp;
mod raw_bsp_parsers;
mod directory_header;
mod texture_flags;
mod helpers;

use nom::IResult::*;
use raw_bsp_parsers::*;

/*********************************
 * All numbers are little-endian *
 *********************************/

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
    match parse_raw_bsp(TRESPASS) {
        Done(_, bsp)  => println!("{:#?}", bsp.planes),
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
