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
            Err(e) => panic!("assertion failed: error = {:?}", e),
        }
    }};
    ($e:expr, $($arg:tt)+) => {{
        use std::result::Result::*;
        match $e {
            Ok(v) => v,
            Err(e) => panic!("assertion failed: error = {:?}: {}", e, format_args!($($arg)+)),
        }
    }};
}
