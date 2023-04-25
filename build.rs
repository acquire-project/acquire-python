fn main() {
    let dst = cmake::Config::new("acquire-video-runtime")
        .target("acquire-video-runtime")
        .profile("RelWithDebInfo")
        .static_crt(true)
        .define("NO_UNIT_TESTS", "TRUE")
        .define("NO_EXAMPLES", "TRUE")
        .define("CMAKE_OSX_DEPLOYMENT_TARGET", "10.15")
        .build();

    build_acquire_driver(&dst, "acquire-driver-common");
    build_acquire_driver(&dst, "acquire-driver-egrabber");
    build_acquire_driver(&dst, "acquire-driver-hdcam");
    build_acquire_driver(&dst, "acquire-driver-zarr");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=acquire-video-runtime");
    println!("cargo:rustc-link-lib=static=acquire-device-properties");
    println!("cargo:rustc-link-lib=static=acquire-device-hal");
    println!("cargo:rustc-link-lib=static=acquire-core-platform");
    println!("cargo:rustc-link-lib=static=acquire-core-logger");
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

    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Failed to write bindings.");
}

fn build_acquire_driver(dst: &std::path::PathBuf, name: &str) {
    cmake::Config::new(name)
        .target(name)
        .profile("RelWithDebInfo")
        .static_crt(true)
        .define("NO_UNIT_TESTS", "TRUE")
        .define("NO_EXAMPLES", "TRUE")
        .define("CMAKE_OSX_DEPLOYMENT_TARGET", "10.15")
        .build();
    copy_acquire_driver(dst, name);
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
    .expect(&format!("Failed to copy {lib} to python folder."));
}
