extern crate bindgen;
extern crate fs_extra;

mod build_support;

use crate::build_support::{Builder, PlatformBuilder};

fn main() {
    let builder = Box::new(PlatformBuilder::default());

    builder.init_submodules();

    if !builder.is_compiled() {
        builder.generate_sources();
        builder.compile_sources();
    }

    builder.link_libraries();
    builder.generate_bindings();
}
