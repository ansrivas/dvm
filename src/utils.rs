// MIT License
//
// Copyright (c) 2018 Ankur Srivastava
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the 'Software'), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
