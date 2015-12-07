#![feature(test)]

#[macro_use]
extern crate nom;
extern crate test;

#[macro_use]
mod macros;
mod raw_bsp;
mod raw_bsp_parsers;
mod directory_header;
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

fn main() {
    match parse_raw_bsp(TRESPASS) {
        Done(_, bsp) => println!("Success! Entities =\n{}", bsp.entities),
        Incomplete(n) => println!("Incomplete: {:?}", n),
        Error(_)   => println!("Failed :("),
    }
}

mod test_main {
    use test::Bencher;
    use super::raw_bsp_parsers;

    #[bench]
    pub fn bench_simple(b: &mut Bencher) {
        b.iter(|| {
            raw_bsp_parsers::parse_raw_bsp(super::SIMPLE_DM5);
        });
    }

    #[bench]
    pub fn bench_complex(b: &mut Bencher) {
        b.iter(|| {
            raw_bsp_parsers::parse_raw_bsp(super::TRESPASS);
        });
    }
}
