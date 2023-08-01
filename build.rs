fn main() {
    cc::Build::new()
        .include("src/c/")
        .file("src/c/xxhash.c")
        .warnings(false)
        .opt_level(3)
        .compile("xxhash");
}
