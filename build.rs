use std::process::Command;

fn main() {

    Command::new("git")
            .arg("clone")
            .arg("https://github.com/pyros2097/rust-embed")
            .output()
            .expect("Couldn't clone :(");

    Command::new("cargo")
            .current_dir("rust-embed")
            .arg("build")
            .output()
            .expect("Couldn't build rust-embed :(");

    Command::new("rust-embed/target/debug/rust-embed")
            .arg("src/cows")
            .arg("src/assets.rs")
            .output()
            .expect("Couldn't build assets.rs file :(");
}
