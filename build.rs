use dotenv::dotenv;
use serde::Deserialize;
use std::env;
use std::io::{prelude::*, Cursor};

#[derive(Deserialize, Clone)]
struct WorkFlowRun {
    id: i128,
    repository_id: i128,
    head_repository_id: i128,
    head_branch: String,
    head_sha: String,
}

#[derive(Deserialize, Clone)]
struct Artifact {
    id: i128,
    node_id: String,
    name: String,
    size_in_bytes: i64,
    url: String,
    archive_download_url: String,
    expired: bool,
    created_at: String,
    updated_at: String,
    expires_at: String,
    workflow_run: WorkFlowRun,
}

#[derive(Deserialize, Clone)]
struct ArtifactsResponse {
    total_count: u32,
    artifacts: Vec<Artifact>,
}

fn main() {
    dotenv().ok();

    let dst = dbg!(cmake::Config::new("acquire-video-runtime")
        .target("acquire-video-runtime")
        .profile("RelWithDebInfo")
        .static_crt(true)
        .define("NOTEST", "TRUE")
        .define("NO_UNIT_TESTS", "TRUE")
        .define("NO_EXAMPLES", "TRUE")
        .define("CMAKE_OSX_DEPLOYMENT_TARGET", "10.15")
        .build());

    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    fetch_acquire_driver(&out, "acquire-driver-common", "d2560d0bd828cc75dd60e8a272fdf74905bc85f0");
    fetch_acquire_driver(&out, "acquire-driver-zarr", "d42c05930860947c987f297e1ca7c5561b5eae56");
    fetch_acquire_driver(&out, "acquire-driver-egrabber", "3ce238dca245de1a7578a73583fbeb7b753978ea");
    fetch_acquire_driver(&out, "acquire-driver-hdcam", "3a0c279b1328c5aa602386b08d5b438383fb1567");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=acquire-video-runtime");
    println!("cargo:rustc-link-lib=static=acquire-device-properties");
    println!("cargo:rustc-link-lib=static=acquire-device-hal");
    println!("cargo:rustc-link-lib=static=acquire-core-platform");
    println!("cargo:rustc-link-lib=static=acquire-core-logger");
    // println!("cargo:rustc-link-lib=static=stdc++");

    println!("cargo:rerun-if-changed=wrapper.h");
    // TODO: expand rerun-if-changed so we don't have to touch wrapper so much
    //       This involves better include isolation so only acquire.h needs to
    //       be watched.
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}/include", dst.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Failed to write bindings.");
}

fn fetch_acquire_driver(dst: &std::path::PathBuf, name: &str, sha: &str) {
    let build = if cfg!(target_os = "windows") {
        if name.ends_with("egrabber") || name.ends_with("hdcam") {
            "Release binaries"
        } else {
            "windows-latest Release binaries"
        }
    } else if cfg!(target_os = "macos") {
        "macos-latest Release binaries"
    } else if cfg!(target_os = "linux") {
        "ubuntu-latest Release binaries"
    } else {
        panic!("Unknown target os")
    };

    // list out build artifacts for this driver
    let uri = format!("https://api.github.com/repos/acquire-project/{name}/actions/artifacts");

    let token = env::var("GH_TOKEN").unwrap();
    let client = reqwest::blocking::Client::builder()
        .user_agent("acquire-project/builder")
        .build()
        .unwrap();

    let request = client
        .get(uri)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Authorization", format!("Bearer {token}"));

    let response = match request.send() {
        Ok(r) => r,
        Err(err) => panic!("HTTP request for {} failed, got {}", name, err),
    };

    let artifacts = response.json::<ArtifactsResponse>().unwrap().artifacts;

    let artifact = artifacts
        .iter()
        .filter(|a| a.workflow_run.head_sha == sha)
        .find(|a| a.name == build)
        .expect(
            format!(
                "Could not find an artifact with sha {} and name '{}' for driver {}",
                sha, build, name
            )
                .as_str(),
        )
        .to_owned();

    let artifact_name = artifact.name;
    let request = client
        .get(artifact.archive_download_url)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Authorization", format!("Bearer {token}"));

    let archive = match request.send() {
        Ok(r) => r.bytes().unwrap(),
        Err(err) => panic!("HTTP request for {} failed, got {}", artifact_name, err),
    };

    let target_dir = std::path::PathBuf::from(dst.join(&artifact_name));
    zip_extract::extract(Cursor::new(archive), &target_dir, true).unwrap();

    let dir_iterator = std::fs::read_dir(&target_dir).unwrap();
    let dir_entry = dir_iterator
        .last()
        .unwrap()
        .expect(format!("No entries in {}", &target_dir.as_path().to_str().unwrap()).as_str());
    let inner_path = target_dir.join(dir_entry.file_name());

    let mut inner_file = std::fs::File::open(&inner_path).unwrap();

    let mut b = vec![];
    inner_file.read_to_end(&mut b).unwrap();
    dbg!(zip_extract::extract(Cursor::new(b), &dst, true).unwrap());

    copy_acquire_driver(&dst, name);
}

fn copy_acquire_driver(dst: &std::path::PathBuf, name: &str) {
    let (prefix, postfix) = if cfg!(target_os = "windows") {
        ("", ".dll")
    } else if cfg!(target_os = "macos") {
        ("lib", ".so")
    } else if cfg!(target_os = "linux") {
        ("lib", ".so")
    } else {
        panic!("Unknown target os")
    };

    let lib = format!("{prefix}{name}{postfix}");

    std::fs::copy(
        format!("{}/lib/{lib}", dst.display()),
        format!("python/acquire/{lib}"),
    )
    .expect(&format!("Failed to copy {}/lib/{lib} to python folder.", dst.display()));
}
