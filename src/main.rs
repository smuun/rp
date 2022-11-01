use colored::Colorize;
use std::env;
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
    Pow,
    //entry
    Number,
    Negate,
    //stack ops
    Pop,
    UndoPop,
    Clear,
    //viewing
    Print,
    Full,
    ClearScreen,
    Quit,
    Swap,
    Flip,
}

struct Line {
    line_type: LineType,
    value: f64,
    combined: bool,
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
        "ud" => Some(LineType::UndoPop),
        "quit" => Some(LineType::Quit),
        "exit" => Some(LineType::Quit),
        _ => {
            return None;
        }
    };
    return t;
}

fn try_combined(value: &str) -> Option<Line> {
    let mut cs = value.chars();
    if let Some(last ) = &(cs.next_back()) {
    let first: &str = cs.as_str();
    let operation: LineType = match try_operator(last) {
        Some(op) => op,
        None => return None,
    };
    let number = match first.parse::<f64>() {
        Ok(val) => val,
        Err(_) => return None,
    };
    return Some(Line {
        line_type: operation,
        value: number,
        combined: true,
    });
} else {
    return None;
}
}

fn process(buff: &String) -> Option<Line> {
    let parsed = buff.trim().parse::<f64>();

    return match parsed {
        Ok(val) => Some(Line {
            line_type: LineType::Number,
            value: val,
            combined: false,
        }),
        Err(_) => match buff.trim().parse::<char>() {
            Ok(val) => match try_operator(&val) {
                Some(t) => Some(Line {
                    line_type: t,
                    value: 0.0,
                    combined: false,
                }),
                _ => None,
            },
            Err(_) => match buff.trim().parse::<String>() {
                Ok(val) => match try_command(&val) {
                    Some(t) => Some(Line {
                        line_type: t,
                        value: 0.0,
                        combined: false,
                    }),
                    None => try_combined(&val),
                },
                Err(_) => None,
            },
        },
    };
}

fn poptwo(v: &mut Vec<f64>) -> Option<(f64, f64)> {
    if let Some(a) = v.pop() {
        if let Some(b) = v.pop(){
            return Some((a, b));
        } else {
            v.push(a);
        }
    }
    return None;
}

fn printresult(r: &f64) {
    println!("{}", r.to_string().bold());
    println!("");
}

fn execute(val: &Line, stack: &mut Vec<f64>, undos: &mut Vec<f64>) {
    if val.combined {
        stack.push(val.value);
    }

    match (*val).line_type {
        LineType::Number => stack.push((*val).value),
        LineType::Print => {
            if let Some(last) = &stack.last() {
                println!("{}", last);
            } else {
                println!("?");
            }
        }
        LineType::Full => {
            for s in stack {
                print!("{}\n", s);
            }
            print!("\n");
            io::stdout().flush().expect("flush failed");
        }
        LineType::Pop => {
            if let Some(r) = stack.pop() {
                printresult(&r);
                printresult(&r);
            };
        }
        LineType::UndoPop => {
            if let Some(r) = undos.pop() {
                stack.push(r);
                printresult(&r);
            } else {
                println!("?");
            }
        }
        LineType::Swap => {
            if let Some((a, b)) = poptwo(stack) {
                stack.push(a);
                stack.push(b);
            } else {
                println!("?");
            }
        }
        LineType::Flip => stack.reverse(),
        LineType::Clear => *stack = Vec::new(),
        LineType::ClearScreen => print!("{esc}c", esc = 27 as char),
        LineType::Plus => {
            if let Some((a, b)) = poptwo(stack) {
                let r = a + b;
                printresult(&r);
                stack.push(r);
            } else {
                println!("?");
            }
        }
        LineType::Minus => {
            if let Some((a, b)) = poptwo(stack) {
                let r = a - b;
                printresult(&r);
                stack.push(r);
            } else {
                println!("?");
            }
        }
        LineType::Times => {
            if let Some((a, b)) = poptwo(stack) {
                let r = a * b;
                printresult(&r);
                stack.push(r);
            } else {
                println!("?");
            }
        }
        LineType::Divide => {
            if let Some((a, b)) = poptwo(stack) {
                let r = b / a;
                printresult(&r);
                stack.push(r);
            } else {
                println!("?");
            }
        }
        LineType::Pow => {
            if let Some((a, b)) = poptwo(stack) {
                let r = f64::powf(b, a);
                printresult(&r);
                stack.push(r);
            } else {
                println!("?");
            }
        }
        LineType::Negate => {
            if let Some(a) = stack.pop() {
                let r = -a;
                printresult(&r);
                stack.push(r);
            } else {
                println!("?");
            }
        }
        LineType::Quit => std::process::exit(0),
    }
}

fn main() {
    println!("rp: command line rpn calculator");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
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
        ud:                              undo drop
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
    let mut undos: Vec<f64> = Vec::new();
    loop {
        buff.clear();
        print!("> ");
        io::stdout().flush().expect("flush failed");
        io::stdin().read_line(&mut buff).expect("readline failed");
        let vals = process(&buff);
        match vals {
            Some(v) => execute(&v, &mut stack, &mut undos),
            None => {
                println!("?");
                continue;
            }
        }
    }
}
