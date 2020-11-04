use std::fs;
use std::env;
use std::path::Path;
use std::process::{Stdio, Command};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dst = Path::new(&out_dir);

    println!("cargo:rustc-link-search=native=./.termbox/build/libtermbox.a");
    println!("cargo:rustc-link-lib=static=termbox");
    println!("cargo:rustc-flags=-L ./.termbox/build/");

    setup();
    configure();
    build();
    install(&dst);
}

fn setup() {
    if Path::new(".termbox").exists() {
        println!(".termbox exists");
        return;
    } 
    let mut cmd = Command::new("git");
    cmd.arg("clone");
    cmd.arg("https://github.com/crnkofe/termbox");
    cmd.arg(".termbox");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(&manifest_dir);
    cmd.current_dir(&cargo_dir);

    run(&mut cmd);
}

fn clean() {
    let mut cmd = Command::new("rm");
    cmd.arg("-rf");
    cmd.arg(".termbox");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(&manifest_dir);
    cmd.current_dir(&cargo_dir);
    run(&mut cmd);
}

fn configure() {
    let mut cmd = new_cmd("ls");
    // cmd.arg("--prefix=/");

    let target = env::var("TARGET").unwrap();
    let mut cflags;
    if target.contains("i686") {
        cflags = "-m32"
    } else if target.contains("x86_64") {
        cflags = "-m64 -fPIC"
    } else {
        cflags = "-fPIC"
    }
    println!("new_cmd configure: setting CFLAGS to: `{}`", cflags);
    env::set_var("CFLAGS", cflags);

    run(&mut cmd);
    env::remove_var("CFLAGS");
}

fn build() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(&manifest_dir);
    let termbox_dir = cargo_dir.join(".termbox");

    let mut mkdir_build_cmd = new_cmd("mkdir");
    mkdir_build_cmd.arg("-p");
    mkdir_build_cmd.arg("build");
    mkdir_build_cmd.current_dir(&termbox_dir);
    run(&mut mkdir_build_cmd);

    let mut build_cmd = Command::new("cmake"); 
    build_cmd.arg("..");
    build_cmd.current_dir(cargo_dir.join(".termbox/build"));
    run(&mut build_cmd);
}

fn install(dst: &Path) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(&manifest_dir);
    let termbox_dir = cargo_dir.join(".termbox/build");

    let mut cmd = new_cmd("make");
    cmd.current_dir(&termbox_dir);
    run(&mut cmd);
}

fn new_cmd(cmd: &str) -> Command {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(&manifest_dir);
    let termbox_dir = cargo_dir.join(".termbox");
    let mut cmd = Command::new(cmd);
    cmd.current_dir(&termbox_dir);
    cmd
}

fn run(cmd: &mut Command) {
    assert!(cmd.stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .unwrap()
                .success());
}
