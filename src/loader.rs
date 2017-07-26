use std::collections::HashMap;
use std::path::Path;
use std::ffi::OsStr;

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
        unimplemented!();
    }



    fn extension_vs_commands() -> HashMap<&'static str, &'static str> {

        let mut ext_cmd = HashMap::new();

        ext_cmd.insert("rar", "unrar x");
        ext_cmd.insert("gz", "gunzip");
        ext_cmd.insert("zip", "unzip");
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

        // im_loader = ImageLoader::new("test-vol", "./test-vol_2017-07-23_095003.tar.gz", true);
        // assert_eq!(Path::new("./test-vol_2017-07-23_095003.tar.gz").extension(),
        //            "");

    }
}
