use cmake::Config;

fn main() {
    let dst = Config::new("deps/td")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("OPENSSL_ROOT_DIR", "/opt/homebrew/opt/openssl/")
        .define("CMAKE_BUILD_PARALLEL_LEVEL", "8")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=tdjson_static");
}
