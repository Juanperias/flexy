use std::path::PathBuf;

use anyhow::{anyhow, Result};
use mlua::Table;
use sdl2::pixels::Color;

use crate::utils::font_util;

pub trait ToColor {
    fn to_color(&self) -> Result<Color>;
}

impl ToColor for String {
    fn to_color(&self) -> Result<Color> {
        if self.len() != 6 {
            return Err(anyhow!("Error: invalid color format"));
        }
        let red = u8::from_str_radix(&self[0..2], 16)?;
        let green = u8::from_str_radix(&self[2..4], 16)?;
        let blue = u8::from_str_radix(&self[4..6], 16)?;
        Ok(Color::RGB(red, green, blue))
    }
}

#[derive(Debug, Clone)]
pub struct Style {
    pub color: Color,
    pub font_size: u16,
    pub pos_x: i32,
    pub pos_y: i32,
    pub font: PathBuf,
}

impl Style {
    pub fn new() -> Self {
        let font = font_util::get("Arial").expect(
            "Arial font not found try downloading it or specifying an already downloaded font",
        );

        Self {
            color: Color::RGB(0, 0, 0),
            font_size: 10,
            pos_x: 0,
            pos_y: 0,
            font,
        }
    }
    pub fn from_table(table: &Table) -> Result<Self> {
        let color_string: String = table.get("color").unwrap_or("ffffff".into());
        let color_rgb = color_string.to_color()?;

        let font_size: u16 = table.get("font_size").unwrap_or(10);
        let pos_x: i32 = table.get("pos_x").unwrap_or(0);
        let pos_y: i32 = table.get("pos_y").unwrap_or(0);
        let font_name: Option<String> = table.get("font").unwrap_or(None);

        let font: PathBuf;

        if let Some(safe_font_name) = font_name {
            font = font_util::get(&safe_font_name)?;
        } else {
            font = font_util::get("Arial").expect(
                "Arial font not found try downloading it or specifying an already downloaded font",
            );
        }

        Ok(Style {
            color: color_rgb,
            font_size,
            pos_x,
            pos_y,
            font,
        })
    }
}
