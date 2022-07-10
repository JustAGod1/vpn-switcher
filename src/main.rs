use std::env;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use futures::StreamExt;
use telegram_bot::{Api, Error, Message, MessageKind, SendMessage, UpdateKind};

#[tokio::main]
async fn main() {
    loop {
        if let Err(e) = run().await {
            println!("{}", e)
        }
        sleep(Duration::new(5, 0))
    }
}

async fn run() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);


    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let Err(s) = handle_message(&api, &message).await {
                api.send(SendMessage::new(&message.chat, s)).await?;
            }
        }
    }

    Ok(())
}

async fn handle_message(api: &Api, message: &Message) -> Result<(), String> {
    if let MessageKind::Text { ref data, .. } = message.kind {
        if let Some(first) = data.split_whitespace().next() {
            if first.starts_with('/') {
                let name = &first[1..];

                return if name == "start" {
                    start(&api, &message).await
                } else if name == "enable" {
                    enable(&api, &message).await
                } else if name == "disable" {
                    disable(&api, &message).await
                } else {
                    Ok(())
                };
            }
        }
    }

    Ok(())
}

async fn start(api: &Api, message: &Message) -> Result<(), String> {
    let m = SendMessage::new(&message.chat, "/enable - enables vpn\n/disable - disables vpn");
    api.send(m).await.map_err(|a| a.to_string())?;

    Ok(())
}

async fn enable(api: &Api, message: &Message) -> Result<(), String> {
    let code = Command::new("sshpass")
        .args(&["-p", "mhg7zeg3hxt@HGB8xum", "ssh", "admin@192.168.1.1", "ip", "hotspot", "host", "b4:e6:2a:d6:a2:18", "policy", "Policy0"])
        .spawn()
        .map_err(|a| a.to_string())?
        .wait()
        .map_err(|a| a.to_string())?
        .code();

    if !matches!(code, Some(0)) {
        return Err(String::from("Unsuccessful"))
    }

    api.send(SendMessage::new(&message.chat, "Done")).await.map_err(|a| a.to_string())?;

    Ok(())
}

async fn disable(api: &Api, message: &Message) -> Result<(), String> {
    let code = Command::new("sshpass")
        .args(&["-p", "mhg7zeg3hxt@HGB8xum", "ssh", "admin@192.168.1.1", "no", "ip", "hotspot", "host", "b4:e6:2a:d6:a2:18", "policy"])
        .spawn()
        .map_err(|a| a.to_string())?
        .wait()
        .map_err(|a| a.to_string())?
        .code();

    if !matches!(code, Some(0)) {
        return Err(String::from("Unsuccessful"))
    }

    api.send(SendMessage::new(&message.chat, "Done")).await.map_err(|a| a.to_string())?;

    Ok(())
}
