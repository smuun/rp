use colored::Colorize;
use std::io;
use std::io::Write;

use crate::line::Line;
use crate::line::LineType;

fn printresult(r: &f64) {
    println!("{}", r.to_string().bold());
    println!("");
}

fn poptwo(v: &mut Vec<f64>) -> Option<(f64, f64)> {
    if let Some(a) = v.pop() {
        if let Some(b) = v.pop() {
            return Some((a, b));
        } else {
            v.push(a);
        }
    }
    return None;
}

pub fn execute(operation: &Line, stack: &mut Vec<f64>, undos: &mut Vec<f64>) {
    if operation.combined {
        stack.push(operation.value);
    }

    match operation.line_type {
        LineType::Number => stack.push(operation.value),
        LineType::Print => {
            if let Some(last) = stack.last() {
                println!("{}", *last);
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
