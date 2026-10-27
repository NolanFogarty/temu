use libc::posix_openpt;

pub fn test() -> i32 {
    unsafe { posix_openpt(0) }
}
