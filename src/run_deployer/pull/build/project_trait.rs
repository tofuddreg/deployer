use std::process::{Command, ExitStatus};
use std::io::Result;

trait Project {
    fn new() -> Self;
    fn build(&self, current_dir: &str) -> Result<ExitStatus>;
    fn get_build_dir(&self) -> &str;
}

// RUST
pub struct Rust<'a> {
    build_dir: &'a str,
}

impl<'a> Project for Rust<'a> {
    fn new() -> Self {
        Rust { build_dir: "/target/release" }
    }

    fn build(&self, current_dir: &str) -> Result<ExitStatus> {
        let mut cmd = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(current_dir)
            .spawn()
            .expect("Failed to build Rust project");
        cmd.wait()
    }

    fn get_build_dir(&self) -> &str {
        self.build_dir
    }
}

// GO
pub struct Go<'a> {
    build_dir: &'a str,
}

impl<'a> Project for Go<'a> {
    fn new() -> Self {
        // it builds in root dir of a project
        Go { build_dir: "" }
    }

    fn build(&self, _current_dir: &str) -> Result<ExitStatus> {
        todo!()
    }

    fn get_build_dir(&self) -> &str {
        self.build_dir
    }
}

// GLEAM
pub struct Gleam<'a> {
    build_dir: &'a str,
}

impl<'a> Project for Gleam<'a> {
    fn new() -> Self {
        Gleam { build_dir: "/build/prod/erlang" }
    }

    fn build(&self, _current_dir: &str) -> Result<ExitStatus> {
        todo!()
    }

    fn get_build_dir(&self) -> &str {
        self.build_dir
    }
}
