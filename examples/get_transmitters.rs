use dapnet_api::Client;

#[tokio::main]
async fn main() {
    let username = std::env::var("DAPNET_USERNAME").unwrap();
    let password = std::env::var("DAPNET_PASSWORD").unwrap();

    let client = Client::new(&username, &password);

    let transmitters = client.get_all_transmitters().await.unwrap().unwrap();

    for transmitter in transmitters {
        println!("{}", transmitter.name);
    }
}
