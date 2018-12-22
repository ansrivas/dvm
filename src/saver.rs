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

use crate::utils::{confirm_user_input, docker_volume_exist, execute_command};
use chrono::prelude::*;
use clap::ArgMatches;
use std::path::PathBuf;

/// Insert a new customer in database.
///
/// # Arguments
///
/// * `Arg1` -
///
pub struct ImageSaver {
    volume_name: String,
    path_to_export_vol: String,
    is_interactive: bool,
}

impl ImageSaver {
    pub fn new(save_matches: &ArgMatches) -> Self {
        let volume = save_matches
            .value_of("volume")
            .unwrap()
            .parse::<String>()
            .expect("Volume name should be a string.");

        let path = save_matches
            .value_of("path")
            .unwrap()
            .parse::<String>()
            .expect("Path should be a string.");

        let is_interactive = save_matches
            .value_of("interactive")
            .unwrap()
            .parse::<bool>()
            .unwrap();

        ImageSaver {
            volume_name: volume,
            path_to_export_vol: path,
            is_interactive: is_interactive,
        }
    }

    fn exported_file_name(&self) -> String {
        let exporting_time = Local::now().format("%Y%m%d_%H%M%S").to_string();
        format!("{}_{}{}", self.volume_name, exporting_time, ".tgz")
    }

    fn get_volume_path(&self) -> String {
        let download_path = PathBuf::from(&self.path_to_export_vol);
        download_path
            .canonicalize()
            .expect("Given path to save docker images does not exist.")
            .to_str()
            .unwrap()
            .to_string()
    }

    /// Check if the docker-volume exists.
    pub fn save(&self) -> bool {
        //'docker run --rm --volume {0}:/mybackup -v {1}:/backup alpine sh -c "cd /mybackup && {2} /backup/{3} --strip
        //'docker 1"'

        let filename = self.exported_file_name();
        let path = self.get_volume_path();

        if !docker_volume_exist(&self.volume_name) {
            println!(
                "Requested docker volume `{}` does not exist.",
                self.volume_name
            );
            return false;
        }

        if self.is_interactive {
            if confirm_user_input() {
                println!("Continuing with saving of image");
            } else {
                println!("Abort, current operation has been cancelled.");
                return false;
            }
        }

        let cmd = format!(
            "docker run --rm --volume {}:/mybackup -v {}:/backup \
             alpine tar czvf /backup/{} /mybackup",
            self.volume_name, path, filename
        );

        return execute_command(&cmd);
    }
}
