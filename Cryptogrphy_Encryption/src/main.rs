#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case,
    deprecated
)]

mod openssl;
mod base64;
mod sha;

fn main()
{
    // openssl::test_all();
     base64::test_all();
    // sha::test_all();
}
