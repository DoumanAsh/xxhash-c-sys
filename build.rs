fn main() {
    let mut cc = cc::Build::new();

    cc.include("src/c/");
    cc.file("src/c/xxhash.c");
    cc.warnings(false);

    cc.compile("xxhash");
}
