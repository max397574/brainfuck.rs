fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Input some brainfuck code as argument");
        return;
    }
    let bf_code = &args[1];
    let mut mem: [u8; 100] = [0; 100];
    let mut ip: usize = 0;
    let mut i: usize = 0;
    while i < bf_code.len() {
        match &bf_code[i..i + 1] {
            "+" => mem[ip] += 1,
            "-" => mem[ip] -= 1,
            ">" => {
                if ip != 99 {
                    ip += 1
                }
            }
            "<" => {
                if ip != 0 {
                    ip -= 1
                }
            }
            "." => {
                println!("{}", mem[ip] as char)
            }
            "[" => {
                if mem[ip] == 0 {
                    while &bf_code[i..i + 1] != "]" {
                        i += 1
                    }
                }
            }
            "]" => {
                if mem[ip] != 0 {
                    while &bf_code[i..i + 1] != "[" {
                        i -= 1
                    }
                }
            }
            _ => {}
        }
        i += 1
    }
}
