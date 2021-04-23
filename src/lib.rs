use device_query::{DeviceQuery, DeviceState};
use node_bindgen::derive::node_bindgen;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[cfg(windows)]
use winput::{message_loop, Action, Vk};

const VERSION: &str = env!("CARGO_PKG_VERSION");

static THREAD: Lazy<Mutex<Option<stoppable_thread::StoppableHandle<()>>>> =
    Lazy::new(|| Mutex::new(None));

#[cfg(windows)]
#[node_bindgen(mt)]
fn start<F: Fn(Vec<String>) + Send + 'static>(returnjs: F) {
    *THREAD.lock() = Some(stoppable_thread::spawn(move |stopvar| {
        let mut receiver = message_loop::start().unwrap();
        let mut keys_return = vec![];
        loop {
            if stopvar.get() {
                receiver.stop();
                break;
            }
            match receiver.next_event() {
                message_loop::Event::Keyboard {
                    vk,
                    action: Action::Press,
                    ..
                } => {
                    let key = char::from(vk.into_u8()).to_string();
                    keys_return.push(key);
                    returnjs(keys_return.clone());
                }
                message_loop::Event::Keyboard {
                    vk,
                    action: Action::Release,
                    ..
                } => {
                    let key = char::from(vk.into_u8()).to_string();
                    let rem_index = keys_return.par_iter().position_any(|x| x.clone() == key);
                    match rem_index {
                        Some(rem) => {
                            keys_return.remove(rem);
                            returnjs(keys_return.clone());
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
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
