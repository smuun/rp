use std::io;
use std::io::Write;
// use rug::{Float, Assign};

#[allow(dead_code)]
enum LineType {
    //arithmetic
    Plus,
    Minus,
    Times,
    Divide,
    //entry
    Number,
    //stack ops
    Pop,
    //viewing
    Print,
    Full,
    Clear,
    Quit
}

struct Line {
    t: LineType,
    v: f64,
}

fn try_operator(value: &char) -> Option<LineType> {
    let t: Option<LineType> = match value {
        '+' => Some(LineType::Plus),
        '-' => Some(LineType::Minus),
        '*' => Some(LineType::Times),
        '/' => Some(LineType::Divide),
        'd' => Some(LineType::Pop),
        'p' => Some(LineType::Print),
        'f' => Some(LineType::Full),
        'c' => Some(LineType::Clear),
        'q' => Some(LineType::Quit),
        _ => {
            //println!("Failed to parse operator");
            return None},
    };
    return t
}

fn process(buff: &String) -> Option<Line> {
    let parsed = buff.trim().parse::<f64>();

    return match parsed {
        Ok(val) => Some( Line {
            t: LineType::Number,
            v: val,
        }),
        Err(_) => {
                //println!("Failed to parse float: buffer contains {}", *buff);
                return match buff.trim().parse::<char>() {
            Ok(val) => match try_operator(&val) {
                Some(t) => Some(Line {t: t, v: 0.0}),
                _ => None,
            },
            Err(_) => None,
        }
        },
    };
}

fn main() {
    println!("rp: command line rpn calculator");
    println!(
        "USAGE:
        <number>: add to stack,
        <operator> (+, -, *, /): pop top two elements, operate, and push
        p: print top of stack
        f: print stack
        d: drop (pop) element
        c: clear stack
        q or <C-d>: quit
        "
    );
    let mut buff = String::new();
    let mut stack: Vec<f64> = Vec::new();
    loop {
        buff.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buff).expect("readline failed");
        let val = process(&buff);
        match val {
            Some(i) => match i.t {
                LineType::Number => stack.push(i.v),
                LineType::Print => println!("{}", stack.last().unwrap()),
                LineType::Full => {for s in &stack { println!("{}", s); }},
                LineType::Pop => println!("{}", stack.pop().unwrap()),
                LineType::Plus => println!("plus"),
                LineType::Clear => {stack = Vec::new()},
                LineType::Plus => {
                    let a = stack.last().unwrap();
                    let b = stack.last().unwrap();
                    let r = a + b;
                    println!("{}", r);
                    stack.push(r);
                },
                LineType::Quit => std::process::exit(0),
                _ => {println!("?"); continue}
            },
            None => {println!("?"); continue}
        }
    }
}
