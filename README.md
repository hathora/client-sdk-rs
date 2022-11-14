# Hathora Rust Client SDK

<a href="https://crates.io/crates/hathora-client-sdk"><img src="https://img.shields.io/crates/v/hathora-client-sdk.svg" alt="crate version"></a>

See this client in action here: https://github.com/hathora/topdown-shooter-bevy-client

## Usage

```rs
let app_id = "e2d8571eb89af72f2abbe909def5f19bc4dad0cd475cce5f5b6e9018017d1f1c".to_string();
let client = HathoraClient::new(app_id, None);
let token = client
    .login_anonymous()
    .expect("Logging in should succeed.");
let roomId = client
    .create(&token, vec![])
    .expect("Creating a room should succeed");
let mut transport = client
    .connect(&token, &roomId)
    .expect("Creating a websocket should succeed.");

let message = transport
    .read()
    .expect("Reading from websocket should succeed");
println!("Got message: {:?}", message);
transport
    .write(b"{ message: \"Hello world\" }".to_vec())
    .expect("Writing to socket should suceed");
```

## Publishing

```bash
cargo publish
```
