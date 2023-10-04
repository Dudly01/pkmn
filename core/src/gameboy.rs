use crate::char::Charset;
use crate::ocr::{read_char, read_field};
use crate::position::Position;
use crate::roi::Roi;
use image::{DynamicImage, GrayImage, Luma};
use imageproc::contours::Contour;
use imageproc::contrast::threshold_mut;

/// Searches and returns the possible screen positions for Pokemon RBY.
///
/// Note that the all-white border of the summary screen may prevent the
/// border to be found. Add a black padding or a marker pixel to prevent that.
pub fn search_screen_rby(contours: &Vec<Contour<i32>>) -> Vec<Position> {
    let width_orig = 160;
    let height_orig = 144;

    // Look for rectangle
    let target_ratio = width_orig as f32 / height_orig as f32;
    let tolerance = 0.01;

    let mut candidates: Vec<Position> = Vec::with_capacity(8);
    for contour in contours {
        let bbox = Position::try_from(contour).expect("Could not create Position");

        if bbox.width < width_orig || bbox.height < height_orig {
            continue; // Smaller than original resolution
        }

        let ratio = bbox.width as f32 / bbox.height as f32;
        if (ratio - target_ratio).abs() > tolerance {
            continue; // Not within tolerance
        }

        candidates.push(bbox);
    }
    candidates
}

/// Searches and returns the possible screen positions for Pokemon GSC.
pub fn search_screen_gsc(contours: &Vec<Contour<i32>>) -> Vec<Position> {
    let width_orig = 160;
    let height_orig = 62;

    // Look for rectangle
    let target_ratio = width_orig as f32 / height_orig as f32;
    let tolerance = 0.01;

    let mut candidates: Vec<Position> = Vec::with_capacity(8);
    for contour in contours {
        let mut bbox = Position::try_from(contour).expect("Could not create Position");

        if bbox.width < width_orig || bbox.height < height_orig {
            continue; // Smaller than original resolution
        }

        let ratio = bbox.width as f32 / bbox.height as f32;
        if (ratio - target_ratio).abs() > tolerance {
            continue; // Not within tolerance
        }

        // Extrapolate full screen position
        let estimated_height = bbox.width as f32 * 144.0 / 160.0;
        let estimated_height = estimated_height as u32;

        bbox.height = estimated_height;

        candidates.push(bbox);
    }
    candidates
}

/// Returns the position of the biggest Game Boy screen on the image.
///
/// Works with the Summary screens of RBY and GSC.
pub fn locate_screen(img: &DynamicImage) -> Option<Position> {
    let mut img = img.to_luma8();

    let threshold_val = 140; // Can be anything in [30, 240]
    threshold_mut(&mut img, threshold_val);

    // find_contours() does not find the border on an all-white image.
    // Add black marker pixel as a work-around.
    *img.get_pixel_mut_checked(0, 0)
        .expect("Image has no pixels") = Luma([0]);

    let contours = imageproc::contours::find_contours::<i32>(&img);

    let rby_candidates = search_screen_rby(&contours);
    let gsc_candidates = search_screen_gsc(&contours);

    let biggest = gsc_candidates
        .iter()
        .chain(rby_candidates.iter())
        .max_by_key(|&p| p.width * p.height);

    match biggest {
        Some(a) => Some(*a),
        None => None,
    }
}

pub struct RbySummary1 {
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

impl RbySummary1 {
    /// Populates the struct with the known positions.
    /// Beware, text that is of no concern are not populated.
    pub fn new() -> RbySummary1 {
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

        RbySummary1 {
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

            let char = read_char(&roi, chars);
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
    ) -> Result<RbySummaryContent, String> {
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

        let content = RbySummaryContent {
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
pub struct RbySummaryContent {
    pub pkmn_no: i32,
    pub level: i32,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub special: i32,
}

pub struct RbySummary2 {
    pub width: i32,
    pub height: i32,
    pub pkmn_ndex_pos: Position,
    pub attack_1: Position,
    pub attack_2: Position,
    pub attack_3: Position,
    pub attack_4: Position,
}

impl RbySummary2 {
    pub fn new() -> RbySummary2 {
        let field_width = 23;
        let field_height = 7;

        let x_pkmn_no = 24;
        let y_pkmn_no = 56;

        RbySummary2 {
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
        let char = read_char(&roi, chars);
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
    ) -> Result<RbySummaryContent3, String> {
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

        let content = RbySummaryContent3 {
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
pub struct RbySummaryContent3 {
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
        let char = read_char(&roi, chars);
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
        let char = read_char(&roi, chars);
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
        let char = read_char(&roi, chars);
        let Ok(char) = char else {
            return false; // Char not recognised
        };
        if char != "/" {
            return false; // Not the char we want
        }
        true
    }
}
