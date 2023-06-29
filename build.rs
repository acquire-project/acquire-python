use std::fs;
use serde::Deserialize;

/// Struct representation of the manifest file drivers.json
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct DriverManifest {
    acquire_driver_common: String,
    acquire_driver_zarr: String,
    acquire_driver_egrabber: String,
    acquire_driver_hdcam: String,
}

fn main() {
    let dst = dbg!(cmake::Config::new("acquire-video-runtime")
        .target("acquire-video-runtime")
        .profile("RelWithDebInfo")
        .static_crt(true)
        .define("NOTEST", "TRUE")
        .define("NO_UNIT_TESTS", "TRUE")
        .define("NO_EXAMPLES", "TRUE")
        .define("CMAKE_OSX_DEPLOYMENT_TARGET", "10.15")
        .build());

    let drivers_json = fs::read_to_string("drivers.json")
        .expect("Failed to read from drivers.json.");
    let tags: DriverManifest = serde_json::from_str(drivers_json.as_str())
        .expect("Failed to parse drivers.json");

    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    fetch_acquire_driver(
        &out,
        "acquire-driver-common",
        tags.acquire_driver_common.as_str(),
    );
    fetch_acquire_driver(
        &out,
        "acquire-driver-zarr",
        tags.acquire_driver_zarr.as_str(),
    );
    fetch_acquire_driver(
        &out,
        "acquire-driver-egrabber",
        tags.acquire_driver_egrabber.as_str(),
    );
    fetch_acquire_driver(
        &out,
        "acquire-driver-hdcam",
        tags.acquire_driver_hdcam.as_str(),
    );

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=acquire-video-runtime");
    println!("cargo:rustc-link-lib=static=acquire-device-properties");
    println!("cargo:rustc-link-lib=static=acquire-device-hal");
    println!("cargo:rustc-link-lib=static=acquire-core-platform");
    println!("cargo:rustc-link-lib=static=acquire-core-logger");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=static=stdc++");

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
    // let build = if cfg!(target_os = "windows") {
    //     "win64"
    // } else if cfg!(target_os = "macos") {
    //     "Darwin"
    // } else if cfg!(target_os = "linux") {
    //     "Linux"
    // } else {
    //     panic!("Unknown target os")
    // };
    //
    // let client = reqwest::blocking::Client::builder()
    //     .user_agent("acquire-project/builder")
    //     .build()
    //     .unwrap();
    //
    // let vstring = if tag == "nightly" {
    //     tag.to_owned()
    // } else {
    //     format!("v{tag}")
    // };
    // let uri = format!("https://github.com/acquire-project/{name}/releases/download/{vstring}/{name}-{vstring}-{build}.zip");
    // let request = client
    //     .get(uri)
    //     .header("Accept", "application/vnd.github+json")
    //     .header("X-GitHub-Api-Version", "2022-11-28");
    //
    // let archive = match request.send() {
    //     Ok(r) => r.bytes(),
    //     Err(err) => panic!("HTTP request for {} failed, got {}", &name, err),
    // }.expect(&*format!("Failed to get response body for {} as bytes.", name));
    //
    // zip_extract::extract(
    //     std::io::Cursor::new(archive), &dst, true,
    // ).expect(&*format!("Failed to extract {name}-{tag}-{build}.zip from response."));
    //
    // copy_acquire_driver(&dst, name);
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
