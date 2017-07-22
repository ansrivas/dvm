use clap::{App, Arg, SubCommand};
use saver::ImageSaver;


fn get_path_arg(help: &str) -> Arg {

    Arg::with_name("path")
        .short("p")
        .long("path")
        .help(help)
        .takes_value(true)
        .default_value(".")
        .required(true)
}

fn get_volume_arg(help: &str) -> Arg {
    Arg::with_name("volume")
        .short("v")
        .long("volume")
        .help(help)
        .takes_value(true)
        .required(true)
}

fn get_interactive_arg(help: &str) -> Arg {

    Arg::with_name("interactive")
        .short("i")
        .long("interactive")
        .help(help)
        .takes_value(true)
        .default_value("true")
        .possible_values(&["true", "false"])
        .required(true)
}


pub fn run() {

    let volume_save = get_volume_arg("The named volume to save locally.");
    let volume_load = get_volume_arg("The target name of volume where the zip is to be uploaded.");

    let path_save = get_path_arg("The path to save the docker volume as .tar.gz");
    let path_load = get_path_arg("The path to load the docker volume from");

    let interactive = get_interactive_arg("Run the command interactively");

    let app = App::new("dvm")
        .version("0.1.0")
        .about("Showcase different options to backup and load docker volumes")
        .subcommand(SubCommand::with_name("save")
                        .about("Save your docker volume locally.")
                        .arg(volume_save)
                        .arg(path_save)
                        .arg(&interactive))
        .subcommand(SubCommand::with_name("load")
                        .about("Load the locally saved volume to named docker-volume.")
                        .arg(volume_load)
                        .arg(path_load)
                        .arg(&interactive));

    let mut out = Vec::new();
    app.write_help(&mut out)
       .expect("Failed to capture help message");


    match app.get_matches().subcommand() {
        ("save", Some(save_matches)) => {
            let image_saver = ImageSaver::new(save_matches);
            image_saver.save();
        },

        ("load", Some(load_matches)) => {
            //TODO: implement loading functionality
        },

        ("", None) => println!("{}", String::from_utf8(out).unwrap()),

        _ => println!("Please run dvm --help"),
    }
}
