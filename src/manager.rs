use clap::{App, Arg, SubCommand};
use saver::ImageSaver;

pub fn run() {

    let volume_save = Arg::with_name("volume")
        .short("v")
        .long("volume")
        .help("The named volume to save locally.")
        .takes_value(true)
        .required(true);


    let volume_load = Arg::with_name("volume")
        .short("v")
        .long("volume")
        .help("The target name of volume where the zip is to be uploaded.")
        .takes_value(true)
        .required(true);

    let interactive = Arg::with_name("interactive")
        .short("i")
        .long("interactive")
        .help("Run the command interactively")
        .takes_value(true)
        .default_value("true")
        .possible_values(&["true", "false"])
        .required(true);

    let path = Arg::with_name("path")
        .short("p")
        .long("path")
        .help("The path to save the docker volume as .tar.gz")
        .takes_value(true)
        .default_value(".")
        .required(true);

    let app = App::new("dvm")
        .version("0.1.0")
        .about("Showcase different options to backup and load docker volumes")
        .subcommand(SubCommand::with_name("save")
                        .about("Save your docker volume locally.")
                        .arg(volume_save)
                        .arg(&path)
                        .arg(&interactive))
        .subcommand(SubCommand::with_name("load")
                        .about("Load the locally saved volume to named docker-volume.")
                        .arg(volume_load)
                        .arg(&path)
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
