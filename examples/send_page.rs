use dapnet_api::{Client, OutgoingCall};

#[tokio::main]
async fn main() {
    let username = std::env::var("DAPNET_USERNAME").unwrap();
    let password = std::env::var("DAPNET_PASSWORD").unwrap();

    let client = Client::new(&username, &password);

    client
        .new_call(&OutgoingCall::new(
            format!("{username}: this is a test"),
            vec![username.clone()],
            vec!["uk-all".to_string()],
        ))
        .await
        .unwrap();

    let calls = client.get_calls_by(&username).await.unwrap();
    println!("calls: {:#?}", calls);
}
