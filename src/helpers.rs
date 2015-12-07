use nom::{IResult,le_i32,le_u32};
use nom::IResult::*;
use nom::Err;
use std::mem::transmute;

pub type Vec3 = [f32; 3];
pub type Vec2 = [f32; 2];
pub type IVec3 = [i32; 3];
pub type IVec2 = [i32; 2];
pub type Rgb = [u8; 3];
pub type Rgba = [u8; 4];

/// Recognizes little endian 4 bytes floating point number
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

pub fn consume_to_vec<T>(
    input: &[u8],
    fun: fn(&[u8]) -> IResult<&[u8], T>
) -> IResult<&[u8], Vec<T>> {
    let mut output = Vec::new();
    let mut bytes: &[u8] = input;
    while bytes.len() > 0 {
        let (rest, result) = itry!(fun(bytes));
        bytes = rest;
        output.push(result);
    }

    Done(bytes, output)
}

pub fn parse_vec<T>(
    input: &[u8],
    fun: fn(&[u8]) -> IResult<&[u8], T>,
    count: usize
) -> IResult<&[u8], Vec<T>> {
    let mut output = Vec::with_capacity(count);
    let mut bytes: &[u8] = input;
    for _ in 0..count {
        let (rest, result) = itry!(fun(bytes));
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

fn treat_as_whitespace(i: u8) -> bool {
    let c = i as char;
    c.is_whitespace() || c.is_control()
}

named! {
    pub whitespace,
    take_while!(treat_as_whitespace)
}

named! {
    pub mandatory_whitespace,
    take_while1!(treat_as_whitespace)
}

pub fn parse_str_float(i: &[u8]) -> IResult<&[u8], f32> {
    use nom::ErrorKind;

    fn is_dec(a: u8) -> bool {
        let c = a as char;
        c.is_digit(10)
    }

    let (rest, neg) = itry!(opt!(i, tag!(b"-")));
    let (rest, pre_dot) = itry!(take_while1!(rest, is_dec));
    let (rest, maybe_dot_and_num) = itry!(
        opt!(rest,
            chain!(
                a: tag!(b".")           ~
                b: take_while1!(is_dec) ,
                || {
                    a.into_iter().chain(b.into_iter())
                }
            )
        )
    );

    if let Ok(Ok(float)) = String::from_utf8(
        neg.into_iter()
            .flat_map(|a| a.into_iter())
            .chain(pre_dot.into_iter())
            .chain(maybe_dot_and_num.into_iter().flat_map(|a| a))
            .map(|&a| a)
            .take_while(|&c| c != 0)
            .collect::<Vec<_>>()
    ).map(|s| s.parse()) {
        Done(
            rest,
            float
        )
    } else {
        Error(Err::Code(ErrorKind::Custom(0)))
    }
}

pub fn parse_str_int(i: &[u8]) -> IResult<&[u8], i32> {
    use nom::ErrorKind;

    fn is_dec(a: u8) -> bool {
        let c = a as char;
        c.is_digit(10)
    }

    let (rest, neg) = itry!(opt!(i, tag!(b"-")));
    let (rest, i_bytes) = itry!(take_while1!(rest, is_dec));

    if let Ok(Ok(int)) = String::from_utf8(
        neg.into_iter()
            .flat_map(|a| a.into_iter())
            .chain(i_bytes.into_iter())
            .map(|&a| a)
            .take_while(|&c| c != 0)
            .collect::<Vec<_>>()
    ).map(|s| s.parse()) {
        Done(
            rest,
            int
        )
    } else {
        Error(Err::Code(ErrorKind::Custom(0)))
    }
}
