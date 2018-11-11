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
      .arg(Arg::with_name("max_width").long("max_width").takes_value(true).help("sets max_width for the resulting spritesheet"))
      .arg(Arg::with_name("max_height").long("max_height").takes_value(true).help("sets max_height for the resulting spritesheet"))
      .arg(Arg::with_name("border_padding").long("border_padding").takes_value(true).help("sets border_padding for the resulting spritesheet"))
      .get_matches();

    let input_folder = matches.value_of("input_folder").unwrap();
    let output_folder = matches.value_of("output_folder").unwrap_or("");
    let output_file_name = matches.value_of("output_file_name").unwrap();

    let max_width = matches.value_of("max_width").unwrap_or("1000");
    let max_height = matches.value_of("max_height").unwrap_or("1000");
    let border_padding = matches.value_of("border_padding").unwrap_or("0");

    let config = SpritesheetGeneratorConfig {
        max_width: max_width.parse().unwrap(),
        max_height: max_height.parse().unwrap(),
        border_padding: border_padding.parse().unwrap(),
        input_folder: input_folder.to_string(),
        output_folder: output_folder.to_string(),
        output_file_name: output_file_name.to_string(),
    };
    println!("Generating spritesheets: {:?}", config);

    generate(config);
    println!("Done.");
}
