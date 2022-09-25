use std::io;
// for later
// use rug::{Float, Assign};

enum LineType {
    Plus,
    Minus,
    Times,
    Divide,
    Number,
    Invalid,
}

struct Line {
    t: LineType,
    v: f64,
}

fn try_operator(value: &str) -> Option<Line> {
    let t: Option<LineType> = match value {
        "+" => Some(LineType::Plus),
        _ => None,
    };
    match t {
        return 
    }
    return Some(Line { t: t, v: 0.00 });
}

fn process(buff: String) -> Option<Line> {
    let parsed = buff.parse::<f64>();
    return match parsed {
        Ok(val) => Some(Line {
            t: LineType::Number,
            v: val,
        }),
        Err(val) => match buff.parse::<char>() {
            Ok(val) => try_operator(val),
            Err(val) => None(),
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
        f: print whole stack
        q or <C-d>: quit
        "
    );
    let mut buff = String::new();
    let mut stack: Vec<f64> = Vec::new();
    loop {
        buff.clear();
        io::stdin().read_line(&mut buff).expect("readline failed");
        let val = process(buff);
    }
}
