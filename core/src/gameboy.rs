use crate::char::Charset;
use crate::ocr::{read_character, read_field};
use crate::position::Position;
use crate::roi::Roi;
use image::{DynamicImage, GrayImage, ImageBuffer, Luma};
use imageproc::contours::Contour;
use imageproc::contrast::threshold;
use std::cmp::{max, min};

/// Returns the inclusive bounding box of a contour.
pub fn contour_to_position(contour: &Contour<i32>) -> Result<Position, &str> {
    if contour.points.len() < 1 {
        return Err("Contour contains no points!");
    }

    let curr_point = &contour.points[0];
    let mut x_min = curr_point.x;
    let mut x_max = curr_point.x;
    let mut y_min = curr_point.y;
    let mut y_max = curr_point.y;

    for point in &contour.points[1..] {
        x_min = min(x_min, point.x);
        x_max = max(x_max, point.x);
        y_min = min(y_min, point.y);
        y_max = max(y_max, point.y);
    }

    let width = x_max - x_min + 1;
    let height = y_max - y_min + 1;

    let pos = Position {
        x: x_min as u32,
        y: y_min as u32,
        width: width as u32,
        height: height as u32,
    };
    Ok(pos)
}

/// Returns the possible Game Boy screen positions.
/// Candidates have a minimum size of 160x140 and a ratio of ~10:9.
fn locate_screen_candidates(contours: &Vec<Contour<i32>>) -> Vec<Position> {
    let target_ratio = 10.0 / 9.0;
    let tolerance = 0.01;

    let mut potential_rects: Vec<Position> = Vec::with_capacity(8);
    for contour in contours {
        let bbox = contour_to_position(&contour).unwrap();

        if bbox.width < 160 || bbox.height < 144 {
            continue; // Smaller than original resolution
        }

        let ratio = bbox.width as f32 / bbox.height as f32;
        if (ratio - target_ratio).abs() > tolerance {
            continue; // Not within tolerance
        }

        potential_rects.push(bbox);
    }
    potential_rects
}

/// Returns the position of the Game Boy screen.
///
/// The input input image is converted to grayscale and thresholded right away.
/// Therefore it accepts images of various kinds.
///
/// Known limitation: fails if the input image is the original GameBoy screen (160x140 pixels).
/// The erode is too much for that.
pub fn locate_screen(img: &DynamicImage) -> Option<Position> {
    let img_gray = img.clone().into_luma8();

    let threshold_val = 200;
    let img_threshold = threshold(&img_gray, threshold_val);

    // Unlike OpenCV, the border type can not be set for erode.
    // Would it be set to a constant zero, then erode would create a black border.
    // Find contours need sthe black border as seen in #38
    let border_size = 1; // pixels
    let new_width = img_threshold.width() + 2 * border_size;
    let new_height = img_threshold.height() + 2 * border_size;
    let mut img_border = ImageBuffer::from_pixel(new_width, new_height, Luma([0]));
    for y in 0..img_threshold.height() {
        for x in 0..img_threshold.width() {
            let pixel = img_threshold.get_pixel(x, y);
            img_border.put_pixel(x + border_size, y + border_size, *pixel);
        }
    }

    // Remove little dots to speed up finding contours
    let erode_size = 1;
    let image_erode = imageproc::morphology::erode(
        &img_border,
        imageproc::distance_transform::Norm::LInf,
        erode_size as u8,
    );

    let contours = imageproc::contours::find_contours::<i32>(&image_erode);

    let screen_candidates = locate_screen_candidates(&contours);

    let largest_candidate = screen_candidates
        .iter()
        .max_by_key(|pos| pos.width * pos.height);

    let screen_position = match largest_candidate {
        Some(p) => Some(Position {
            x: p.x - erode_size - border_size,
            y: p.y - erode_size - border_size,
            width: p.width + 2 * erode_size,
            height: p.height + 2 * erode_size,
        }),
        None => None,
    };

    screen_position
}

/// The layout of the stats screen 1.
/// Contains the position of the fields.
pub struct StatScreen1Layout {
    pub width: i32,
    pub height: i32,
    pub pkmn_ndex_pos: Position,
    pub level_field_pos: Position,
    pub hp_field_pos: Position,
    pub attack_field_pos: Position,
    pub defense_field_pos: Position,
    pub speed_field_pos: Position,
    pub special_field_pos: Position,
    pub slash_positions: [Position; 5],
}

