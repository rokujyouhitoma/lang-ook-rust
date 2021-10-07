use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, Read};

mod parser {
    use std::collections::HashMap;

    pub struct Parsed {
        pub tokens: Vec<String>,
        pub bracket_map: HashMap<u64, u64>,
    }
}

struct Tape {
    position: u64,
    thetape: Vec<u64>,
}

impl Tape {
    fn new() -> Self {
        Tape {
            position: 0,
            thetape: vec![0],
        }
    }

    fn get(&mut self) -> u64 {
        return self.thetape[self.position as usize];
    }

    fn set(&mut self, val: u64) {
        self.thetape[self.position as usize] = val;
    }

    fn inc(&mut self) {
        self.thetape[self.position as usize] += 1;
    }

    fn dec(&mut self) {
        if self.thetape[self.position as usize] > 1 {
            self.thetape[self.position as usize] -= 1;
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        if self.thetape.len() as u64 <= self.position {
            self.thetape.push(0);
        }
    }

    fn devance(&mut self) {
        if self.position > 1 {
            self.position -= 1;
        }
    }
}

fn mainloop(parsed: parser::Parsed) {
    let mut pc: u64 = 0;
    let mut tape = Tape::new();

    for token in parsed.tokens.iter() {
        if token.eq("Ook. Ook?") {
            tape.advance();
        } else if token.eq("Ook? Ook.") {
            tape.devance();
        } else if token.eq("Ook. Ook.") {
            tape.inc();
        } else if token.eq("Ook! Ook!") {
            tape.dec();
        } else if token.eq("Ook! Ook.") {
            // print
            //println!("{}", tape.get());
            //println!("{}", (tape.get() as u8) as char);
            tape.get();
        } else if token.eq("Ook. Ook!") {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer);
            tape.set(buffer.parse::<u64>().unwrap());
        } else if token.eq("Ook! Ook?") && tape.get() == 0 {
            pc = parsed.bracket_map[&pc];
        } else if token.eq("Ook? Ook!") && tape.get() != 0 {
            pc = parsed.bracket_map[&pc];
        }
        pc += 1;
        //println!("{}", pc);
    }
}

fn split(program: String) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let fragments: Vec<String> = program.split(" ").map(|s| s.to_string()).collect();
    let length = fragments.len() / 2;
    for n in 0..length {
        let mut s = String::new();
        s.push_str(&fragments[n * 2]);
        s.push(' ');
        s.push_str(&fragments[n * 2 + 1]);
        tokens.push(s);
    }
    return tokens;
}

fn parse(program: String) -> parser::Parsed {
    let tokens = split(program);
    
    let mut parsed: Vec<String> = vec![];
    let mut bracket_map: HashMap<u64, u64> = HashMap::new();
    let mut leftstack: Vec<u64> = vec![];

    let mut pc: u64 = 0;

    let instructions: HashSet<&'static str> = [
        "Ook. Ook?",
        "Ook? Ook.",
        "Ook. Ook.",
        "Ook! Ook!",
        "Ook. Ook!",
        "Ook! Ook.",
        "Ook! Ook?",
        "Ook? Ook!",
    ]
    .iter()
    .cloned()
    .collect();

    for token in tokens.iter() {
        if instructions.contains(token.as_str()) {
            parsed.push(token.to_string());

            if token.eq("Ook! Ook?") {
                leftstack.push(pc);
            } else if token.eq("Ook? Ook!") {
                let left = match leftstack.pop() {
                    Some(number) => number,
                    None => 0,
                };
                let right = pc;
                bracket_map.insert(left, right);
                bracket_map.insert(right, left);
            }

            pc += 1;
        }
    }

    return parser::Parsed {
        tokens: parsed,
        bracket_map: bracket_map,
    };
}

fn run(mut file: &File) {
    let mut contents = String::new();
    let res = file.read_to_string(&mut contents);
    match res {
        Err(e) => println!("{:?}", e),
        _ => (),
    }
    let parsed = parse(contents);
    mainloop(parsed);
}

fn entry_point(args: Vec<String>) -> std::io::Result<()> {
    if args.len() < 2 {
        println!("You must supply a filename");
        std::process::exit(1);
    }
    let filename: &String = &args[1];
    println!("{}", filename);
    let file = File::open(filename)?;
    run(&file);
    std::process::exit(0);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let res = entry_point(args);
    match res {
        Err(e) => println!("{:?}", e),
        _ => (),
    }
}
