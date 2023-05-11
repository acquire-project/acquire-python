use serde::Deserialize;
use std::io::Cursor;

/// Component of the `Artifact` JSON object returned from the GitHub API.
#[allow(dead_code)]
#[derive(Deserialize, Clone)]
struct WorkFlowRun {
    id: i128,
    repository_id: i128,
    head_repository_id: i128,
    head_branch: String,
    head_sha: String,
}

/// Metadata for a build artifact from given commit to a given repository, returned from GitHub API.
#[allow(dead_code)]
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

/// List of build artifacts for a given repository, returned from GitHub API.
#[allow(dead_code)]
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
    fetch_acquire_driver(
        &out,
        "acquire-driver-common",
        "0.1.1",
    );
    fetch_acquire_driver(
        &out,
        "acquire-driver-zarr",
        "0.1.1",
    );
    fetch_acquire_driver(
        &out,
        "acquire-driver-egrabber",
        "0.1.1",
    );
    fetch_acquire_driver(
        &out,
        "acquire-driver-hdcam",
        "0.1.1",
    );

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

fn fetch_acquire_driver(dst: &std::path::PathBuf, name: &str, tag: &str) {
    let build = if cfg!(target_os = "windows") {
        "win64"
    } else if cfg!(target_os = "macos") {
        "Darwin"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else {
        panic!("Unknown target os")
    };

    let client = reqwest::blocking::Client::builder()
        .user_agent("acquire-project/builder")
        .build()
        .unwrap();

    let uri = format!("https://github.com/acquire-project/{name}/releases/download/v{tag}/{name}-{tag}-{build}.zip");
    let request = client
        .get(uri)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28");

    let archive = match request.send() {
        Ok(r) => r.bytes().unwrap(),
        Err(err) => panic!("HTTP request for {} failed, got {}", &name, err),
    };

    zip_extract::extract(Cursor::new(archive), &dst, true).unwrap();

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
    .expect(&format!(
        "Failed to copy {}/lib/{lib} to python folder.",
        dst.display()
    ));
}
