extern crate spritesheet_generator;

use spritesheet_generator::spritesheet_generator::generate;
use spritesheet_generator::spritesheet_generator_config::SpritesheetGeneratorConfig;
use clap::{Arg, App};

fn main() {

    let matches = App::new("Sprite Sheet generator")
      .version("1.0")
      .arg(Arg::with_name("input_folder")
           .short("i")
           .long("input_folder")
           .help("Sets input_folder, ignores files which can not be handled")
           .required(true)
           .takes_value(true))
      .arg(Arg::with_name("output_file_name")
           .short("o")
           .long("output_file_name")
           .required(true)
           .help("output_file_name, will create a output_file_name.png and output_file_name.png")
           .takes_value(true))
      .arg(Arg::with_name("output_folder")
           .short("o")
           .long("output_folder")
           .takes_value(true))
      .get_matches();

    let input_folder = matches.value_of("input_folder").unwrap();
    let output_folder = matches.value_of("output_folder").unwrap_or("");
    let output_file_name = matches.value_of("output_file_name").unwrap();
    let args:Vec<_> = std::env::args().collect();

    println!("input_folder {:?}", args[1]);
    println!("output_folder {:?}", args[2]);
    let config = SpritesheetGeneratorConfig {
        max_width: 500,
        max_height: 500,
        border_padding: 0,
        input_folder: input_folder.to_string(),
        output_folder: output_folder.to_string(),
        output_file_name: output_file_name.to_string(),
    };
    println!("Generating spritesheets: {:?}", config);

    generate(config);
    println!("Done.");
}
