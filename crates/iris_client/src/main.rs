use iris_client::{connect, Expression, DeleteExpression};

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut client = connect("127.0.0.1:3000").await?;

    let deleted = client.delete(DeleteExpression::ID("1")).await?;
    let result = client.count(Expression::Number(-1)).await?;

    // let result = client
    //     .pipe()
    //     .set("0", "foo bar")
    //     .get("")
    //     .execute()
    //     .await;

    println!("{} {:?}", deleted.data, result);

    Ok(())
}
