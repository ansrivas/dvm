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

use clap::{App, Arg, SubCommand};
use loader::ImageLoader;
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

fn get_interactive_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("interactive")
        .short("i")
        .long("interactive")
        .help("Run the command interactively")
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

    let interactive = get_interactive_arg();

    let app = App::new("dvm")
        .version("0.1.0")
        .about("Showcase different options to backup and load docker volumes")
        .subcommand(
            SubCommand::with_name("save")
                .about("Save your docker volume locally.")
                .arg(volume_save)
                .arg(path_save)
                .arg(&interactive),
        ).subcommand(
            SubCommand::with_name("load")
                .about("Load the locally saved volume to named docker-volume.")
                .arg(volume_load)
                .arg(path_load)
                .arg(&interactive),
        );

    let mut out = Vec::new();
    app.write_help(&mut out)
        .expect("Failed to capture help message");

    match app.get_matches().subcommand() {
        ("save", Some(save_matches)) => {
            let image_saver = ImageSaver::new(save_matches);
            image_saver.save();
        }

        ("load", Some(load_matches)) => {
            let volume = load_matches
                .value_of("volume")
                .unwrap()
                .parse::<String>()
                .expect("Volume name should be a string.");

            let path = load_matches
                .value_of("path")
                .unwrap()
                .parse::<String>()
                .expect("Path should be a string.");

            let is_interactive = load_matches
                .value_of("interactive")
                .unwrap()
                .parse::<bool>()
                .unwrap();

            let image_loader = ImageLoader::new(&volume, &path, is_interactive);

            image_loader.load();
            unimplemented!();
        }

        ("", None) => println!("{}", String::from_utf8(out).unwrap()),

        _ => println!("Please run dvm --help"),
    }
}
