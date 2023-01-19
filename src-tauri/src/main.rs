#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Listen for key events using device_query library and when a key event occurs, send it on the mpsc channel
use device_query::{DeviceEvents, DeviceState};
use std::sync::{mpsc, Mutex};
struct MpscChannel {
    inner: Mutex<mpsc::Sender<String>>,
}

fn listen_for_key_events(sender: tauri::State<'_, MpscChannel>) {
    let device_state = DeviceState::new();

    let _guard = device_state.on_key_up(|key| {
        let output_tx = *sender.inner.lock().unwrap();
        output_tx.send(key.to_string());
    });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
