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

    pub struct TokenSet<'a> {
        pub advance: &'a String,      // ">", "Ook. Ook?"
        pub devance: &'a String,      // "<", "Ook? Ook."
        pub increment: &'a String,    // "+", "Ook. Ook."
        pub decrement: &'a String,    // "-", "Ook! Ook!"
        pub set: &'a String,          // ",", "Ook. Ook!"
        pub print: &'a String,        // ".", "Ook! Ook."
        pub jump_forward: &'a String, // "[", "Ook! Ook?"
        pub jump_back: &'a String,    // "]", "Ook? Ook!"
    }
}

struct Tape {
    position: i64,
    thetape: Vec<i64>,
}

impl Tape {
    fn new() -> Self {
        Tape {
            position: 0,
            thetape: vec![0],
        }
    }

    fn get(&mut self) -> i64 {
        return self.thetape[self.position as usize];
    }

    fn set(&mut self, val: i64) {
        self.thetape[self.position as usize] = val;
    }

    fn inc(&mut self) {
        self.thetape[self.position as usize] += 1;
    }

    fn dec(&mut self) {
        self.thetape[self.position as usize] -= 1;
    }

    fn advance(&mut self) {
        self.position += 1;
        if self.thetape.len() as i64 <= self.position {
            self.thetape.push(0);
        }
    }

    fn devance(&mut self) {
        self.position -= 1;
    }
}

fn mainloop(token_set: &parser::TokenSet, parsed: parser::Parsed) {
    let mut pc: u64 = 0;
    let mut tape = Tape::new();

    while pc < parsed.tokens.len() as u64 {
        let token = &parsed.tokens[pc as usize];

        if token.eq(token_set.advance) {
            tape.advance();
        } else if token.eq(token_set.devance) {
            tape.devance();
        } else if token.eq(token_set.increment) {
            tape.inc();
        } else if token.eq(token_set.decrement) {
            tape.dec();
        } else if token.eq(token_set.print) {
            // print
            print!("{}", (tape.get() as u8) as char);
        } else if token.eq(token_set.set) {
            let mut buffer = String::new();
            let _ = io::stdin().read_to_string(&mut buffer);
            tape.set(buffer.parse::<i64>().unwrap());
        } else if token.eq(token_set.jump_forward) && tape.get() == 0 {
            pc = parsed.bracket_map[&pc];
        } else if token.eq(token_set.jump_back) && tape.get() != 0 {
            pc = parsed.bracket_map[&pc];
        }
        pc += 1;
    }
}

fn split(program: String) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let fragments: Vec<String> = program.split(" ").map(|s| s.to_string()).collect();
    let length = fragments.len() / 2;
    for n in 0..length {
        let mut s = String::new();
        let base = n * 2;
        s.push_str(&fragments[base]);
        s.push(' ');
        s.push_str(&fragments[base + 1]);
        tokens.push(s);
    }
    return tokens;
}

fn parse(token_set: &parser::TokenSet, program: String) -> parser::Parsed {
    let tokens = split(program);

    let mut parsed: Vec<String> = vec![];
    let mut bracket_map: HashMap<u64, u64> = HashMap::new();
    let mut leftstack: Vec<u64> = vec![];

    let mut pc: u64 = 0;

    let instructions: HashSet<&str> = [
        token_set.advance.as_str(),
        token_set.devance.as_str(),
        token_set.increment.as_str(),
        token_set.decrement.as_str(),
        token_set.set.as_str(),
        token_set.print.as_str(),
        token_set.jump_forward.as_str(),
        token_set.jump_back.as_str(),
    ]
    .iter()
    .cloned()
    .collect();

    for token in tokens.iter() {
        if instructions.contains(token.as_str()) {
            parsed.push(token.to_string());

            if token.eq(token_set.jump_forward) {
                leftstack.push(pc);
            } else if token.eq(token_set.jump_back) {
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
    let token_set = parser::TokenSet {
        advance: &String::from("Ook. Ook?"),
        devance: &String::from("Ook? Ook."),
        increment: &String::from("Ook. Ook."),
        decrement: &String::from("Ook! Ook!"),
        set: &String::from("Ook. Ook!"),
        print: &String::from("Ook! Ook."),
        jump_forward: &String::from("Ook! Ook?"),
        jump_back: &String::from("Ook? Ook!"),
    };
    match res {
        Err(e) => println!("{:?}", e),
        _ => (),
    }
    let parsed = parse(&token_set, contents);
    mainloop(&token_set, parsed);
}

fn entry_point(args: Vec<String>) -> std::io::Result<()> {
    if args.len() < 2 {
        println!("You must supply a filename");
        std::process::exit(1);
    }
    let filename: &String = &args[1];
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
