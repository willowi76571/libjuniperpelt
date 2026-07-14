use std::env;
const INFO: &str = "path-glob-b63ffc";
fn main() {
    println!("[{}] System Information:", INFO);
    println!("  OS: {}", env::consts::OS);
    println!("  Arch: {}", env::consts::ARCH);
    println!("  Family: {}", env::consts::FAMILY);
    println!("  CWD: {}", env::current_dir().map(|p| p.display().to_string()).unwrap_or_else(|_| "unknown".into()));
    println!("  EXE: {}", env::current_exe().map(|p| p.display().to_string()).unwrap_or_else(|_| "unknown".into()));
    let args: Vec<String> = env::args().collect();
    println!("  Args: {:?}", &args[1..]);
    println!("[{}] Done.", INFO);
}
