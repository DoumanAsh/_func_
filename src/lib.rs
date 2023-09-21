//! Proc macro to insert function name within body of function
//!
//! ## Usage
//!
//! ```rust
//! use func_::_func_;
//!
//! #[_func_]
//! fn my_func() {
//!     assert_eq!(__func__, "my_func");
//!
//!     println!("{__func__}: log with function name");
//! }
//!
//! #[_func_]
//! fn my_generic_func<T>() {
//!     assert_eq!(__func__, "my_generic_func");
//!
//!     println!("{__func__}: log with generic function name but without generics bullshit cuz why the would I want generics in my generic name?");
//! }
//!
//! #[_func_]
//! const fn const_func() {
//!     assert!(__func__.len() == "const_func".len());
//! }
//!
//! #[_func_]
//! async fn async_func() {
//!     assert_eq!(__func__, "async_func");
//! }
//!
//! my_func();
//! my_generic_func::<u16>();
//! const_func();
//! ```

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree, Span, Delimiter};

use core::fmt::Write;

#[cold]
#[inline(never)]
fn compile_error(span: Span, error: &str) -> TokenStream {
    if let Some(source) = span.source_text() {
        format!("compile_error!(\"{source}\n\n{error}\");").parse().unwrap()
    } else {
        format!("compile_error!(\"{error}\");").parse().unwrap()
    }
}

#[proc_macro_attribute]
///Adds constant `__func__` with name of function to the function's body.
pub fn _func_(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut fn_name = String::new();
    let mut output = String::new();
    let mut parts = input.into_iter();

    //Skip until `fn` ident
    while let Some(part) = parts.next() {
        if let TokenTree::Ident(ident) = part {
            let ident_span = ident.span();
            let ident = ident.to_string();
            if ident == "fn" {
                write!(&mut output, "{ident} ").expect("Cannot format");
                match parts.next() {
                    Some(TokenTree::Ident(ident)) => {
                        fn_name = ident.to_string();
                        write!(&mut output, "{fn_name}").expect("Cannot format");
                        break;
                    },
                    Some(unexpected) => return compile_error(unexpected.span(), "Expected ident"),
                    None => return compile_error(ident_span, "No function name follows"),
                };
            } else {
                write!(&mut output, "{ident} ").expect("Cannot format");
            }
        } else {
            write!(&mut output, "{part} ").expect("Cannot format");
        }
    }

    if fn_name.is_empty() {
        compile_error(Span::call_site(), "No function name found, please write valid code");
    }

    //Extract code block and insert our constant before pasting rest of code
    while let Some(part) = parts.next() {
        if let TokenTree::Group(code_block) = &part {
            if let Delimiter::Brace = code_block.delimiter() {
                output.push('{');
                output.push('\n');
                write!(&mut output, "    const __func__: &str = \"{fn_name}\";\n").expect("Cannot format");
                write!(&mut output, "{}\n", code_block.stream()).expect("Cannot format");
                output.push('}');
                break;
            } else {
                //should not be the case, not going to validate your stupid syntax error
                write!(&mut output, "{part} ").expect("Cannot format");
            }
        } else {
            write!(&mut output, "{part} ").expect("Cannot format");
        }
    }

    for part in parts {
        let _ = write!(&mut output, "{part}");
    }

    match output.parse() {
        Ok(output) => output,
        Err(error) => compile_error(
            Span::call_site(),
            &format!("codegen generated unexpectedly invalid code:\n{error}\n-----Output:\n{output}"),
        )
    }
}