impl StatScreen1Layout {
    /// Populates the struct with the known positions.
    /// Beware, text that is of no concern are not populated.
    pub fn new() -> StatScreen1Layout {
        let field_width = 23;
        let field_height = 7;

        let x_pkmn_no = 24;
        let y_pkmn_no = 56;

        let x_level = 120;
        let y_level = 16;

        let x_hp = 150 - field_width + 1;
        let y_hp = 39 - field_height;

        let x_attack = 70 - field_width + 1;
        let y_attack = 87 - field_height;

        let x_defense = 70 - field_width + 1;
        let y_defense = 103 - field_height;

        let x_speed = 70 - field_width + 1;
        let y_speed = 119 - field_height;

        let x_special = 70 - field_width + 1;
        let y_special = 135 - field_height;

        StatScreen1Layout {
            width: 160,
            height: 144,
            pkmn_ndex_pos: Position {
                x: x_pkmn_no,
                y: y_pkmn_no,
                width: field_width,
                height: field_height,
            },
            level_field_pos: Position {
                x: x_level,
                y: y_level,
                width: field_width,
                height: field_height,
            },
            hp_field_pos: Position {
                x: x_hp,
                y: y_hp,
                width: field_width,
                height: field_height,
            },
            attack_field_pos: Position {
                x: x_attack,
                y: y_attack,
                width: field_width,
                height: field_height,
            },
            defense_field_pos: Position {
                x: x_defense,
                y: y_defense,
                width: field_width,
                height: field_height,
            },
            speed_field_pos: Position {
                x: x_speed,
                y: y_speed,
                width: field_width,
                height: field_height,
            },
            special_field_pos: Position {
                x: x_special,
                y: y_special,
                width: field_width,
                height: field_height,
            },
            slash_positions: [
                Position {
                    x: 120,
                    y: 33,
                    width: 7,
                    height: 7,
                },
                Position {
                    x: 120,
                    y: 49,
                    width: 7,
                    height: 7,
                },
                Position {
                    x: 120,
                    y: 73,
                    width: 7,
                    height: 7,
                },
                Position {
                    x: 96,
                    y: 105,
                    width: 7,
                    height: 7,
                },
                Position {
                    x: 96,
                    y: 121,
                    width: 7,
                    height: 7,
                },
            ],
        }
    }

    pub fn verify_layout(&self, img: &GrayImage, chars: &Charset) -> bool {
        if img.width() as i32 != self.width || img.height() as i32 != self.height {
            return false;
        }

        for pos in &self.slash_positions {
            let roi = Roi {
                img: img,
                pos: pos.clone(),
            };

            let char = read_character(&roi, chars);
            let Ok(char) = char else {
                return false; // Char not recognised
            };
            if char != "/" {
                return false; // Not the char we want
            }
        }
        true
    }

    pub fn read_fields(
        &self,
        img: &GrayImage,
        chars: &Charset,
    ) -> Result<StatsSreen1Content, String> {
        if img.width() as i32 != self.width || img.height() as i32 != self.height {
            return Err("Mismatch in image and layout dimensions.".to_string());
        }

        let roi = Roi {
            img: img,
            pos: self.pkmn_ndex_pos.clone(),
        };
        let pkmn_no = read_field(&roi, chars)
            .expect("Failed to read ndex")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse ndex to an i32");

        let roi = Roi {
            img: img,
            pos: self.level_field_pos.clone(),
        };
        let level = read_field(&roi, chars)
            .expect("Failed to read level")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse ndex to an i32");

        let roi = Roi {
            img: img,
            pos: self.hp_field_pos.clone(),
        };
        let hp = read_field(&roi, chars)
            .expect("Failed to read HP")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse HP to an i32");

        let roi = Roi {
            img: img,
            pos: self.attack_field_pos.clone(),
        };
        let attack = read_field(&roi, chars)
            .expect("Failed to read Attack")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse Attack to an i32");

        let roi = Roi {
            img: img,
            pos: self.defense_field_pos.clone(),
        };
        let defense = read_field(&roi, chars)
            .expect("Failed to read Defense")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse Defense to an i32");

        let roi = Roi {
            img: img,
            pos: self.speed_field_pos.clone(),
        };
        let speed = read_field(&roi, chars)
            .expect("Failed to read Speed")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse Speed to an i32");

        let roi = Roi {
            img: img,
            pos: self.special_field_pos.clone(),
        };
        let special = read_field(&roi, chars)
            .expect("Failed to read Special")
            .trim()
            .to_string()
            .parse()
            .expect("Failed to parse Special to an i32");

        let content = StatsSreen1Content {
            pkmn_no,
            level,
            hp,
            attack,
            defense,
            speed,
            special,
        };
        Ok(content)
    }
}

