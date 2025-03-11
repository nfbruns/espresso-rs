use std::{
    ffi::{c_char, CStr},
    mem::MaybeUninit,
    os::raw::c_void,
};

use itemizer::{Item, Itemizer};
use ndarray::{Array2, AssignElem, Axis};
use std::fmt::Write;

use crate::{free, run_espresso_from_data};

///
pub fn espresso_multi(
    matrix: &Array2<Option<Vec<String>>>,
    variables: &Vec<Itemizer<String>>,
) -> Array2<Option<Vec<String>>> {
    let mut pla_string = String::new();

    writeln!(
        pla_string,
        ".mv {} 0 {}",
        matrix.dim().1,
        variables
            .iter()
            .map(|x| x.len().to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
    .unwrap();

    writeln!(pla_string, ".p {}", matrix.dim().0).unwrap();
    writeln!(pla_string, ".type f").unwrap();

    for row in matrix.axis_iter(Axis(0)) {
        for (i, var) in row.iter().zip(variables) {
            if let Some(i) = i {
                let mut s = std::iter::repeat("0")
                    .take(var.len() - 1)
                    .collect::<String>();
                for x in i {
                    let id = var.id_of_exists(x).unwrap().as_index() - 1;

                    s.insert(id, '1');
                }
                write!(pla_string, "{}", s).unwrap()
            } else {
                for _ in 0..var.len() {
                    write!(pla_string, "1").unwrap()
                }
            }
            write!(pla_string, "|").unwrap();
        }

        writeln!(pla_string, "").unwrap();
    }

    writeln!(pla_string, ".e").unwrap();

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

    let mut array = MaybeUninit::<Array2<Option<Vec<String>>>>::zeroed();
    let mut row_position = 0;

    for l in result.lines() {
        if l.starts_with(".e") {
            break;
        }
        if l.starts_with(".p") {
            let len = l[3..l.len()].parse::<usize>().unwrap();

            array.assign_elem(Array2::default((len, variables.len())));
            continue;
        }
        if l.starts_with(".") {
            continue;
        }

        let array = unsafe { array.assume_init_mut() };

        for (j, var) in l.split(' ').skip(1).enumerate() {
            if j >= variables.len() {
                break;
            }
            if matrix.dim().1 - 1 != j && var.chars().all(|x| x == '1') {
                continue;
            }
            for (p, c) in var.chars().enumerate() {
                if c == '1' {
                    let val = variables[j]
                        .value_of(&Item::with_id((p + 1) as u32))
                        .clone();

                    if let Some(a) = &mut array[[row_position, j]] {
                        a.push(val);
                    } else {
                        array[[row_position, j]] = Some(vec![val]);
                    }
                }
            }
        }
        row_position += 1;
    }

    unsafe { array.assume_init() }
}

#[cfg(test)]
mod test {
    use itemizer::Itemizer;
    use ndarray::{arr2, Axis};

    use crate::multi_reduce::espresso_multi;

    #[test]
    fn test_espresso_compress() {
        let matrix = arr2(&[
            [
                Some(vec!["A".to_string()]),
                Some(vec!["X".to_string()]),
                Some(vec!["U".to_string()]),
            ],
            [
                Some(vec!["A".to_string()]),
                Some(vec!["Y".to_string()]),
                Some(vec!["U".to_string()]),
            ],
            [
                Some(vec!["B".to_string()]),
                Some(vec!["Y".to_string()]),
                Some(vec!["V".to_string()]),
            ],
        ]);

        let mut variables = Vec::<Itemizer<String>>::new();

        for column in matrix.axis_iter(Axis(1)) {
            let mut itemizer = Itemizer::new();
            for i in column {
                if let Some(i) = i {
                    for x in i {
                        itemizer.id_of(x);
                    }
                }
            }
            variables.push(itemizer);
        }

        let result = espresso_multi(&matrix, &variables);

        assert_eq!(
            result,
            arr2(&[
                [
                    Some(vec!["B".to_string()]),
                    Some(vec!["Y".to_string()]),
                    Some(vec!["V".to_string()]),
                ],
                [
                    Some(vec!["A".to_string()]),
                    None,
                    Some(vec!["U".to_string()]),
                ],
            ])
        );
    }
}
