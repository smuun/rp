use crate::line::LineType;
use crate::line::Line;

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
        _ => None,
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
    if let Some(last) = &(cs.next_back()) {
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

pub fn process_input(buff: &String) -> Option<Line> {
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
