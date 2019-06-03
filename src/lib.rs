#![doc(html_root_url = "https://docs.rs/assertive/0.1.0")]
#![deny(missing_debug_implementations, missing_docs, unreachable_pub, rust_2018_idioms)]
#![cfg_attr(test, deny(warnings))]

//! Collection of assertions.

/// Asserts that the expression evaluates to `Ok` and returns the value.
///
/// This will invoke the `panic!` macro if the provided expression does not evaluate to `Ok` at
/// runtime.
///
/// # Custom Messages
///
/// This macro has a second form, where a custom panic message can be provided with or without
/// arguments for formatting.
///
/// # Examples
///
/// ```
/// use assertive::assert_ok;
///
/// let n: u32 = assert_ok!("123".parse());
///
/// let s = "123";
/// let n: u32 = assert_ok!(s.parse(), "testing parsing {:?} as a u32", s);
/// ```
#[macro_export]
macro_rules! assert_ok {
    ($e:expr) => {
        assert_ok!($e,)
    };
    ($e:expr,) => {{
        use std::result::Result::*;
        match $e {
            Ok(v) => v,
            Err(e) => panic!("assertion failed: Err({:?})", e),
        }
    }};
    ($e:expr, $($arg:tt)+) => {{
        use std::result::Result::*;
        match $e {
            Ok(v) => v,
            Err(e) => panic!("assertion failed: Err({:?}): {}", e, format_args!($($arg)+)),
        }
    }};
}

/// Asserts that the expression evaluates to `Err` and returns the error.
///
/// This will invoke the `panic!` macro if the provided expression does not evaluate to `Err` at
/// runtime.
///
/// # Custom Messages
///
/// This macro has a second form, where a custom panic message can be provided with or without
/// arguments for formatting.
///
/// # Examples
///
/// ```
/// use assertive::assert_err;
/// use std::str::FromStr;
///
///
/// let err = assert_err!(u32::from_str("fail"));
///
/// let msg = "fail";
/// let err = assert_err!(u32::from_str(msg), "testing parsing {:?} as u32", msg);
/// ```
#[macro_export]
macro_rules! assert_err {
    ($e:expr) => {
        assert_err!($e,);
    };
    ($e:expr,) => {{
        use std::result::Result::*;
        match $e {
            Ok(v) => panic!("assertion failed: Ok({:?})", v),
            Err(e) => e,
        }
    }};
    ($e:expr, $($arg:tt)+) => {{
        use std::result::Result::*;
        match $e {
            Ok(v) => panic!("assertion failed: Ok({:?}): {}", v, format_args!($($arg)+)),
            Err(e) => e,
        }
    }};
}
