extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

extern crate regex;
extern crate image;
extern crate texture_packer;

mod file_data;
mod file_texture;

pub mod spritesheet;
pub mod spritesheet_generator;
pub mod spritesheet_generator_config;

pub use self::spritesheet_generator::generate;
pub use self::spritesheet_generator_config::SpritesheetGeneratorConfig;
