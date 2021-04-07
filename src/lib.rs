use parking_lot::{Mutex, RwLock};
use std::sync::mpsc;

use device_query::{DeviceQuery, DeviceState};
use node_bindgen::derive::node_bindgen;
use once_cell::sync;

static DEVICEMPSC: sync::Lazy<(
    Mutex<mpsc::Sender<Vec<String>>>,
    Mutex<mpsc::Receiver<Vec<String>>>,
)> = sync::Lazy::new(|| {
    let (tx, rx) = mpsc::channel::<Vec<String>>();
    (Mutex::new(tx), Mutex::new(rx))
});

static SHOULDSTOP: sync::Lazy<RwLock<bool>> = sync::Lazy::new(|| RwLock::new(false));

static DEVICETHREAD: sync::Lazy<Mutex<Option<std::thread::JoinHandle<bool>>>> =
    sync::Lazy::new(|| Mutex::new(None));

#[node_bindgen]
fn start() {
    *SHOULDSTOP.write() = false;
    *DEVICETHREAD.lock() = Some(std::thread::spawn(|| {
        let sender = DEVICEMPSC.0.lock();
        let device_state = DeviceState::new();
        let mut prev_keys = vec![];
        loop {
            let keys = device_state.get_keys();
            if *SHOULDSTOP.read() {
                return true;
            } else if keys != prev_keys {
                let returnkeys: Vec<String> =
                    keys.clone().into_iter().map(|x| format!("{}", x)).collect();
                sender.send(returnkeys).unwrap();
            }
            prev_keys = keys;
        }
    }));
}

#[node_bindgen]
fn get_keys() -> Result<Vec<String>, String> {
    let reciever = DEVICEMPSC.1.lock();
    match reciever.recv() {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}

#[node_bindgen]
fn unload() -> Result<(), &'static str> {
    *SHOULDSTOP.write() = true;
    match DEVICETHREAD.lock().take().unwrap().join() {
        Ok(true) => Ok(()),
        _ => Err("Failed to kill worker thread"),
    }
}

#[node_bindgen]
fn is_running() -> bool {
    DEVICETHREAD.lock().is_some()
}

#[node_bindgen]
fn stop() -> Result<(), &'static str> {
    *SHOULDSTOP.write() = true;
    match DEVICETHREAD.lock().take().unwrap().join() {
        Ok(true) => std::process::exit(0),
        _ => Err("Failed to kill worker thread"),
    }
}
