use std::io;
use std::process::{Command, Output};

fn main() {
    let _ = application_version_set(&lsb_release().unwrap());
    let _ = status_set("active", "Unit is ready.");
}

fn status_set(state: &str, message: &str) -> Result<Output, io::Error> {
    println!("Trying to set status: {}: {}", state, message);
    Command::new("status-set")
        .arg(state)
        .arg(message)
        .output()
}

fn application_version_set(version: &str) -> Result<Output, io::Error> {
    println!("Trying to set application version: {}", version);
    Command::new("application-version-set")
        .arg(version)
        .output()
}

fn lsb_release() -> Result<String, io::Error> {
    Command::new("/usr/bin/lsb_release")
    .arg("--short")
    .arg("-r")
    .output()
    .map(|o| String::from_utf8_lossy(&o.stdout).to_string() )
}