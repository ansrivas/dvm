use std::io;
use std::process::Command;

/// Get a boolean user input for current operation
pub fn confirm_user_input() -> bool {

    println!("Do you want to continue? [yes/no]");
    let mut input_text = String::new();
    loop {
        input_text.clear();
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read from stdin");

        match input_text.to_lowercase().trim() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Unexpected input, please enter `yes` or `no`"),
        }
    }
}

/// Check if the docker-volume exists.
pub fn docker_volume_exist(volume: &str) -> bool {

    let status = Command::new("docker")
        .arg("volume")
        .arg("inspect")
        .arg(volume)
        .status()
        .expect("docker volume command failed to start");

    status.success()
}
