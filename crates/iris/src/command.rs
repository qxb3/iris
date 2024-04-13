#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    ID(String),
    Number(i32),
    Range(i32, i32)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Get { id: String },
    List { expr: Expr },
    Count { expr: Expr },
    Set { id: String, data: String },
    Delete { expr: Expr },
    Invalid { reason: String },
}

pub fn parse_command(input: String) -> Command {
    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts.as_slice() {
        // Error Handling
        ["GET" | "SET" | "APP"] => Command::Invalid { reason: "Missing ID".to_owned() },
        ["LST" | "CNT" | "DEL"] => Command::Invalid { reason: "Missing Expression".to_string() },
        ["SET" | "APP", _id] => Command::Invalid { reason: "Missing Data".to_string() },

        ["GET", id] => Command::Get { id: id.to_string() },

        ["LST", expr] => match parse_expr(expr) {
            Ok(expr) => Command::List { expr },
            Err(err) => Command::Invalid { reason: err.to_owned() },
        },

        ["CNT", expr] => match parse_expr(expr) {
            Ok(expr) => Command::Count { expr },
            Err(err) => Command::Invalid { reason: err.to_string() },
        },

        ["SET", id, data @ ..] => Command::Set { id: id.to_string(), data: data.join(" ") },

        ["DEL", expr] => match parse_expr(expr) {
            Ok(expr) => Command::Delete { expr },
            Err(err) => Command::Invalid { reason: err.to_string() },
        },

        _ => Command::Invalid { reason: "Invalid Command".to_string() },
    }
}

fn parse_expr(expr_str: &str) -> Result<Expr, &str> {
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

    Ok(Expr::ID(expr_str.to_string()))
}
