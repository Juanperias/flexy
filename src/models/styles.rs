use anyhow::{anyhow, Result};
use mlua::Table;
use nannou::color::{Rgb, Srgb, BLACK};

pub trait ToColor {
    fn to_color(&self) -> Result<Rgb>;
}

impl ToColor for String {
    fn to_color(&self) -> Result<Rgb> {
        if self.len() != 6 {
            return Err(anyhow!("Error: invalid color format"));
        }
        let red = f32::from(u8::from_str_radix(&self[0..2], 16)?) / 255.0;
        let green = f32::from(u8::from_str_radix(&self[2..4], 16)?) / 255.0;
        let blue = f32::from(u8::from_str_radix(&self[4..6], 16)?) / 255.0;

        Ok(Srgb::new(red, green, blue))
    }
}

#[derive(Debug)]
pub struct Style {
    pub color: Srgb,
    pub font_size: u32,
    pub pos_x: f32,
    pub pos_y: f32,
}

impl Style {
    pub fn new() -> Self {
        Self {
            color: BLACK.into_format(),
            font_size: 10,
            pos_x: 0.0,
            pos_y: 0.0,
        }
    }
    pub fn from_table(table: Table) -> Result<Self> {
        let color_string: String = table.get("color").unwrap_or("black".into());
        let color_rgb = color_string.to_color()?;

        let font_size: u32 = table.get("font_size").unwrap_or(10);
        let pos_x: f32 = table.get("pos_x").unwrap_or(0.0);
        let pos_y: f32 = table.get("pos_y").unwrap_or(0.0);

        Ok(Style {
            color: color_rgb,
            font_size,
            pos_x,
            pos_y,
        })
    }
}
