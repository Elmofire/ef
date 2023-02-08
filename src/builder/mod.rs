use std::env;
use std::path::{Path};
use std::process::Command;
use std::fs;
use std::io::{self, Write};
use is_executable::IsExecutable;

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

pub fn build(p_res: &str, rand_dir: &str, cross_build: &str, output_fn: &str) {
    // Get Origin working directory
    let og_cwd: String = get_current_working_dir();
    // Locate the /tmp folder and switch to it
    let temp_directory = env::temp_dir();
    assert!(env::set_current_dir(&temp_directory).is_ok());
    // Intialize the Obfuscated Payload Crate
    let create = Command::new("cargo")
    .args(["init", &rand_dir])
    .output()
    .expect("Failed to execute command");
    io::stdout().write_all(&create.stdout).unwrap();
    // Change location to inside the /src file of the Payload crate.
    let payload_loc = format!("{}/{}", &rand_dir, "src");
    let payload_root = Path::new(&payload_loc);
    assert!(env::set_current_dir(&payload_root).is_ok());    
    // Remove old main.rs file
    fs::remove_file("main.rs").expect("could not remove file");
    // Send payload code to new main.rs
    let mut payload_main = std::fs::File::create("main.rs").expect("create failed");
    payload_main.write_all(p_res.as_bytes()).expect("write failed");
    // Step back
    let step_back = Path::new("../");
    assert!(env::set_current_dir(&step_back).is_ok());
    // Add package dependencies to the Cargo.toml
    let add_deps = Command::new("cargo")
    .args(["add", "async-std", "futures", "subprocess"])
    .output()
    .expect("Failed to execute command");
    io::stdout().write_all(&add_deps.stdout).unwrap();
    // Cross build the payload for provided target value
    let build_target = format!("{}", &cross_build);
    let build = Command::new("cross")
    .args(["build", "--target", &build_target])
    .output()
    .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&build.stdout));
    io::stdout().write_all(&build.stdout).unwrap();
    // Declare output filename
    let outfile = format!("{}/{}", og_cwd, output_fn);
    // Get path to generated executable
    let payload_path = format!("{}/{}/{}/", "target", cross_build, "debug");
    let path = Path::new(&payload_path);
    // Enumerate the target/architecture/debug folder structure searching for the executable file 
    for payload_file in fs::read_dir(path).unwrap() {
        let payload_file = payload_file.unwrap();
        let payload = payload_file.path();
        if payload.is_file() && payload.is_executable() {
            let p_path = format!("{}/{}", payload_path,payload.file_name().unwrap().to_str().unwrap());
            // Copy the executable to the user's present directory as the file name specified by the user.
            Command::new("cp")
            .args([&p_path, &outfile])
            .output()
            .expect("Failed to execute command");
        }
    }
    // Set current payload path based on current file system orientation.
    let payload_path = get_current_working_dir();
    //Return to origin.
    assert!(env::set_current_dir(&og_cwd).is_ok());
    // FIX ME: Remove the payload artifacts 
    //fs::remove_dir(&payload_path).expect("could not remove folder");
    Command::new("rm")
    .args(["-rf", &payload_path])
    .output()
    .expect("Failed to execute command");
    // Show payload type with 'file' command.
    println!("Payload Generation complete.\n");
    let file_cmd = Command::new("file")
    .args([&outfile])
    .output()
    .expect("Failed to execute command");
    println!("Results: {}", String::from_utf8_lossy(&file_cmd.stdout));
}