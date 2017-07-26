use chrono::prelude::*;
use clap::ArgMatches;
use std::path::PathBuf;
use std::process::Command;
use utils::{confirm_user_input, docker_volume_exist};

pub struct ImageSaver {
    volume_name: String,
    path_to_export_vol: String,
    is_interactive: bool,
}

impl ImageSaver {
    pub fn new(save_matches: &ArgMatches) -> Self {

        let volume = save_matches.value_of("volume")
                                 .unwrap()
                                 .parse::<String>()
                                 .expect("Volume name should be a string.");

        let path = save_matches.value_of("path")
                               .unwrap()
                               .parse::<String>()
                               .expect("Path should be a string.");

        let is_interactive = save_matches.value_of("interactive")
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
        let exporting_time = Local::now().format("%Y-%m-%d_%H%M%S").to_string();
        format!("{}_{}{}", self.volume_name, exporting_time, ".tgz")

    }

    fn get_volume_path(&self) -> String {

        let download_path = PathBuf::from(&self.path_to_export_vol);
        download_path.canonicalize()
                     .expect("Given path to save docker images does not exist.")
                     .to_str()
                     .unwrap()
                     .to_string()
    }

    /// Check if the docker-volume exists.
    pub fn save(&self) -> bool {
        //'docker run --rm --volume {0}:/mybackup -v {1}:/backup alpine sh -c "cd /mybackup && {2} /backup/{3} --strip
        //'docker 1"'
        let volume = &self.volume_name;
        let filename = self.exported_file_name();
        let path = self.get_volume_path();

        if !docker_volume_exist(volume) {
            println!("Requested docker volume `{}` does not exist.", volume);
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
            volume,
            path,
            filename
        );

        let vec = cmd.as_str().split(' ');
        let splitted: Vec<&str> = vec.collect();


        let status = Command::new(splitted[0])
            .args(&splitted[1..])
            .status()
            .expect("Docker volume command failed to start");

        status.success()
    }
}
