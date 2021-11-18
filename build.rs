fn main() {
    let mut cfg = cc::Build::new();
    cfg.file("src/zuser.c");
    if let Some(include) = std::env::var_os("DEP_Z_INCLUDE") {
        cfg.include(include);
    }
    cfg.shared_flag(true);
    cfg.static_flag(false);
    cfg.compile("zuser");
    println!("cargo:rerun-if-changed=src/zuser.c");
    pkg_config::Config::new().probe("zlib").unwrap();
}