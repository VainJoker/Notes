#[tokio::main]
async fn main() -> mini_redis::Result<()>{
    let mut client = mini_redis::client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;
    println!("从服务器短获取的结果为{:?}",result);
    Ok(())
}
