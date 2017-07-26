use std::collections::HashMap;
use std::path::Path;
use std::ffi::OsStr;
use utils::{confirm_user_input, docker_volume_exist};

pub struct ImageLoader {
    volume_name: String,
    image_path: String,
    is_interactive: bool,
    command_list: HashMap<&'static str, &'static str>,
}

impl ImageLoader {
    pub fn new(volume: &str, path: &str, is_interactive: bool) -> Self {

        ImageLoader {
            volume_name: volume.to_string(),
            image_path: path.to_string(),
            is_interactive: is_interactive,
            command_list: ImageLoader::extension_vs_commands(),
        }
    }

    pub fn load(&self) {

        // Check if the intended docker volume is already present.
        if docker_volume_exist(&self.volume_name) {
            println!("Requested docker volume `{}` already exists.", volume);
            if confirm_user_input() {
                println!("Continuing with loading of image");
            } else {
                println!("Abort, current operation has been cancelled.");
                return false;
            }
        }

        let file_ext = match self.get_extension_from_filename() {
            Some(ext) => ext,
            None => {
                println!("Unable to extract the file extension");
                return;
            },
        };

        if let Some(extract_command) = self.command_list.get(file_ext) {
            let cmd =
                format!("docker run --rm --volume {}:/mybackup -v {}:/backup alpine sh \
            -c \"cd /mybackup && {} /backup/{} --strip 1\"",
            volume,
            path,
            extract_command
            filename);


            let vec = cmd.as_str().split(' ');
            let splitted: Vec<&str> = vec.collect();


            let status = Command::new(splitted[0])
                .args(&splitted[1..])
                .status()
                .expect("Docker volume command failed to start");

            status.success()

        } else {
            println!("Abort, current file extension is not supported.");
            return false;
        }


        unimplemented!();
    }



    fn extension_vs_commands() -> HashMap<&'static str, &'static str> {

        let mut ext_cmd = HashMap::new();

        ext_cmd.insert("gz", "gunzip");
        ext_cmd.insert("zip", "unzip");
        ext_cmd.insert("rar", "unrar x");
        ext_cmd.insert("tar", "tar xvf");
        ext_cmd.insert("tgz", "tar xvzf");
        ext_cmd.insert("tbz2", "tar xvjf");
        ext_cmd
    }

    fn get_extension_from_filename(&self) -> Option<String> {

        //Change it to a canonical file path.
        let path = Path::new(&self.image_path)
            .canonicalize()
            .expect("Expecting a correct filename");
        path.extension().and_then(OsStr::to_str).map(String::from)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let mut im_loader = ImageLoader::new("test-vol", "./test-vol_2017-07-23_095003.tar.gz", true);
        assert!(im_loader.get_extension_from_filename() == Some("gz".to_string()));
        im_loader = ImageLoader::new("test-vol", "./test-vol_2017-07-23_095003.gz", true);
        assert!(im_loader.get_extension_from_filename() == Some("gz".to_string()));
        im_loader = ImageLoader::new("test-vol", "../test-vol_2017-07-23_095003.gz", true);
        assert!(im_loader.get_extension_from_filename() == Some("gz".to_string()));
    }
}
