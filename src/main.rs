use std::{env, fs, char};


#[derive(Debug)]
pub struct Interpreter {
    mem: Vec<i32>,
    inst_pointer: usize, //instruction pointer
    mem_pointer: usize, //memory pointer
    add_stack: Vec<usize>, //address stack
    program: String,
    input: String,
    output:String
}

impl Interpreter {
    
    fn interpret(&mut self) {
        let eof = false;

        let tokens = self.tokenize();

        while !eof {
            if self.inst_pointer >= tokens.len() {
                break;
            }

            match tokens[self.inst_pointer] {
                '>' => { self.mem_pointer += 1; }
                '<' => { 
                    if self.mem_pointer > 0 {
                        self.mem_pointer -= 1;
                    }
                }
                '+' => {
                    self.mem[self.mem_pointer] += 1;
                }
                '-' => {
                    self.mem[self.mem_pointer] -= 1;
                }
                '.' => {
                    self.output.push((self.mem[self.mem_pointer] as u8) as char);
                }
                ',' => {
                    //TODO: implement input
                }
                '[' => {
                    if self.mem[self.mem_pointer] != 0  {
                        self.add_stack.push(self.inst_pointer)
                    } else {
                        let mut count = 1;
                        while count > 0 {
                            self.inst_pointer += 1;
                            if tokens[self.inst_pointer] == '[' {
                                count += 1;
                            } else if tokens[self.inst_pointer] == ']' {
                                count -= 1;
                            }
                        }
                    }
                }
                ']' => {
                    if self.add_stack.len() > 0 {
                        self.inst_pointer = self.add_stack.pop().unwrap() - 1;
                    }
                }
                _ => {}
            }

            self.inst_pointer += 1;
        }

        if self.output.len() > 0 {
            println!("{}", self.output);
        }

    }

    fn tokenize(&mut self) -> Vec<char> {
        let mut tokens: Vec<char> = Vec::new();
        for ch in self.program.chars() {
            if self.is_valid_token(ch) {
                tokens.push(ch)
            }
        }
        return tokens;
    }

    fn is_valid_token(&self, ch:char) -> bool {
        return ch == '>' || ch == '<' || ch == '+' || ch == '-' || ch == '.' || ch == ',' || ch == '[' || ch == ']';
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut contents = String::new();

    match args.len() {
        1 => {
            println!("No file provided")
        }
        _ => {
            let path = &args[1];
            contents = fs::read_to_string(path).expect("Should have been able to find and read provided file");
        }
    }

    let mut interpreter = Interpreter{
        mem: vec![0; 100],
        inst_pointer: 0,
        mem_pointer: 0,
        add_stack: Vec::new(),
        program: contents,
        input: String::new(),
        output: String::new(),
    };

    let x = interpreter.mem[0];
    interpreter.interpret();

}
