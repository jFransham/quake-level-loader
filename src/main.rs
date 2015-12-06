#[macro_use]
extern crate nom;

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

fn main() {
    let b = include_bytes!(
        "../assets/simple-dm5.bsp"
    );

    match parse_raw_bsp(b) {
        Done(_, _) => println!("Success!"),
        Incomplete(n) => println!("Incomplete: {:?}", n),
        Error(_)   => println!("Failed :("),
    }
}