/// The content of the fields present on stats screen 1.
#[derive(PartialEq, PartialOrd, Clone)]
pub struct StatsSreen1Content {
    pub pkmn_no: i32,
    pub level: i32,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub special: i32,
}

pub struct StatScreen2Layout {
    pub width: i32,
    pub height: i32,
    pub pkmn_ndex_pos: Position,
    pub attack_1: Position,
    pub attack_2: Position,
    pub attack_3: Position,
    pub attack_4: Position,
}

impl StatScreen2Layout {
    pub fn new() -> StatScreen2Layout {
        let field_width = 23;
        let field_height = 7;

        let x_pkmn_no = 24;
        let y_pkmn_no = 56;

        StatScreen2Layout {
            width: 160,
            height: 144,
            pkmn_ndex_pos: Position {
                x: x_pkmn_no,
                y: y_pkmn_no,
                width: field_width,
                height: field_height,
            },
            attack_1: Position {
                x: 16,
                y: 72,
                width: 95,
                height: 7,
            },
            attack_2: Position {
                x: 16,
                y: 88,
                width: 95,
                height: 7,
            },
            attack_3: Position {
                x: 16,
                y: 104,
                width: 95,
                height: 7,
            },
            attack_4: Position {
                x: 16,
                y: 120,
                width: 95,
                height: 7,
            },
        }
    }

    pub fn verify_layout(&self, img: &GrayImage, chars: &Charset) -> bool {
        if img.width() as i32 != self.width || img.height() as i32 != self.height {
            return false;
        }

        let roi = Roi {
            img: img,
            pos: Position {
                x: 128,
                y: 81,
                width: 7,
                height: 7,
            },
        };
        let char = read_character(&roi, chars);
        let Ok(char) = char else {
            return false; // Char not recognised
        };
        if char != "/" {
            return false; // Not the char we want
        }
        true
    }

    pub fn read_fields(
        &self,
        img: &GrayImage,
        chars: &Charset,
    ) -> Result<StatsSreen2Content, String> {
        if img.width() as i32 != self.width || img.height() as i32 != self.height {
            return Err("Mismatch in image and layout dimensions.".to_string());
        }

        let roi = Roi {
            img: img,
            pos: self.pkmn_ndex_pos,
        };
        let pkmn_no = read_field(&roi, chars).unwrap().trim().to_string();

        let roi = Roi {
            img: img,
            pos: self.attack_1,
        };
        let attack_1 = read_field(&roi, chars).unwrap().trim().to_string();

        let roi = Roi {
            img: img,
            pos: self.attack_2,
        };
        let attack_2 = read_field(&roi, chars).unwrap().trim().to_string();

        let roi = Roi {
            img: img,
            pos: self.attack_3,
        };
        let attack_3 = read_field(&roi, chars).unwrap().trim().to_string();

        let roi = Roi {
            img: img,
            pos: self.attack_4,
        };
        let attack_4 = read_field(&roi, chars).unwrap().trim().to_string();

        let content = StatsSreen2Content {
            pkmn_no,
            attack_1,
            attack_2,
            attack_3,
            attack_4,
        };
        Ok(content)
    }
}

#[derive(PartialEq, PartialOrd, Clone)]
pub struct StatsSreen2Content {
    pub pkmn_no: String,
    pub attack_1: String,
    pub attack_2: String,
    pub attack_3: String,
    pub attack_4: String,
}

pub struct GscSummary1 {
    pub width: i32,
    pub height: i32,

    pub ndex: Position,
    pub level: Position,

    pub hp: Position,
}

