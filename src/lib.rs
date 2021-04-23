#[cfg(not(windows))]
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
                    let key = match vk {
                        Vk::_0 => "Key0".to_string(),
                        Vk::_1 => "Key1".to_string(),
                        Vk::_2 => "Key2".to_string(),
                        Vk::_3 => "Key3".to_string(),
                        Vk::_4 => "Key4".to_string(),
                        Vk::_5 => "Key5".to_string(),
                        Vk::_6 => "Key6".to_string(),
                        Vk::_7 => "Key7".to_string(),
                        Vk::_8 => "Key8".to_string(),
                        Vk::_9 => "Key9".to_string(),
                        Vk::F1 => "F1".to_string(),
                        Vk::F2 => "F2".to_string(),
                        Vk::F3 => "F3".to_string(),
                        Vk::F4 => "F4".to_string(),
                        Vk::F5 => "F5".to_string(),
                        Vk::F6 => "F6".to_string(),
                        Vk::F7 => "F7".to_string(),
                        Vk::F8 => "F8".to_string(),
                        Vk::F9 => "F9".to_string(),
                        Vk::F10 => "F10".to_string(),
                        Vk::F11 => "F11".to_string(),
                        Vk::F12 => "F12".to_string(),
                        Vk::Escape => "Escape".to_string(),
                        Vk::Space => "Space".to_string(),
                        Vk::LeftControl => "LControl".to_string(),
                        Vk::RightControl => "RControl".to_string(),
                        Vk::Control => "Control".to_string(),
                        Vk::LeftShift => "LShift".to_string(),
                        Vk::RightShift => "RShift".to_string(),
                        Vk::Alt => "LAlt".to_string(),
                        Vk::LeftWin => "Meta".to_string(),
                        Vk::Enter => "Enter".to_string(),
                        Vk::UpArrow => "Up".to_string(),
                        Vk::DownArrow => "Down".to_string(),
                        Vk::LeftArrow => "Left".to_string(),
                        Vk::RightArrow => "Right".to_string(),
                        Vk::Backspace => "Backspace".to_string(),
                        Vk::CapsLock => "CapsLock".to_string(),
                        Vk::Tab => "Tab".to_string(),
                        Vk::Home => "Home".to_string(),
                        Vk::End => "End".to_string(),
                        Vk::PageUp => "PageUp".to_string(),
                        Vk::PageDown => "PageDown".to_string(),
                        Vk::Insert => "Insert".to_string(),
                        Vk::Delete => "Delete".to_string(),
                        Vk::Numpad0 => "Numpad0".to_string(),
                        Vk::Numpad1 => "Numpad1".to_string(),
                        Vk::Numpad2 => "Numpad2".to_string(),
                        Vk::Numpad3 => "Numpad3".to_string(),
                        Vk::Numpad4 => "Numpad4".to_string(),
                        Vk::Numpad5 => "Numpad5".to_string(),
                        Vk::Numpad6 => "Numpad6".to_string(),
                        Vk::Numpad7 => "Numpad7".to_string(),
                        Vk::Numpad8 => "Numpad8".to_string(),
                        Vk::Numpad9 => "Numpad9".to_string(),
                        Vk::Oem3 => "Grave".to_string(),
                        Vk::Minus => "Minus".to_string(),
                        Vk::Oem4 => "LeftBracket".to_string(),
                        Vk::Oem6 => "RightBracket".to_string(),
                        Vk::Oem5 => "Backslash".to_string(),
                        Vk::Oem1 => "Semicolon".to_string(),
                        Vk::Oem7 => "Apostrophe".to_string(),
                        Vk::Comma => "Comma".to_string(),
                        Vk::Period => "Dot".to_string(),
                        Vk::Oem2 => "Slash".to_string(),
                        _ => char::from(vk.into_u8()).to_string(),
                    };
                    if !keys_return.contains(&key) {
                        keys_return.push(key);
                    }
                    returnjs(keys_return.clone());
                }
                message_loop::Event::Keyboard {
                    vk,
                    action: Action::Release,
                    ..
                } => {
                    let key = match vk {
                        Vk::_0 => "Key0".to_string(),
                        Vk::_1 => "Key1".to_string(),
                        Vk::_2 => "Key2".to_string(),
                        Vk::_3 => "Key3".to_string(),
                        Vk::_4 => "Key4".to_string(),
                        Vk::_5 => "Key5".to_string(),
                        Vk::_6 => "Key6".to_string(),
                        Vk::_7 => "Key7".to_string(),
                        Vk::_8 => "Key8".to_string(),
                        Vk::_9 => "Key9".to_string(),
                        Vk::F1 => "F1".to_string(),
                        Vk::F2 => "F2".to_string(),
                        Vk::F3 => "F3".to_string(),
                        Vk::F4 => "F4".to_string(),
                        Vk::F5 => "F5".to_string(),
                        Vk::F6 => "F6".to_string(),
                        Vk::F7 => "F7".to_string(),
                        Vk::F8 => "F8".to_string(),
                        Vk::F9 => "F9".to_string(),
                        Vk::F10 => "F10".to_string(),
                        Vk::F11 => "F11".to_string(),
                        Vk::F12 => "F12".to_string(),
                        Vk::Escape => "Escape".to_string(),
                        Vk::Space => "Space".to_string(),
                        Vk::LeftControl => "LControl".to_string(),
                        Vk::RightControl => "RControl".to_string(),
                        Vk::Control => "Control".to_string(),
                        Vk::LeftShift => "LShift".to_string(),
                        Vk::RightShift => "RShift".to_string(),
                        Vk::Alt => "LAlt".to_string(),
                        Vk::LeftWin => "Meta".to_string(),
                        Vk::Enter => "Enter".to_string(),
                        Vk::UpArrow => "Up".to_string(),
                        Vk::DownArrow => "Down".to_string(),
                        Vk::LeftArrow => "Left".to_string(),
                        Vk::RightArrow => "Right".to_string(),
                        Vk::Backspace => "Backspace".to_string(),
                        Vk::CapsLock => "CapsLock".to_string(),
                        Vk::Tab => "Tab".to_string(),
                        Vk::Home => "Home".to_string(),
                        Vk::End => "End".to_string(),
                        Vk::PageUp => "PageUp".to_string(),
                        Vk::PageDown => "PageDown".to_string(),
                        Vk::Insert => "Insert".to_string(),
                        Vk::Delete => "Delete".to_string(),
                        Vk::Numpad0 => "Numpad0".to_string(),
                        Vk::Numpad1 => "Numpad1".to_string(),
                        Vk::Numpad2 => "Numpad2".to_string(),
                        Vk::Numpad3 => "Numpad3".to_string(),
                        Vk::Numpad4 => "Numpad4".to_string(),
                        Vk::Numpad5 => "Numpad5".to_string(),
                        Vk::Numpad6 => "Numpad6".to_string(),
                        Vk::Numpad7 => "Numpad7".to_string(),
                        Vk::Numpad8 => "Numpad8".to_string(),
                        Vk::Numpad9 => "Numpad9".to_string(),
                        Vk::Oem3 => "Grave".to_string(),
                        Vk::Minus => "Minus".to_string(),
                        Vk::Oem4 => "LeftBracket".to_string(),
                        Vk::Oem6 => "RightBracket".to_string(),
                        Vk::Oem5 => "Backslash".to_string(),
                        Vk::Oem1 => "Semicolon".to_string(),
                        Vk::Oem7 => "Apostrophe".to_string(),
                        Vk::Comma => "Comma".to_string(),
                        Vk::Period => "Dot".to_string(),
                        Vk::Oem2 => "Slash".to_string(),
                        _ => char::from(vk.into_u8()).to_string(),
                    };
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
