use dapnet_api::{Client, OutgoingNews};

#[tokio::main]
async fn main() {
    let username = std::env::var("DAPNET_USERNAME").unwrap();
    let password = std::env::var("DAPNET_PASSWORD").unwrap();
    let rubric_name = std::env::var("DAPNET_RUBRIC").unwrap();

    let client = Client::new(&username, &password);

    client
        .new_news(&OutgoingNews::new(
            rubric_name.clone(),
            format!("{username}: this is a test"),
        ))
        .await
        .unwrap();

    let news = client.get_news(&rubric_name).await.unwrap();
    println!("news: {:#?}", news);
}
