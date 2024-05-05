use dapnet_api::{Client, OutgoingCallBuilder};

#[tokio::main]
async fn main() {
    let username = std::env::var("DAPNET_USERNAME").unwrap();
    let password = std::env::var("DAPNET_PASSWORD").unwrap();

    let client = Client::new(&username, &password);

    client
        .new_call(
            &OutgoingCallBuilder::default()
                .text(format!("{username}: this is a test"))
                .recipients(vec![username.clone()])
                .transmitter_groups(vec!["uk-all".to_string()])
                .build()
                .unwrap(),
        )
        .await
        .unwrap();

    let calls = client.get_calls_by(&username).await.unwrap();
    println!("calls: {:#?}", calls);
}
