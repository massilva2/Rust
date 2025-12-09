use std::process::Command;

fn main() {
    let status = Command::new("rustc")
        .arg("--version")
        .status()
        .expect("failed to run command");

    if status.succe() {
        println!("✔ Command succeeded");
    } else {
        println!("✘ Command failed");
    }
}
