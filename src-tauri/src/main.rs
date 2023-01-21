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
use tauri::Manager;
struct MpscChannel {
    inner: Mutex<mpsc::Sender<String>>,
}

fn listen_for_key_events(sender: MpscChannel) {
    let device_state = DeviceState::new();

    let _guard = device_state.on_key_up(|key| {
        let output_tx = *sender.inner.lock().unwrap();
        output_tx.send(key.to_string());
    });

    loop {}
}

fn send_to_frontend<R: tauri::Runtime>(manager: &impl Manager<R>, key: String) {
    manager.emit_all("key_event", key).unwrap();
}

fn main() {
    let (output_tx, output_rx) = mpsc::channel::<String>();

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            tauri::async_runtime::spawn(async move {
                while let key = output_rx.recv().unwrap() {
                    send_to_frontend(&app_handle, key);
                }
            });

            // Listen for key events on a seperate runtime
            tauri::async_runtime::spawn(async move {
                listen_for_key_events(MpscChannel {
                    inner: Mutex::new(output_tx),
                });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
