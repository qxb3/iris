<h1 align="center">iris_client</h1>
<p align="center">
    <img src="https://img.shields.io/crates/v/iris_client" />
    <img src="https://img.shields.io/crates/dr/iris_client" />
</p>
<p align="center">A crate to interact with iris. an in-memory database</p>

## Installation

```toml
[dependencies]
iris_client = "0.1.1"
```

## Example

```rust
use iris_client::{connect, Expression, DeleteExpression};

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut client = connect("127.0.0.1:3000").await?;

    // Sets an item in the database
    let user_id = client.set("user:joe", "foo bar").await?; // Returns the id so "user:joe"

    // Gets the value based on the id
    let user_value = client.get(user_id).await?; // Returns "foo bar"

    // List items in the database based on count. -1 means get all of them
    let list_count = client.list(Expression::Number(-1)).await?; // Returns Vec<Item>

    // List items in the database based on range. You can also do (3..-1) to get the items from 3 up to the length of the items
    let list_expr = client.list(Expression::Range(0..3)).await?; // Returns Vec<Item>

    // Just returns how many items currently in the database
    let count = client.count(Expression::Number(-1)).await?; // Returns u32

    // Deletes an item in the database based on id
    let deleted_user_id = client.delete(DeleteExpression::ID("user:joe")).await?; // Returns Vec<Item>

    // Deletes an item in the database based on count. (This deletes every item from 0 to 2)
    let deleted_user_count = client.delete(DeleteExpression::Number(2)).await?; // Returns Vec<Item>

    // Deletes an item in the database based on range.
    let deleted_user_expr = client.delete(DeleteExpression::Range(0..2)).await?; // Returns Vec<Item>

    // If you want you can also send commands raw
    let raw = client.delete("GET user:joe").await?; // Returns ServerResponse

    Ok(())
}
```

## Pipes

You also can pipe commands. The return value of the previous command will be appended to the current command

```rust
use iris_client::connect;

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut client = connect("127.0.0.1:3000").await?;

    // NOTE: I will rewrite this api to make it better. It sucks rn
    let pipe_commands = client.pipe()
        .pipe()
        .set("someid", "foo bar") // Returns an id
        .get("") // The id will be appended in here
        .await?; // Returns ServerResponse for now.

    // This shows how you can pipe commands in raw. Tbh this makes sense more than the current pipe api
    let pipe_raw = client.raw("SET someid this is data ~> GET").await?; // Returns ServerResponse

    Ok(())
}
```

## Contribution

Contributions to iris are welcome! If you have ideas for improvements, new features, or bug fixes, feel free to open an issue or submit a pull request on [iris](https://github.com/qxb3/iris)
