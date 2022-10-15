fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Input some brainfuck code as argument");
        return;
    }
    let instructions: Vec<char> = (&args[1]).chars().collect();
    let mut mem: [u8; 100] = [0; 100];
    let mut ip: usize = 0;
    let mut i: usize = 0;
    let mut open_brackets: u8;
    while i < instructions.len() {
        match instructions[i] {
            '+' => mem[ip] += 1,
            '-' => {
                if mem[ip] > 0 {
                    mem[ip] -= 1
                }
            }
            '>' => {
                if ip != 99 {
                    ip += 1;
                }
            }
            '<' => {
                if ip != 0 {
                    ip -= 1;
                }
            }
            '.' => {
                println!("{}", mem[ip] as char);
            }
            ',' => {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                mem[ip] = *input.as_bytes().first().expect("no byte read");
            }
            '[' => {
                if mem[ip] == 0 {
                    while instructions[i] != ']' {
                        i += 1;
                    }
                }
            }
            ']' => {
                if mem[ip] != 0 {
                    open_brackets = 1;
                    while open_brackets > 0 {
                        if instructions[i] == '[' {
                            open_brackets -= 1;
                        }
                        i -= 1;
                        if instructions[i] == ']' {
                            open_brackets += 1;
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1
    }
}
