use std::{fmt::Debug, vec};

use rustsat::{instances::Cnf, types::TernaryVal};

struct Lines {
    inputs: Vec<TernaryVal>,
    outputs: Vec<TernaryVal>,
}
pub struct PLA(Vec<Lines>);

impl Default for PLA {
    fn default() -> Self {
        PLA(Vec::new())
    }
}

impl PLA {
    pub fn add_line(&mut self, inputs: Vec<TernaryVal>, outputs: Vec<TernaryVal>) {
        self.0.push(Lines { inputs, outputs });
    }
}

impl From<Cnf> for PLA {
    fn from(cnf: Cnf) -> Self {
        let mut pla = PLA::default();
        let max = 10;

        for clause in cnf {
            let mut inputs = vec![TernaryVal::DontCare; max];

            for i in clause {
                inputs[i.vidx()] = TernaryVal::from(i.is_pos());
            }

            pla.add_line(inputs, vec![TernaryVal::True]);
        }

        pla
    }
}

impl Debug for PLA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            for i in &line.inputs {
                write!(f, "{:?}", i)?;
            }
            write!(f, " ")?;
            for i in &line.outputs {
                write!(f, "{:?}", i)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl From<String> for PLA {
    fn from(value: String) -> Self {
        let mut result = Vec::new();

        let lines: Vec<&str> = value.lines().collect();

        for l in lines {
            if l.starts_with(".") {
                continue;
            }
            if l.starts_with(".e") {
                break;
            }
            let parts: Vec<&str> = l.split(' ').collect();
            let inputs_str = parts[0];
            let outputs_str = parts[1];
            let mut inputs: Vec<TernaryVal> = Vec::new();
            let mut outputs: Vec<TernaryVal> = Vec::new();
            for c in inputs_str.chars() {
                match c {
                    '1' => inputs.push(TernaryVal::True),
                    '0' => inputs.push(TernaryVal::False),
                    '-' => inputs.push(TernaryVal::DontCare),
                    _ => panic!("Invalid character in PLA file"),
                }
            }
            for c in outputs_str.chars() {
                match c {
                    '1' => outputs.push(TernaryVal::True),
                    '0' => outputs.push(TernaryVal::False),
                    '-' => outputs.push(TernaryVal::DontCare),
                    _ => panic!("Invalid character in PLA file"),
                }
            }
            result.push(Lines { inputs, outputs });
        }

        PLA(result)
    }
}

impl From<PLA> for String {
    fn from(pla: PLA) -> Self {
        //implement function to convert a tuple of two vectors into anappropriate PLA string representation
        let mut result = String::new();
        result.push_str(".i ");
        result.push_str(&(pla.0[0].inputs.len().to_string()));
        result.push_str("\n.o ");
        result.push_str(&(pla.0[0].outputs.len().to_string()));
        result.push_str("\n.type f\n");
        for f in pla.0 {
            for i in 0..f.inputs.len() {
                match f.inputs[i] {
                    TernaryVal::True => result.push('1'),
                    TernaryVal::False => result.push('0'),
                    TernaryVal::DontCare => result.push('-'),
                }
            }
            result.push(' ');
            for i in 0..f.outputs.len() {
                match f.outputs[i] {
                    TernaryVal::True => result.push('1'),
                    TernaryVal::False => result.push('0'),
                    TernaryVal::DontCare => result.push('-'),
                }
            }
            result.push('\n');
        }
        result.push_str(".e\n");
        result
    }
}
