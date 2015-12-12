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
        let slice = &$bytes[start..];
        itry!(
            parse_vec::<$t>(
                slice,
                $fun,
                $field.size as usize / $size
            )
        )
    }}
}

macro_rules! from_header {
    ($bytes:expr, $field:expr, $fun:ident, $t:ty) => {{
        let start = $field.offset as usize;
        let end = ($field.offset + $field.size) as usize;
        let slice = &$bytes[start..end];
        itry!(
            $fun(slice)
        )
    }}
}

macro_rules! consume_from_header {
    ($bytes:expr, $field:expr, $fun:expr, $t:ty) => {{
        let start = $field.offset as usize;
        let end = start + $field.size as usize;
        let slice = &$bytes[start..end];
        itry!(
            consume_to_vec::<$t>(
                slice,
                $fun
            )
        )
    }}
}

macro_rules! take_s {
    ($count:expr) => {{
        |i: &[u8]| take_s!(i, $count)
    }};
    ($i:expr, $count:expr) => {{
        use nom::ErrorKind;

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

macro_rules! take_s_until {
    ($c:expr) => {{
        |i: &[u8]| take_s!(i, $c)
    }};
    ($i:expr, $c:expr) => {{
        use nom::ErrorKind;

        match take_until!($i, $c) {
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
