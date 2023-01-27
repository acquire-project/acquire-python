fn main() {
    let dst = cmake::Config::new("cpx")
        .target("cpx")
        .profile("RelWithDebInfo")
        .static_crt(true)
        .define("NO_UNIT_TESTS", "TRUE")
        .define("NO_EXAMPLES", "TRUE")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=cpx");

    #[cfg(target_os = "windows")]
    {
        // FIXME: hardcoded path to daqmx and blosc libs. Ideally these would be plugins and
        //        we'd be building them separately.  This is a fine hack till then.
        println!(
            "cargo:rustc-link-search=native=cpx/src/devices/signals/3rdParty/nidaqmx/lib64/msvc/"
        );
        println!("cargo:rustc-link-lib=static=NIDAQmx");

        println!(
            "cargo:rustc-link-search=native=cpx/src/devices/storage/3rdParty/c-blosc/lib/win64/"
        );
        println!("cargo:rustc-link-lib=static=libblosc");

        // Copy dcam lib
        std::fs::copy(
            format!("{}/lib/dcam_plugin.dll", dst.display()),
            "python/calliphlox/dcam_plugin.dll",
        )
        .expect("Failed to copy dcam_plugin.dll to python folder.");
    }

    println!("cargo:rerun-if-changed=wrapper.h");
    // TODO: expand rerun-if-changed so we don't have to touch wrapper so much
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
