macro_rules! itry {
    ($res:expr) => {
        match $res {
            Done(rest, result) => {
                (rest, result)
            },
            Error(e) => {
                return Error(e)
            },
            Incomplete(needed) => {
                println!("Incomplete at {}, {}", line!(), file!());
                return Incomplete(needed)
            }
        }
    }
}

macro_rules! get_from_header {
    ($bytes:expr, $field:expr, $fun:expr, $t:ty) => {{
        use std::mem::size_of;
        get_from_header!($bytes, $field, $fun, $t, size_of::<$t>())
    }};
    ($bytes:expr, $field:expr, $fun:expr, $t:ty, $size:expr) => {{
        let start = $field.offset as usize;
        let end = ($field.offset + $field.size) as usize;
        let slice = &$bytes[start..end];
        /*
        println!("Number of {}: {} ({}b each) in section {}-{} ({}b)",
            stringify!($t),
            $field.size as usize / $size,
            $size,
            start,
            end,
            slice.len()
        );
        */
        itry!(
            parse_vec::<$t>(
                slice,
                $fun,
                $field.size as usize / $size
            )
        )
    }}
}

macro_rules! take_s {
    ($count:expr) => {{
        |i: &[u8]| take_s!(i, $count)
    }};
    ($i:expr, $count:expr) => {{
        match take!($i, $count) {
            Done(rest, arr) =>
                if let Ok(s) = String::from_utf8(
                    arr.into_iter()
                        .map(|&a| a)
                        .take_while(|&c| c != 0)
                        .collect::<Vec<_>>()
                ) {
                    Done(
                        rest,
                        s
                    )
                } else {
                    use nom::ErrorKind;
                    Error(Err::Code(ErrorKind::Custom(0)))
                },
            Error(e) => {
                Error(e)
            },
            Incomplete(needed) => {
                Incomplete(needed.into())
            }
        }
    }};
}

macro_rules! take_exact {
    ($count:expr) => {{
        |i: &[u8]| take_exact!(i, $count)
    }};
    ($slice:expr, $count:expr) => {{
        let taken = take!($slice, $count);
        match taken {
            Done(rest, result) => {
                let mut array = [0u8; $count];
                for (&x, p) in result.iter().zip(array.iter_mut()) {
                    *p = x;
                }
                Done(rest, array)
            },
            Error(e) => {
                Error(e)
            },
            Incomplete(needed) => {
                Incomplete(needed.into())
            }
        }
    }}
}
