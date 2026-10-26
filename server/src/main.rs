use common::DrawMessage;
use futures_util::{SinkExt, StreamExt};
use serde_json;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("server running on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut write, mut read) = ws_stream.split();

            while let Some(msg) = read.next().await {
                let msg = msg.unwrap();
                if msg.is_text() {
                    let data: DrawMessage = serde_json::from_str(msg.to_text().unwrap()).unwrap();
                    println!("recv: {:?}", data);
                    let resp = serde_json::to_string(&data).unwrap();
                    write.send(Message::Text(resp)).await.unwrap();
                }
            }
        });
    }
}
