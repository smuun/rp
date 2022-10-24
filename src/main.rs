use std::io;
use std::io::Write;
use std::env;
use colored::Colorize;
// use rug::{Float, Assign};

#[allow(dead_code)]
enum LineType {
    //arithmetic
    Plus,
    Minus,
    Times,
    Divide,
    Pow,
    //entry
    Number,
    Negate,
    //stack ops
    Pop,
    Clear,
    //viewing
    Print,
    Full,
    ClearScreen,
    Quit,
    Swap,
    Flip
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
        '_' => Some(LineType::Negate),
        '^' => Some(LineType::Pow),
        'd' => Some(LineType::Pop),
        'p' => Some(LineType::Print),
        'f' => Some(LineType::Full),
        'c' => Some(LineType::Clear),
        'q' => Some(LineType::Quit),
        's' => Some(LineType::Swap),
        'S' => Some(LineType::Flip),
        ' ' => Some(LineType::ClearScreen),
        _ => {
            return None;
        }
    };
    return t;
}

fn try_command(value: &str) -> Option<LineType> {
    let t: Option<LineType> = match value {
        "cl" => Some(LineType::ClearScreen),
        "quit" => Some(LineType::Quit),
        "exit" => Some(LineType::Quit),
        _ => {
            return None;
        }
    };
    return t;
}

fn process(buff: &String) -> Option<Line> {
    let parsed = buff.trim().parse::<f64>();

    return match parsed {
        Ok(val) => Some(Line {
            t: LineType::Number,
            v: val,
        }),
        Err(_) => {
            return match buff.trim().parse::<char>() {
                Ok(val) => match try_operator(&val) {
                    Some(t) => Some(Line { t: t, v: 0.0 }),
                    _ => None,
                },
                Err(_) => match buff.trim().parse::<String>() {
                    Ok(val) => match try_command(&val) {
                        Some(t) => Some(Line { t: t, v: 0.0 }),
                        _ => None,
                    }
                    Err(_) => None,
                },
            };
        }
    };
}

fn poptwo(v: &mut Vec<f64>) -> Option<(f64, f64)> {
    let a = v.pop().unwrap();
    let b = v.pop().unwrap();
    return Some((a, b))
}

fn printresult(r: &f64) {
    println!("{}", r.to_string().bold());
    println!("");
}

fn main() {
    println!("rp: command line rpn calculator");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1{
    println!(
        "USAGE:  rp
        COMMANDS:
        <number>:                        add to stack
        <operator> (+, -, *, /, ^):      operate on last two elements
        _                                negate last element
        clear, cl:                       clear screen
        p:                               print top of stack
        f:                               print whole stack
        d:                               drop last element
        c:                               clear stack
        s:                               flip last two elements
        S:                               flip stack
        q, quit, exit, <C-c>:            quit
        "
    );
    std::process::exit(1)
    }
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
                LineType::Print => println!("{}", &stack.last().unwrap()),
                LineType::Full => {
                    for s in &stack {
                        print!("{}\n", s);
                    }
                    print!("\n");
                    io::stdout().flush().unwrap();
                }
                LineType::Pop => printresult(&(stack.pop().unwrap())),
                LineType::Swap => {
                    let (a, b) = poptwo(&mut stack).unwrap();
                    stack.push(a);
                    stack.push(b);
                },
                LineType::Flip => {stack.reverse()},
                LineType::Clear => stack = Vec::new(),
                LineType::ClearScreen => print!("{esc}c", esc = 27 as char),
                LineType::Plus => {
                    let (a, b) = poptwo(&mut stack).unwrap();
                    let r = a + b;
                    printresult(&r);
                    stack.push(r);
                }
                LineType::Minus => {
                    let (a, b) = poptwo(&mut stack).unwrap();
                    let r = a - b;
                    printresult(&r);
                    stack.push(r);
                }
                LineType::Times => {
                    let (a, b) = poptwo(&mut stack).unwrap();
                    let r = a * b;
                    printresult(&r);
                    stack.push(r);
                }
                LineType::Divide => {
                    let (a, b) = poptwo(&mut stack).unwrap();
                    let r = b / a;
                    printresult(&r);
                    stack.push(r);
                }
                LineType::Pow => {
                    let (a, b) = poptwo(&mut stack).unwrap();
                    let r = f64::powf(b, a);
                    printresult(&r);
                    stack.push(r);
                }
                LineType::Negate => {
                    let a = stack.pop().unwrap();
                    let r = -a;
                    printresult(&r);
                    stack.push(r);
                }
                LineType::Quit => std::process::exit(0),
            },
            None => {
                println!("?");
                continue;
            }
        }
    }
}
