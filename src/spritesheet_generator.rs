use std::fs::File;

use serde_json;
use image;
use texture_packer::texture::Texture;
use texture_packer::{TexturePacker, TexturePackerConfig};
use texture_packer::exporter::ImageExporter;
use std::io::Write;

use crate::file_texture;
use crate::spritesheet;
use crate::spritesheet_generator_config;

pub fn generate(config: spritesheet_generator_config::SpritesheetGeneratorConfig) {
    // Initial setup
    let input_folder = config.input_folder;
    let output_folder = config.output_folder;
    let output_file_name = config.output_file_name;

    // Perform texture packing
    let config = TexturePackerConfig {
        max_width: config.max_width,
        max_height: config.max_height,
        border_padding: config.border_padding,
        allow_rotation: false,
        texture_outlines: false,
        trim: false,
        ..Default::default()
    };
    let mut packer = TexturePacker::new_skyline(config);
    for file_textures in file_texture::find_all(input_folder) {
        packer.pack_own(file_textures.file.name, file_textures.texture);
    }

    let res = spritesheet::to_ron(
        packer.get_frames(),
        packer.width(),
        packer.height(),
    );
    let ron_path = format!("{}{}.ron", output_folder, output_file_name);
    File::create(ron_path).unwrap().write_all(res.ron.as_bytes()).unwrap();

    let name_to_id_pat = format!("{}{}_name_to_id.json", output_folder, output_file_name);
    File::create(name_to_id_pat).unwrap().write_all(serde_json::to_string(&res.name_to_id).unwrap().as_bytes()).unwrap();

    // Save Image
    let exporter = ImageExporter::export(&packer).unwrap();
    let image_path = format!("{}{}.png", output_folder, output_file_name);
    let mut image_file = File::create(image_path).unwrap();
    exporter.write_to(&mut image_file, image::PNG).unwrap();
}
