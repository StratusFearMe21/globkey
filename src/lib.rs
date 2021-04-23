use node_bindgen::derive::node_bindgen;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::convert::TryInto;

use std::mem;
#[cfg(windows)]
use winapi::{
    shared::{hidusage, windef::HWND},
    um::winuser,
};

#[cfg(windows)]
use winapi::um::winuser::{GetAsyncKeyState, RegisterHotKey, MSG};

const VERSION: &str = env!("CARGO_PKG_VERSION");

// static HOTKEY: Mutex<Vec<String>> = Mutex::new(Vec::new());

static THREAD: Lazy<Mutex<Option<stoppable_thread::StoppableHandle<()>>>> =
    Lazy::new(|| Mutex::new(None));

#[cfg(windows)]
#[node_bindgen(mt)]
fn start<F: Fn() + Send + 'static>(
    // keybind: Vec<String>,
    pressed: F,
    // released: B,
) {
    *THREAD.lock() = Some(stoppable_thread::spawn(move |stopvar| unsafe {
        RegisterHotKey(
            0 as HWND,
            1,
            (winapi::um::winuser::MOD_CONTROL | winapi::um::winuser::MOD_NOREPEAT)
                .try_into()
                .unwrap(),
            0x42,
        );

        let mut rid: [winuser::RAWINPUTDEVICE; 1] = mem::zeroed();
        // Keyboard
        rid[0].dwFlags = winuser::RIDEV_NOLEGACY | winuser::RIDEV_INPUTSINK;
        rid[0].usUsagePage = hidusage::HID_USAGE_PAGE_GENERIC;
        rid[0].usUsage = hidusage::HID_USAGE_GENERIC_KEYBOARD;
        rid[0].hwndTarget = 0 as HWND;

        winuser::RegisterRawInputDevices(
            rid.as_ptr(),
            rid.len() as _,
            mem::size_of::<winuser::RAWINPUTDEVICE>() as _,
        );

        let mut message: MSG = std::mem::MaybeUninit::zeroed().assume_init();
        let mut hotpressed = false;
        while (winapi::um::winuser::GetMessageW(&mut message, 0 as HWND, 0, 0) != 0)
            && !stopvar.get()
        {
            match message.message {
                winapi::um::winuser::WM_HOTKEY => {
                    println!("pressed");
                    hotpressed = true;
                }
                winapi::um::winuser::WM_INPUT => {
                    if hotpressed {
                        if GetAsyncKeyState(0x42) == 0 {
                            println!("released");
                            hotpressed = false;
                        }
                    }
                }
                _ => {}
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
