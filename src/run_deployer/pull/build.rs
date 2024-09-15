// Parse services' commands to build the project

use std::io;
#[allow(unused_imports)]
use std::process::Command;

use crate::generate_conf::file_struct::Service;

pub fn build(_services: &[Service]) -> io::Result<()> {
    Ok(())
}
