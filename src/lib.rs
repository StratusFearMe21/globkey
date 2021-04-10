use parking_lot::Mutex;
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

static DEVICETHREAD: sync::Lazy<Mutex<Option<stoppable_thread::StoppableHandle<()>>>> =
    sync::Lazy::new(|| Mutex::new(None));

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[node_bindgen]
fn start() {
    *DEVICETHREAD.lock() = Some(stoppable_thread::spawn(|stop| {
        let sender = DEVICEMPSC.0.lock();
        let device_state = DeviceState::new();
        let mut prev_keys = vec![];
        while !stop.get() {
            let keys = device_state.get_keys();
            if keys != prev_keys {
                let returnkeys: Vec<String> =
                    keys.clone().into_iter().map(|x| format!("{}", x)).collect();
                sender.send(returnkeys).unwrap();
            }
            prev_keys = keys;
        }
        ()
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
    match DEVICETHREAD.lock().take().unwrap().stop().join() {
        Ok(()) => Ok(()),
        _ => Err("Failed to kill worker thread"),
    }
}

#[node_bindgen]
fn is_running() -> bool {
    DEVICETHREAD.lock().is_some()
}

#[node_bindgen]
fn stop() -> Result<(), &'static str> {
    match DEVICETHREAD.lock().take().unwrap().stop().join() {
        Ok(()) => std::process::exit(0),
        _ => Err("Failed to kill worker thread"),
    }
}

#[node_bindgen]
fn version() -> String {
    VERSION.to_string()
}
