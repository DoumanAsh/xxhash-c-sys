use xxhash_c_sys::{self as sys, XXH3_64bits, XXH64};
use core::mem;

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

#[test]
fn should_work3() {
    let data = "lolka";

    let result = unsafe {
        XXH3_64bits(data.as_ptr() as _, data.len() as _)
    };

    assert_ne!(result, 0);

    let mut state = mem::MaybeUninit::uninit();
    let stream_result = unsafe {
        sys::XXH3_64bits_reset(state.as_mut_ptr());
        sys::XXH3_64bits_update(state.as_mut_ptr(), data.as_ptr() as _, data.len() - 2);
        sys::XXH3_64bits_update(state.as_mut_ptr(), data.as_ptr().add(data.len() - 2) as _, 2);
        sys::XXH3_64bits_digest(state.as_ptr())
    };

    assert_eq!(stream_result, result);
}
