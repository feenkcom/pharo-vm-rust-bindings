use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fmt, fs};

pub trait Builder: Debug {
    fn is_compiled(&self) -> bool {
        self.vm_binary().exists()
    }

    fn profile(&self) -> String {
        std::env::var("PROFILE").unwrap()
    }

    fn is_debug(&self) -> bool {
        self.profile() == "debug"
    }

    fn should_embed_debug_symbols(&self) -> bool {
        std::env::var("VM_CLIENT_EMBED_DEBUG_SYMBOLS").map_or(false, |value| value == "true")
    }

    fn cmake_build_type(&self) -> String {
        (if self.is_debug() {
            "-DCMAKE_BUILD_TYPE=RelWithDebInfo"
        }
        else if self.should_embed_debug_symbols() {
            "-DCMAKE_BUILD_TYPE=RelWithDebInfo"
        }
        else {
            "-DCMAKE_BUILD_TYPE=Release"
        }).to_string()
    }

    fn output_directory(&self) -> PathBuf {
        Path::new(env::var("OUT_DIR").unwrap().as_str()).to_path_buf()
    }

    /// Return a path to the compiled vm binary.
    /// For example, on Mac it would be an executable inside of the .app bundle
    fn vm_binary(&self) -> PathBuf;

    fn vm_sources_directory(&self) -> PathBuf {
        std::env::current_dir().unwrap()
            .join("opensmalltalk-vm")
    }

    // git submodule update --init --recursive
    fn init_submodules(&self) {
        Command::new("git")
            .arg("submodule")
            .arg("update")
            .arg("--init")
            .arg("--recursive")
            .status()
            .unwrap();
    }

    fn generate_sources(&self);

    fn compile_sources(&self) {
        Command::new("cmake")
            .arg("--build")
            .arg(self.output_directory())
            .arg("--config")
            .arg(self.profile())
            .status()
            .unwrap();
    }

    fn squeak_include_directory(&self) -> PathBuf {
        self.vm_sources_directory()
            .join("extracted")
            .join("vm")
            .join("include")
    }

    fn common_include_directory(&self) -> PathBuf {
        self.squeak_include_directory().join("common")
    }

    fn platform_include_directory(&self) -> PathBuf;
    fn generated_config_directory(&self) -> PathBuf;

    fn generate_bindings(&self) {
        let include_dir = self.vm_sources_directory().join("include");

        let generated_vm_include_dir = self
            .output_directory()
            .join("generated")
            .join("64")
            .join("vm")
            .join("include");

        let bindings = bindgen::Builder::default()
            .whitelist_function("vm_.*")
            .whitelist_function("free")
            .header(
                include_dir
                    .join("pharovm")
                    .join("pharoClient.h")
                    .display()
                    .to_string(),
            )
            .clang_arg(format!("-I{}", &include_dir.display()))
            .clang_arg(format!("-I{}", &include_dir.join("pharovm").display()))
            .clang_arg(format!(
                "-I{}",
                &self.generated_config_directory().display()
            ))
            .clang_arg(format!("-I{}", &generated_vm_include_dir.display()))
            .clang_arg(format!("-I{}", self.common_include_directory().display()))
            .clang_arg(format!("-I{}", self.platform_include_directory().display()))
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        bindings
            .write_to_file(self.output_directory().join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }

    fn link_libraries(&self);

    fn print_directories(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entry(&"is_compiled".to_string(), &self.is_compiled())
            .entry(
                &"output_directory".to_string(),
                &self.output_directory().display(),
            )
            .entry(&"vm_binary".to_string(), &self.vm_binary().display())
            .entry(
                &"vm_sources_directory".to_string(),
                &self.vm_sources_directory().display(),
            )
            .finish()
    }
}
