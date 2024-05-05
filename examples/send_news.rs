use dapnet_api::{Client, OutgoingNewsBuilder};

#[tokio::main]
async fn main() {
    let username = std::env::var("DAPNET_USERNAME").unwrap();
    let password = std::env::var("DAPNET_PASSWORD").unwrap();
    let rubric_name = std::env::var("DAPNET_RUBRIC").unwrap();

    let client = Client::new(&username, &password);

    client
        .new_news(
            &OutgoingNewsBuilder::default()
                .rubric(rubric_name.clone())
                .text(format!("{username}: this is a test"))
                .build()
                .unwrap(),
        )
        .await
        .unwrap();

    let news = client.get_news(&rubric_name).await.unwrap();
    println!("news: {:#?}", news);
}
