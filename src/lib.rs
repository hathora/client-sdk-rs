use std::net::TcpStream;

use reqwest::Url;

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

use serde::{Deserialize, Serialize};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};

use anyhow::{bail, Result};

#[derive(Serialize, Debug)]
struct InitialState {
    token: String,
    stateId: String,
}

#[derive(Deserialize)]
struct CreateRoomResponse {
    stateId: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Token {
    id: String,
}

pub fn login_anonymous(app_id: &str) -> Result<LoginResponse> {
    let login_url = format!("https://coordinator.hathora.dev/{app_id}/login/anonymous");
    let client = reqwest::blocking::Client::new();
    let resp: LoginResponse = client.post(login_url).send()?.json()?;
    Ok(resp)
}

pub fn decode_user_id_without_validating_jwt(token: &str) -> Result<String> {
    let segments: Vec<&str> = token.split('.').collect();
    let bytes = base64::decode_config(segments[1], base64::URL_SAFE_NO_PAD)?;
    let token: Token = serde_json::from_slice(&bytes)?;
    Ok(token.id)
}

pub fn create_nonblocking_subscribed_websocket(
    app_id: &str,
    token: &str,
    room_id: &str,
) -> Result<WebSocket<MaybeTlsStream<TcpStream>>> {
    let websocket_url = format!("wss://coordinator.hathora.dev/connect/{app_id}");
    let (mut socket, _response) =
        connect(Url::parse(&websocket_url).unwrap()).expect("Can't connect to websockets");
    let initial_state = InitialState {
        token: token.to_string(),
        stateId: room_id.to_string(),
    };
    let message = serde_json::to_vec(&initial_state).expect("Serialization should work");
    socket.write_message(Message::binary(message))?;
    match socket.get_mut() {
        MaybeTlsStream::Plain(tcp_stream) => {
            tcp_stream.set_nonblocking(true)?;
        }
        MaybeTlsStream::NativeTls(tls_stream) => {
            tls_stream.get_mut().set_nonblocking(true)?;
        }
        _ => {
            bail!("Unknown socket type.");
        }
    }
    Ok(socket)
}

pub fn create_room(app_id: &str, token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let create_url = format!("https://coordinator.hathora.dev/{app_id}/create");
    let response: CreateRoomResponse = client
        .post(create_url)
        .header(AUTHORIZATION, token)
        .header(CONTENT_TYPE, "application/octet-stream")
        .body(vec![])
        .send()?
        .json()?;
    Ok(response.stateId)
}
