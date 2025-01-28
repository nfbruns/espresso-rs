use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_uint, c_void};

use pla::PLA;
use rustsat::instances::Cnf;

mod pla;

extern "C" {
    fn run_espresso_from_data(data: *const c_char, l: c_uint, out: *mut *mut c_char);
    fn free(p: *mut c_void);
}

pub fn espresso(pla: PLA) -> PLA {
    let pla_string = String::from(pla);
    let bytes = pla_string.as_bytes();

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

    PLA::from(result)
}

pub fn espresso_cnf(cnf: Cnf, max_id: u32) -> Cnf {
    let pla = PLA::from_cnf(cnf, max_id);

    let result = espresso(pla);

    return result.to_cnf();
}

#[cfg(test)]
mod tests {
    use rustsat::types::TernaryVal;

    use crate::{espresso, pla::PLA};

    #[test]
    fn test_espresso() {
        let mut pla = PLA::default();

        pla.add_line(
            vec![
                TernaryVal::False,
                TernaryVal::False,
                TernaryVal::False,
                TernaryVal::False,
            ],
            vec![TernaryVal::True],
        );
        pla.add_line(
            vec![
                TernaryVal::False,
                TernaryVal::False,
                TernaryVal::False,
                TernaryVal::True,
            ],
            vec![TernaryVal::True],
        );
        pla.add_line(
            vec![
                TernaryVal::False,
                TernaryVal::True,
                TernaryVal::False,
                TernaryVal::True,
            ],
            vec![TernaryVal::True],
        );
        pla.add_line(
            vec![
                TernaryVal::False,
                TernaryVal::True,
                TernaryVal::True,
                TernaryVal::True,
            ],
            vec![TernaryVal::True],
        );

        let result = espresso(pla);

        println!("{:?}", result);
        //assert!(false);
    }

    #[test]
    fn test_espresso_2() {
        let mut pla = PLA::default();

        pla.add_line(
            vec![
                TernaryVal::DontCare,
                TernaryVal::False,
                TernaryVal::DontCare,
                TernaryVal::True,
            ],
            vec![TernaryVal::True],
        );
        pla.add_line(
            vec![
                TernaryVal::False,
                TernaryVal::False,
                TernaryVal::False,
                TernaryVal::DontCare,
            ],
            vec![TernaryVal::True],
        );

        let result = espresso(pla);

        println!("{:?}", result);
        assert!(false);
    }
}
