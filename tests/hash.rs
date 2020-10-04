use xxhash_c_sys::XXH64;

#[test]
fn assert_version() {
    let version = unsafe {
        xxhash_c_sys::XXH_versionNumber()
    };

    //Should be always careful with updating version after all
    //#define XXH_VERSION_NUMBER  (XXH_VERSION_MAJOR *100*100 + XXH_VERSION_MINOR *100 + XXH_VERSION_RELEASE)
    assert_eq!(version, xxhash_c_sys::XXH_VERSION_NUMBER);
}

#[test]
fn should_work() {
    let data = "lolka";

    let result = unsafe {
        XXH64(data.as_ptr() as _, data.len() as _, 0)
    };

    assert_ne!(result, 0);
}
