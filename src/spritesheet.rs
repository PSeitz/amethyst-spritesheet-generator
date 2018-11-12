use std::collections::{BTreeMap, HashMap};
use serde::{Serialize, Serializer};

use texture_packer;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Screen {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Frame {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub screen: Screen,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Spritesheet {
    #[serde(serialize_with = "ordered_map")]
    pub frames: HashMap<String, Frame>,
}

fn ordered_map<S>(value: &HashMap<String, Frame>, serializer: S) -> Result<S::Ok, S::Error>
where
S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

pub fn to_atlas(
    frames: &HashMap<String, texture_packer::Frame>,
    image_width: u32,
    image_height: u32,
) -> Spritesheet {
    let frames_map = frames
        .iter()
        .map(|(name, frame)| (
                name.clone(),
                Frame {
                    x: frame.frame.x,
                    y: frame.frame.y,
                    w: frame.frame.w,
                    h: frame.frame.h,
                    screen: Screen {
                        x: 1. / (image_width as f32 / frame.frame.x as f32),
                        y: 1. / (image_height as f32 / frame.frame.y as f32),
                        w: 1. / (image_width as f32 / frame.frame.w as f32),
                        h: 1. / (image_height as f32 / frame.frame.h as f32),
                    }
                }
            )
        )
        .collect();

    return Spritesheet { frames: frames_map };
}


#[derive(Debug, Serialize, Deserialize)]
struct Ron {
    spritesheet_width: f32,
    spritesheet_height: f32,
    sprites: Vec<SpriteRon>
}

#[derive(Debug, Serialize, Deserialize)]
struct SpriteRon {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}
#[derive(Debug, Clone)]
pub struct RonResult {
    pub name_to_id: HashMap<String, u32>,
    pub ron: String,
}

pub fn to_ron(
    frames: &HashMap<String, texture_packer::Frame>,
    image_width: u32,
    image_height: u32,
) -> RonResult {
    let mut i_to_name = vec![];

    let mut frames:Vec<_> = frames.iter().collect();
    frames.sort_by_key(|el|el.1.frame.x);

    let frames_map = frames
        .iter()
        .map(|el| {
            // let (name, frame)
            let name = el.0;
            let frame = el.1;
            i_to_name.push(name.to_string());
            SpriteRon {
                x: frame.frame.x as f32,
                y: frame.frame.y as f32,
                width: frame.frame.w as f32,
                height: frame.frame.h as f32,
            }
        })
        .collect();
    // let frames_map = frames
    //     .iter()
    //     .map(|el| {
    //         // let (name, frame)
    //         let name = el.0;
    //         let frame = el.1;
    //         i_to_name.push(name.to_string());
    //         SpriteRon {
    //             x: frame.frame.x as f32,
    //             y: frame.frame.y as f32,
    //             width: frame.frame.w as f32,
    //             height: frame.frame.h as f32,
    //         }
    //     })
    //     .collect();

    let name_to_id: HashMap<String, u32> =  i_to_name.iter().enumerate().map(|(i, name)|(name.to_string(), i as u32)).collect();

    // println!("    fn sprite_id_for_name(name: &str) -> usize {{");
    // for (i, name) in i_to_name.iter().enumerate() {
    //     println!("        if name==\"{}\"{{",name );
    //     println!("            return {};", i);
    //     println!("        }}");
    // }
    // println!(r#"        panic!("sprite for name not found {{:?}}", name);"#);
    // println!("    }}");
    let ron = Ron { 
        spritesheet_width: image_width as f32,
        spritesheet_height: image_height as f32,
        sprites: frames_map
    };

    // (ron::ser::to_string_pretty(&ron, ron::ser::PrettyConfig::default()).unwrap(), map)

    RonResult{
        name_to_id,
        ron: ron::ser::to_string_pretty(&ron, ron::ser::PrettyConfig::default()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_to_atlas() {
        let mut converted_frames: HashMap<String, texture_packer::Frame> = HashMap::new();
        converted_frames.insert(
            "test1".to_string(),
            texture_packer::Frame {
                key: "test1".to_string(),
                frame: texture_packer::Rect{ x: 0, y: 0, w: 10, h: 50},
                source: texture_packer::Rect{ x: 0, y: 0, w: 10, h: 50},
                rotated: false,
                trimmed: false,
            }
        );
        let atlas = to_atlas(&converted_frames, 100, 100);

        let mut created_frames: HashMap<String, Frame> = HashMap::new();
        created_frames.insert(
            "test1".to_string(),
            Frame {
                x: 0, y: 0, w: 10, h: 50, screen: Screen { x: 0., y: 0., w: 0.1, h: 0.5 }
            }
        );
        created_frames.insert(
            "test2".to_string(),
            Frame {
                x: 1, y: 1, w: 1, h: 1, screen: Screen { x: 0., y: 0., w: 0., h: 0. }
            }
        );

        let converted = atlas.frames.get("test1").unwrap();
        let created = created_frames.get("test1").unwrap();

        assert_eq!(converted.x, created.x);
        assert_eq!(converted.y, created.y);
        assert_eq!(converted.w, created.w);
        assert_eq!(converted.h, created.h);
        assert_eq!(converted.screen.x, created.screen.x);
        assert_eq!(converted.screen.y, created.screen.y);
        assert_eq!(converted.screen.w, created.screen.w);
        assert_eq!(converted.screen.h, created.screen.h);
    }
}
