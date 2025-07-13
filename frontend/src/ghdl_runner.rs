use std::process::{Command, Output};
use std::path::Path;

pub fn ghdl_analyze(file: &Path) -> std::io::Result<Output> {
    Command::new("ghdl")
        .args(["-a", file.to_str().unwrap()])
        .output()
}

pub fn ghdl_elaborate(entity: &str) -> std::io::Result<Output> {
    Command::new("ghdl")
        .args(["-e", entity])
        .output()
}

pub fn ghdl_run(entity: &str, vcd_output: Option<&Path>) -> std::io::Result<Output> {
    let mut cmd = Command::new("ghdl");
    cmd.arg("-r").arg(entity);
    if let Some(vcd) = vcd_output {
        cmd.arg("-vcd").arg(vcd);
    }
    cmd.output()
}