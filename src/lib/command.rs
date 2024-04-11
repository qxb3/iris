#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expr {
    Number(i32),
    Range(i32, i32)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command<'a> {
    Get { id: u32 },
    List { expr: Expr },
    Count { expr: Expr },
    Set { id: u32, data: String },
    Delete { expr: Expr },
    Pipe {},
    Invalid { reason: &'a str },
}

pub fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts.as_slice() {
        // Error Handling
        ["GET", "SET", "APP"] => Command::Invalid { reason: "Missing ID" },
        ["SET" | "APP", _id] => Command::Invalid { reason: "Missing Data" },
        ["LST" | "CNT" | "DEL"] => Command::Invalid { reason: "Missing Expression" },

        ["GET", id] => match id.parse::<u32>() {
            Ok(id) => Command::Get { id },
            Err(_) => Command::Invalid { reason: "Invalid ID" },
        },

        ["LST", expr] => match parse_expr(expr) {
            Ok(expr) => Command::List { expr },
            Err(err) => Command::Invalid { reason: err },
        },

        ["CNT", expr] => match parse_expr(expr) {
            Ok(expr) => Command::Count { expr },
            Err(err) => Command::Invalid { reason: err },
        },

        ["SET", id, data @ ..] => match id.parse::<u32>() {
            Ok(id) => Command::Set { id, data: data.join(" ") },
            Err(_) => Command::Invalid { reason: "Invalid ID" },
        },

        ["DEL", expr] => match parse_expr(expr) {
            Ok(expr) => Command::Delete { expr },
            Err(err) => Command::Invalid { reason: err },
        },

        _ => Command::Invalid { reason: "Invalid Command" },
    }
}

fn parse_expr<'a>(expr_str: &'a str) -> Result<Expr, &'a str> {
    if let Ok(number) = expr_str.parse::<i32>() {
        return Ok(Expr::Number(number));
    }

    let parts: Vec<&str> = expr_str.split("..").collect();
    if parts.len() == 2 {
        if let Ok(start) = parts[0].parse::<i32>() {
            if let Ok(end) = parts[1].parse::<i32>() {
                return Ok(Expr::Range(start, end));
            }
        }
    }

    Err("Invalid expression")
}
