use util::*;

pub mod a;
pub mod b;

#[derive(Debug, Clone, Copy)]
enum Operand {
    Literal(u8),
    Combo(u8),
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    code: u8,
    operand: Operand,
}

impl Instruction {
    pub fn new(code: u8, operand: u8) -> Self {
        let operand = match code {
            1 | 3 | 4 => Operand::Literal(operand),
            0 | 2 | 5 | 6 | 7 => match operand <= 3 {
                true => Operand::Literal(operand),
                false => Operand::Combo(operand),
            },
            _ => panic!(),
        };
        Self { code, operand }
    }
}

struct Computer {
    a: usize,
    b: usize,
    c: usize,
    instructions: Vec<u8>,
}

fn get_register(s: &str) -> usize {
    s.ssplit_once(": ").1.uinto()
}

impl FromStr for Computer {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (registers, instructions) = s.ssplit_once("\n\n");
        let mut lines = registers.lines();
        let program = instructions.ssplit_once(": ").1;

        Ok(Self {
            a: get_register(lines.unext()),
            b: get_register(lines.unext()),
            c: get_register(lines.unext()),
            instructions: program.split(',').map(u8::ufrom).collect_vec(),
        })
    }
}

impl Computer {
    fn operand_val(&self, operand: Operand) -> usize {
        match operand {
            Operand::Literal(v) => v.into(),
            Operand::Combo(v) => match v {
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => panic!(),
            },
        }
    }

    pub fn execute(&mut self) -> String {
        let mut ptr = 0;
        let mut outputs = Vec::new();
        while ptr < self.instructions.len() - 1 {
            let ins = Instruction::new(self.instructions[ptr], self.instructions[ptr + 1]);
            let op = self.operand_val(ins.operand);
            // println!("{} {op}  -- [{}, {}, {}]", ins.code, self.a, self.b, self.c);
            match ins.code {
                0 => self.a /= 2usize.pow(op as u32),
                1 => self.b ^= op,
                2 => self.b = op % 8,
                3 => {
                    if self.a != 0 {
                        ptr = op;
                        continue;
                    }
                }
                4 => self.b ^= self.c,
                5 => outputs.push((op % 8) as u8),
                6 => self.b = self.a / 2usize.pow(op as u32),
                7 => self.c = self.a / 2usize.pow(op as u32),
                _ => panic!(),
            }

            ptr += 2;
        }

        outputs.iter().copied().join(",")
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(
            run(read("src/y24/d17/sample.txt").unwrap()).unwrap(),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y24/d17/input.txt").unwrap()).unwrap(),
            "5,0,3,5,7,6,1,5,4"
        );
    }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn offical() {
        assert_eq!(
            run(read("src/y24/d17/input.txt").unwrap()).unwrap(),
            164516454365621
        );
    }
}
