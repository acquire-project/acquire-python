fn main() {
    let dst = cmake::Config::new("demo")
        .target("core")
        .profile("RelWithDebInfo")
        .static_crt(true)
        .define("NOTEST", "TRUE")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=core");

    #[cfg(target_os="windows")]
    {
        println!("cargo:rustc-link-search=native=demo/src/devices/signals/3rdParty/nidaqmx/lib64/msvc/");
        println!("cargo:rustc-link-lib=static=NIDAQmx");
    }

    println!("cargo:rerun-if-changed=wrapper.h"); 
    // TODO: expand rerun-if-changed so we don't have to touch wrapper so much
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}/include",dst.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Failed to write bindings.");
}
