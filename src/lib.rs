use std::ffi::CStr;
use std::os::raw::{c_void, c_char, c_uint};
use std::mem::MaybeUninit;

extern "C" {
    fn run_espresso_from_data(data: *const c_char, l: c_uint, out: *mut *mut c_char);
    fn run_d1merge_from_data(data: *const c_char, l: c_uint, out: *mut *mut c_char);
    fn free(p: *mut c_void);
}

pub fn espresso(lines: &str) -> String {
    let bytes = lines.as_bytes();
    let mut buf = MaybeUninit::<*mut c_char>::uninit();
    unsafe { run_espresso_from_data(bytes.as_ptr() as *const c_char, bytes.len() as u32, buf.as_mut_ptr()); }
    let c_str = Box::new(unsafe { buf.assume_init() });
    let result = unsafe {CStr::from_ptr(*c_str).to_str().unwrap().to_owned() };
    unsafe { free(*c_str as *mut c_void) };
    result
}

pub fn d1merge(lines: &str) -> String {
    let bytes = lines.as_bytes();
    let mut buf = MaybeUninit::<*mut c_char>::uninit();
    unsafe { run_d1merge_from_data(bytes.as_ptr() as *const c_char, bytes.len() as u32, buf.as_mut_ptr()); }
    let c_str = Box::new(unsafe { buf.assume_init() });
    let result = unsafe {CStr::from_ptr(*c_str).to_str().unwrap().to_owned() };
    unsafe { free(*c_str as *mut c_void) };
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_espresso_nothing_do_be_done() {
    let input = r#".i 128
.o 1
.type f
----0-1-0----------------------------------------------------------------------------------------------------------------------- 1
.e
"#;

        let expected = r#".i 128
.o 1
.p 1
----0-1-0----------------------------------------------------------------------------------------------------------------------- 1
.e
"#;

        let result = espresso(input);
        assert_eq!(result, expected);
    }



    #[test]
    #[serial]
    fn test_espresso() {
        let input = r#".i 4
.o 1
.type f
0000 1
0001 1
0101 1
0111 1
.e
"#;

        let expected1 = r#".i 4
.o 1
.p 2
000- 1
01-1 1
.e
"#;
        let expected2 = r#".i 4
.o 1
.p 2
01-1 1
000- 1
.e
"#;
        let result = espresso(input);
        assert!(result == expected1 || result == expected2);
    }

    #[test]
    #[serial]
    fn test_d1merge() {
        let input = r#".i 4
.o 1
.type f
0000 1
0001 1
0101 1
0111 1
.e
"#;
        let expected = r#".i 4
.o 1
.p 3
0111 1
0-01 1
0000 1
.e
"#;
        let result = d1merge(input);
        assert_eq!(result, expected);
    }
}
