use std::{error::Error, fmt::Display, io, mem::MaybeUninit, task::Wake};


#[derive(Debug)]
enum BrainFuckError {
    InvalidToken(String) 
}

impl Display for BrainFuckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrainFuckError::InvalidToken(msg) => {
                write!(f, "Invalid token error: {:?}", msg)
            }
        }
    }
}

impl Error for BrainFuckError {}

struct Interpeter {
    memory: Vec<u8>,
    instruction: usize,
    loop_stack: Vec<usize>,
    tokens: Vec<u8>,
    current: usize,
}

impl Interpeter {
    fn new(input: &str) -> Self {
        Interpeter { memory: vec![0; 30_000], tokens: input.as_bytes().to_vec(), current: 0, instruction: 0, loop_stack: Vec::new() }
    }

    pub fn run(&mut self) -> Result<(), BrainFuckError> {
        while let Some(token) = self.tokens.get(self.instruction) {
            match token {
                b'>' => {
                    self.instruction += 1;
                    self.current += 1;
                }

                b'<' => {
                    self.instruction += 1;
                    self.current -= 1; 
                }

                b'+' => {
                    self.instruction += 1;


                    self.memory[self.current] = self.memory[self.current].wrapping_add(1);
                }

                b'-' => {
                    self.instruction += 1;
                    self.memory[self.current] = self.memory[self.current].wrapping_sub(1);
                }

                b'[' => {
                    self.loop_stack.push(self.instruction + 1);
                    self.instruction += 1;
                }

                b'.' => {
                    self.instruction += 1;
                    print!("{}", self.memory[self.current] as char);
                }

                b',' => {
                    self.instruction += 1;
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf).unwrap();

                    let input = buf.chars().next().unwrap();
                    self.memory[self.current] = input as u8;

                }

                
                b'\n' => {
                    self.instruction += 1;
                }

                b' ' => {
                    self.instruction += 1;
                }

                b']' => {
                    match self.memory[self.current] {
                        0 => {
                            self.loop_stack.pop().unwrap();
                            self.instruction += 1;
                        }

                        _=> {
                            self.instruction = *self.loop_stack.last().unwrap();
                        }
                    }
                }

                _=> {
                    return Err(BrainFuckError::InvalidToken(token.to_string()));
                }
            }

     }


        println!("Mem: {:?}", &self.memory[..10]);
        Ok(())

        }

}


fn main() -> Result<(), BrainFuckError> {
    let mut interpeter = Interpeter::new(",[>>+>+<<<-]>>>[<<<+>>>-]>+<<[-----[>]>>[<<<+++>>>[-]]");
    


    interpeter.run()?; 

    Ok(())
}
