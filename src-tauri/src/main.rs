#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Listen for key events using device_query library and when a key event occurs, send it on the mpsc channel
use device_query::{DeviceEvents, DeviceState};
use std::sync::{mpsc, Mutex};
use tauri::Manager;
struct MpscChannel {
    inner: Mutex<mpsc::Sender<String>>,
}

fn listen_for_key_events(sender: MpscChannel) {
    let device_state = DeviceState::new();

    let _guard = device_state.on_key_up(move |key| {
        let output_tx = &*sender.inner.lock().unwrap();
        output_tx.send(key.to_string()).unwrap();
        // println!("Listening on backend: {}\n", key.to_string());
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
                    // println!("Lisening on Main thread: {}", key);
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
