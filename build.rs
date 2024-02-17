fn main() {
    let mut cc = cc::Build::new();

    cc.include("src/c/");
    #[cfg(feature = "no_std")]
    cc.define("XXH_NO_STDLIB", None);

    cc.file("src/c/xxhash.c");
    cc.warnings(false);

    cc.compile("xxhash");
}
