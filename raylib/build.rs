use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() {
    let root = workspace_dir();
    println!("cargo:warning=WORKSPACE={:?}", root);

    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR environment variable");
    println!("cargo:warning=CARGO_MANIFEST_DIR={:?}", manifest_dir);

    let profile = env::var("PROFILE").expect("PROFILE environment variable");
    println!("cargo:warning=PROFILE={:?}", profile);

    let cwd = env::current_dir().expect("current working directory");
    println!("cargo:warning=CWD={:?}", cwd);

    let lib_path = Path::new(&manifest_dir).join("external\\lib");
    println!("cargo:rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-lib=dylib=raylibdll");

    let input_path = lib_path.join("raylib.dll");
    let output_path = root.join("target").join(profile).join("raylib.dll");

    let res = fs::copy(input_path, output_path);
    println!("cargo:warning={:?}", res)
}

fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}
