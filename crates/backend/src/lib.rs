#![recursion_limit = "256"]
#![cfg_attr(feature = "extra-traits", deny(missing_debug_implementations))]
#![doc(html_root_url = "https://docs.rs/wasm-bindgen-backend/0.2")]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate serde_json;
extern crate syn;

extern crate wasm_bindgen_shared as shared;

pub mod ast;
mod codegen;
pub mod defined;
pub mod util;
