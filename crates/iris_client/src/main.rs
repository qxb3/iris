use iris_client::{connect, Expression};

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut client = connect("127.0.0.1:3000").await?;

    for i in 0..10 {
        let i = i.to_string();
        client.set(&i, &i).await.unwrap();
    }

    let list = client.list(Expression::Number(-1)).await?;
    let count = client.count(Expression::Number(-1)).await?;

    // let result = client
    //     .pipe()
    //     .set("0", "foo bar")
    //     .get("")
    //     .execute()
    //     .await;

    println!("{:#?}", list);
    println!("count => {count}");

    Ok(())
}
