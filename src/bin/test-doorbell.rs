use printer_actions::{
    remote::notify_homebridge::NotifyHomebridge, traits::notify_trait::Notifier,
};

fn main() {
    let web_client = reqwest::Client::new();
    let notifier = NotifyHomebridge::new(web_client);

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(notifier.notify())
        .unwrap();
}
