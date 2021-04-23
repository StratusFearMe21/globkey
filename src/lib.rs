#[cfg(not(windows))]
use device_query::{DeviceQuery, DeviceState};

use node_bindgen::derive::node_bindgen;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[cfg(windows)]
use inputbot::KeybdKey::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

static THREAD: Lazy<Mutex<Option<stoppable_thread::StoppableHandle<()>>>> =
    Lazy::new(|| Mutex::new(None));

#[cfg(windows)]
#[node_bindgen(mt)]
fn start<F: Fn(Vec<String>) + Sync + Send + 'static>(returnjs: F) {
    *THREAD.lock() = Some(stoppable_thread::spawn(move |stopvar| loop {
        LControlKey.bind(|| {
            returnjs(vec!["LControl".to_string(), "Key0".to_string()]);
            while LControlKey.is_pressed() {
                std::hint::spin_loop()
            }
            returnjs(vec![]);
        })
    }));
}

#[cfg(not(windows))]
#[node_bindgen(mt)]
fn start<F: Fn(Vec<String>) + Send + 'static>(returnjs: F) {
    *THREAD.lock() = Some(stoppable_thread::spawn(move |stopvar| {
        let device_state = DeviceState::new();
        let mut prev_keys = vec![];
        while !stopvar.get() {
            let keys = device_state.get_keys();
            if keys != prev_keys {
                let returnkeys: Vec<String> = keys
                    .clone()
                    .into_par_iter()
                    .map(|x| format!("{}", x))
                    .collect();
                returnjs(returnkeys);
            }
            prev_keys = keys;
        }
    }));
}

#[node_bindgen]
fn unload() -> Result<(), &'static str> {
    match THREAD.lock().take().unwrap().stop().join() {
        Ok(()) => Ok(()),
        _ => Err("Failed to kill worker thread"),
    }
}

#[node_bindgen]
fn is_running() -> bool {
    THREAD.lock().is_some()
}

#[node_bindgen]
fn stop() -> Result<(), &'static str> {
    match THREAD.lock().take().unwrap().stop().join() {
        Ok(()) => std::process::exit(0),
        _ => Err("Failed to kill worker thread"),
    }
}

#[node_bindgen]
fn version() -> String {
    VERSION.to_string()
}
