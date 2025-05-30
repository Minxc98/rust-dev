use std::sync::Arc;
use tokio::sync::broadcast;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    extract::State,
};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct WsManager {
    tx: broadcast::Sender<String>,
}

impl WsManager {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self { tx }
    }

    pub fn broadcast(&self, message: String) -> Result<(), broadcast::error::SendError<String>> {
        self.tx.send(message).map(|_| ())
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<WsManager>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<WsManager>) {
    use futures_util::SinkExt;
    let (mut sender, mut receiver) = socket.split();

    // 订阅广播消息
    let mut rx = state.tx.subscribe();

    // 处理接收到的消息
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                println!("Received message: {}", text);
            }
        }
    });

    // 处理广播消息
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let message = Message::Text(msg.to_string().into());
            if sender.send(message).await.is_err() {
                break;
            }
        }
    });

    // 等待任意一个任务完成
    tokio::select! {
        _ = &mut recv_task => send_task.abort(),
        _ = &mut send_task => recv_task.abort(),
    }
}

// REST API 请求体
#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastMessage {
    pub message: String,
}

// REST API 处理函数
pub async fn broadcast_message(
    State(state): State<Arc<WsManager>>,
    axum::Json(payload): axum::Json<BroadcastMessage>,
) -> Result<(), String> {
    state
        .broadcast(payload.message)
        .map_err(|e| e.to_string())
} 