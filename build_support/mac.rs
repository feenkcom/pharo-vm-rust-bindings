use crate::build_support::Builder;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::process::Command;

#[derive(Default, Clone)]
pub struct MacBuilder {}

impl MacBuilder {

}

impl Debug for MacBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.print_directories(f)
    }
}

impl Builder for MacBuilder {
    fn vm_binary(&self) -> PathBuf {
        self.output_directory().join("libPharoVMCore.a")
    }

    fn generate_sources(&self) {
        Command::new("cmake")
            .arg(self.cmake_build_type())
            .arg("-DFEATURE_LIB_SDL2=OFF")
            .arg("-DFEATURE_LIB_CAIRO=OFF")
            .arg("-DFEATURE_LIB_FREETYPE2=OFF")
            .arg("-DFEATURE_LIB_GIT2=OFF")
            .arg("-DBUILD_BUNDLE=OFF")
            .arg("-DCOMPILE_STATIC_LIBRARIES=ON")
            .arg("-DCOMPILE_EXECUTABLE=OFF")
            .arg("-DPHARO_DEPENDENCIES_PREFER_DOWNLOAD_BINARIES=OFF")
            .arg("-S")
            .arg(self.vm_sources_directory())
            .arg("-B")
            .arg(self.output_directory())
            .status()
            .unwrap();
    }

    fn platform_include_directory(&self) -> PathBuf {
        self.squeak_include_directory().join("osx")
    }

    fn generated_config_directory(&self) -> PathBuf {
        self.output_directory()
            .join("build")
            .join("include")
            .join("pharovm")
    }

    fn link_libraries(&self) {
        println!("cargo:rustc-link-search=native={}", self.output_directory().display());
        println!("cargo:rustc-link-lib=static=PharoVMCore");
        println!("cargo:rustc-link-lib=framework=AppKit");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
    }
}
