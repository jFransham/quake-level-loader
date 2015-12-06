use nom::{GetOutput,IResult,le_i32,le_u32,le_u64};
use nom::IResult::*;
use std::mem::transmute;

pub type Vec3 = [f32; 3];
pub type Vec2 = [f32; 2];
pub type IVec3 = [i32; 3];
pub type IVec2 = [i32; 2];
pub type Rgb = [u8; 3];
pub type Rgba = [u8; 4];

/// Recognizes big endian 4 bytes floating point number
#[inline]
pub fn le_f32(input: &[u8]) -> IResult<&[u8], f32> {
  match le_u32(input) {
    Error(e)      => Error(e),
    Incomplete(e) => Incomplete(e),
    Done(i,o) => {
      unsafe {
        Done(i, transmute::<u32, f32>(o))
      }
    }
  }
}

/// Recognizes big endian 8 bytes floating point number
#[inline]
pub fn le_f64(input: &[u8]) -> IResult<&[u8], f64> {
  match le_u64(input) {
    Error(e)      => Error(e),
    Incomplete(e) => Incomplete(e),
    Done(i,o) => {
      unsafe {
        Done(i, transmute::<u64, f64>(o))
      }
    }
  }
}

pub fn parse_vec<T, F: ?Sized>(
    input: &[u8],
    fun: Box<F>,
    count: usize
) -> IResult<&[u8], Vec<T>>
    where F: Fn(&[u8]) -> IResult<&[u8], T>
{
    let mut output = Vec::with_capacity(count);
    let mut bytes: &[u8] = input;
    for i in 0..count {
        let (rest, result) = itry!((*fun)(bytes));
        bytes = rest;
        output.push(result);
    }

    Done(bytes, output)
}

named! {
    pub parse_vec3 <Vec3>,
    chain!(
        v0: le_f32 ~
        v1: le_f32 ~
        v2: le_f32 ,
        || {
            [v0, v1, v2]
        }
    )
}

named! {
    pub parse_vec2 <Vec2>,
    chain!(
        v0: le_f32 ~
        v1: le_f32 ,
        || {
            [v0, v1]
        }
    )
}

named! {
    pub parse_ivec3 <IVec3>,
    chain!(
        v0: le_i32 ~
        v1: le_i32 ~
        v2: le_i32 ,
        || {
            [v0, v1, v2]
        }
    )
}

named! {
    pub parse_ivec2 <IVec2>,
    chain!(
        v0: le_i32 ~
        v1: le_i32 ,
        || {
            [v0, v1]
        }
    )
}

fn parse_name(i: &[u8]) -> IResult<&[u8], [u8; 64]> {
    take_exact!(i, 64)
}
