#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    tokio::select! {
        _ = server_bg() => {
            println!("do_stuff_async() completed first")
        }
        _ = delayed_gui() => {
            println!("more_async_work() completed first")
        }
    };
}

async fn server_bg() {
    tokio::spawn(server::run()).await.unwrap().unwrap();
}

async fn delayed_gui() {
    tokio::time::sleep(Duration::from_secs_f32(0.01)).await;
    gui::run().await;
}
