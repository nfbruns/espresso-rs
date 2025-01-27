use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_uint, c_void};

use rustsat::types::{Lit, TernaryVal};

pub type PLA_F = Vec<(Vec<TernaryVal>, Vec<TernaryVal>)>;

extern "C" {
    fn run_espresso_from_data(data: *const c_char, l: c_uint, out: *mut *mut c_char);
    fn run_d1merge_from_data(data: *const c_char, l: c_uint, out: *mut *mut c_char);
    fn free(p: *mut c_void);
}

fn espresso(lines: &str) -> String {
    let bytes = lines.as_bytes();
    let mut buf = MaybeUninit::<*mut c_char>::uninit();
    unsafe {
        run_espresso_from_data(
            bytes.as_ptr() as *const c_char,
            bytes.len() as u32,
            buf.as_mut_ptr(),
        );
    }
    let c_str = Box::new(unsafe { buf.assume_init() });
    let result = unsafe { CStr::from_ptr(*c_str).to_str().unwrap().to_owned() };
    unsafe { free(*c_str as *mut c_void) };
    result
}

fn d1merge(lines: &str) -> String {
    let bytes = lines.as_bytes();
    let mut buf = MaybeUninit::<*mut c_char>::uninit();
    unsafe {
        run_d1merge_from_data(
            bytes.as_ptr() as *const c_char,
            bytes.len() as u32,
            buf.as_mut_ptr(),
        );
    }
    let c_str = Box::new(unsafe { buf.assume_init() });
    let result = unsafe { CStr::from_ptr(*c_str).to_str().unwrap().to_owned() };
    unsafe { free(*c_str as *mut c_void) };
    result
}

fn create_pla(F: PLA_F) -> string {
    //implement function to convert a tuple of two vectors into anappropriate PLA string representation
    let mut result = String::new();
    result.push_str(".i ");
    result.push_str(&(F[0].0.len().to_string()));
    result.push_str("\n.o ");
    result.push_str(&(F[0].1.len().to_string()));
    result.push_str("\n.type f\n");
    for f in F {
        for i in 0..f.0.len() {
            match f.0[i] {
                TernaryVal::True => result.push('1'),
                TernaryVal::False => result.push('0'),
                TernaryVal::DontCare => result.push('-'),
            }
        }
        result.push(' ');
        for i in 0..f.1.len() {
            match f.1[i] {
                TernaryVal::True => result.push('1'),
                TernaryVal::False => result.push('0'),
                TernaryVal::DontCare => result.push('-'),
            }
        }
        result.push('\n');
    }
    result.push_str(".e");
    result
}

fn interpret_pla(pla: &str) -> PLA_F {
    let mut result: PLA_F = Vec::new();
    let lines: Vec<&str> = pla.lines().collect();
    let mut inputs = 0;
    let mut outputs = 0;
    let mut type_line_index = 0;
    for (i, l) in lines.iter().enumerate() {
        if l.starts_with(".i") {
            inputs = l[3..].parse().unwrap();
        } else if l.starts_with(".o") {
            outputs = l[3..].parse().unwrap();
        } else if l.starts_with(".type") {
            type_line_index = i;
        }
    }
    for l in lines.iter().skip(type_line_index + 1) {
        if l.starts_with(".e") {
            break;
        }
        let mut parts: Vec<&str> = l.split(' ').collect();
        let inputs_str = parts[0];
        let outputs_str = parts[1];
        let mut inputs_vec: Vec<TernaryVal> = Vec::new();
        let mut outputs_vec: Vec<TernaryVal> = Vec::new();
        for c in inputs_str.chars() {
            match c {
                '1' => inputs_vec.push(TernaryVal::True),
                '0' => inputs_vec.push(TernaryVal::False),
                '-' => inputs_vec.push(TernaryVal::DontCare),
                _ => panic!("Invalid character in PLA file"),
            }
        }
        for c in outputs_str.chars() {
            match c {
                '1' => outputs_vec.push(TernaryVal::True),
                '0' => outputs_vec.push(TernaryVal::False),
                '-' => outputs_vec.push(TernaryVal::DontCare),
                _ => panic!("Invalid character in PLA file"),
            }
        }
        result.push((inputs_vec, outputs_vec));
    }
    result
}

pub fn espresso(F: PLA_F) -> PLA_F {
    let pla_string = create_pla(F);
    let pla = espresso(&pla_string);
    interpret_pla(&pla)
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