impl GscSummary1 {
    pub fn new() -> GscSummary1 {
        let width = 160;
        let height = 144;

        let ndex = Position {
            x: 80,
            y: 0,
            width: 23,
            height: 7,
        };

        let level = Position {
            x: 120,
            y: 0,
            width: 23,
            height: 7,
        };

        let hp = Position {
            x: 40,
            y: 80,
            width: 23,
            height: 7,
        };

        let layout = GscSummary1 {
            width,
            height,
            ndex,
            level,
            hp,
        };

        layout
    }

    pub fn verify_layout(&self, img: &GrayImage, chars: &Charset) -> bool {
        if img.width() as i32 != self.width || img.height() as i32 != self.height {
            return false;
        }

        let roi = Roi {
            img: img,
            pos: Position {
                // hp divider slash
                x: 32,
                y: 80,
                width: 7,
                height: 7,
            },
        };
        let char = read_character(&roi, chars);
        let Ok(char) = char else {
            return false; // Char not recognised
        };
        if char != "/" {
            return false; // Not the char we want
        }
        true
    }
}

pub struct GscSummary2 {
    pub width: i32,
    pub height: i32,

    pub ndex: Position,
    pub level: Position,

    pub attack_1: Position,
    pub attack_2: Position,
    pub attack_3: Position,
    pub attack_4: Position,
}

impl GscSummary2 {
    pub fn new() -> GscSummary2 {
        let layout = GscSummary2 {
            width: 160,
            height: 144,
            ndex: Position {
                x: 80,
                y: 0,
                width: 23,
                height: 7,
            },
            level: Position {
                x: 120,
                y: 0,
                width: 23,
                height: 7,
            },
            attack_1: Position {
                x: 64,
                y: 80,
                width: 95,
                height: 7,
            },
            attack_2: Position {
                x: 64,
                y: 96,
                width: 95,
                height: 7,
            },
            attack_3: Position {
                x: 64,
                y: 112,
                width: 95,
                height: 7,
            },
            attack_4: Position {
                x: 64,
                y: 128,
                width: 95,
                height: 7,
            },
        };

        layout
    }

    pub fn verify_layout(&self, img: &GrayImage, chars: &Charset) -> bool {
        if img.width() as i32 != self.width || img.height() as i32 != self.height {
            return false;
        }

        let roi = Roi {
            img: img,
            pos: Position {
                // First attack PP divider
                x: 136,
                y: 88,
                width: 7,
                height: 7,
            },
        };
        let char = read_character(&roi, chars);
        let Ok(char) = char else {
            return false; // Char not recognised
        };
        if char != "/" {
            return false; // Not the char we want
        }
        true
    }
}

pub struct GscSummary3 {
    pub width: i32,
    pub height: i32,

    pub ndex: Position,
    pub level: Position,

    pub attack: Position,
    pub defense: Position,
    pub spc_attack: Position,
    pub spc_defense: Position,
    pub speed: Position,
}

impl GscSummary3 {
    pub fn new() -> GscSummary3 {
        let layout = GscSummary3 {
            width: 160,
            height: 144,
            ndex: Position {
                x: 80,
                y: 0,
                width: 23,
                height: 7,
            },
            level: Position {
                x: 120,
                y: 0,
                width: 23,
                height: 7,
            },
            attack: Position {
                x: 136,
                y: 72,
                width: 23,
                height: 7,
            },
            defense: Position {
                x: 136,
                y: 88,
                width: 23,
                height: 7,
            },
            spc_attack: Position {
                x: 136,
                y: 104,
                width: 23,
                height: 7,
            },
            spc_defense: Position {
                x: 136,
                y: 120,
                width: 23,
                height: 7,
            },
            speed: Position {
                x: 136,
                y: 136,
                width: 23,
                height: 7,
            },
        };

        layout
    }

    pub fn verify_layout(&self, img: &GrayImage, chars: &Charset) -> bool {
        if img.width() as i32 != self.width || img.height() as i32 != self.height {
            return false;
        }

        let roi = Roi {
            img: img,
            pos: Position {
                // OT divider slash
                x: 16,
                y: 96,
                width: 7,
                height: 7,
            },
        };
        let char = read_character(&roi, chars);
        let Ok(char) = char else {
            return false; // Char not recognised
        };
        if char != "/" {
            return false; // Not the char we want
        }
        true
    }
}
