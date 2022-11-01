use std::env;
use std::io;
use std::io::Write;

mod calculate;
mod line;
mod parse;

fn main() {
    println!();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!(
            "rp: command line rpn calculator
        USAGE:  rp [--help]

        OPTIONS:
        --help:                          print this message

        COMMANDS:
        help:                            print this message
        <number>:                        add to stack
        (+, -, *, /, ^):                 operate on last two elements
        _:                               negate last element
        clear, cl:                       clear screen
        p:                               print top of stack
        f:                               print whole stack
        d:                               drop last element
        ud:                              undo drop
        c:                               clear stack
        s:                               flip last two elements
        S:                               flip stack
        (q, quit, exit, <C-c>):          quit
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
        let operation = parse::process_input(&buff);
        match operation {
            Some(v) => calculate::execute(&v, &mut stack, &mut undos),
            None => {
                println!("?");
                continue;
            }
        }
    }
}
