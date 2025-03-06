use std::{fmt::Debug, vec};

use rustsat::{
    instances::Cnf,
    types::{Clause, Lit, TernaryVal},
};

use crate::pla::PLA;

struct Lines {
    inputs: Vec<TernaryVal>,
    outputs: Vec<TernaryVal>,
}
pub struct PlaBinary(Vec<Lines>);

impl Default for PlaBinary {
    fn default() -> Self {
        PlaBinary(Vec::new())
    }
}

impl PlaBinary {
    pub fn add_line(&mut self, inputs: Vec<TernaryVal>, outputs: Vec<TernaryVal>) {
        self.0.push(Lines { inputs, outputs });
    }
}

impl PLA for PlaBinary {}

impl Debug for PlaBinary {
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

impl From<String> for PlaBinary {
    fn from(value: String) -> Self {
        let mut result = Vec::new();

        let lines: Vec<&str> = value.lines().collect();

        for l in lines {
            if l.starts_with(".e") {
                break;
            }
            if l.starts_with(".") {
                continue;
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

        PlaBinary(result)
    }
}

impl From<&PlaBinary> for String {
    fn from(pla: &PlaBinary) -> Self {
        //implement function to convert a tuple of two vectors into anappropriate PLA string representation
        let mut result = String::new();
        result.push_str(".i ");
        result.push_str(&(pla.0[0].inputs.len().to_string()));
        result.push_str("\n.o ");
        result.push_str(&(pla.0[0].outputs.len().to_string()));
        result.push_str("\n.type f\n");
        for f in &pla.0 {
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

impl ToString for PlaBinary {
    fn to_string(&self) -> String {
        String::from(self)
    }
}

impl PlaBinary {
    pub fn from_cnf(cnf: Cnf, max_id: u32) -> Self {
        let mut pla = PlaBinary::default();

        for clause in cnf {
            let mut inputs = vec![TernaryVal::DontCare; max_id as usize];

            for i in clause {
                // Inverted because the CNF is optimized invertedly to form a DNF
                // Therefor the literal is inverted according to DeMorgan's Law

                inputs[i.vidx()] = TernaryVal::from(i.is_neg());
            }

            pla.add_line(inputs, vec![TernaryVal::True]);
        }

        pla
    }

    pub fn to_cnf(&self) -> Cnf {
        let mut cnf = Cnf::new();

        for line in &self.0 {
            let mut clause = Clause::new();

            for (i, val) in line.inputs.iter().enumerate() {
                match val {
                    TernaryVal::True => clause.add(Lit::negative(i as u32)),
                    TernaryVal::False => clause.add(Lit::positive(i as u32)),
                    _ => {}
                }
            }

            cnf.add_clause(clause);
        }

        cnf
    }
}

#[cfg(test)]
mod test {
    use rustsat::{clause, instances::Cnf, lit};

    use crate::espresso_cnf;

    #[test]
    fn cnf() {
        let mut cnf = Cnf::new();
        cnf.add_clause(clause!(lit![1], lit![2]));
        cnf.add_clause(clause!(lit![1], lit![3]));
        cnf.add_clause(clause!(lit![1], lit![4]));
        cnf.add_clause(clause!(lit![1], lit![5]));

        let opt = espresso_cnf(cnf, 6);

        println!("{:?}", opt);
    }
}
