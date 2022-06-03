use piston_window::types;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub const WHITE : Color = Color { r: 255, g: 255, b: 255 };
    pub const BLACK : Color = Color { r: 0, g: 0, b: 0 };
    pub const RED : Color = Color { r: 255, g: 51, b: 51 };
    pub const BLUE : Color = Color { r: 51, g: 51, b: 255 };
    pub const GREEN : Color = Color { r: 0, g: 153, b: 0 };
    pub const ORANGE : Color = Color { r: 255, g: 128, b: 0 };
    pub const YELLOW : Color = Color { r: 255, g: 255, b: 0 };
    pub const PURPLE : Color = Color { r: 102, g: 0, b: 204 };
    pub const PINK : Color = Color { r: 255, g: 0, b: 255 };
    pub const AQUAMARINE : Color = Color { r: 128, g: 128, b: 128 };
    pub const LILA : Color = Color { r: 204, g: 153, b: 255 };
    pub const GRAY : Color = Color { r: 192, g: 192, b: 192 };
}

impl From<Color> for types::Color {
    fn from(color: Color) -> Self {
        [color.r as f32/255.0, 
         color.g as f32 /255.0, 
         color.b as f32 /255.0, 
         1.0]
    }
}

impl From<types::Color> for Color {
    fn from(color: types::Color) -> Self {
       Color { 
            r: (255.0 * color[0]) as u8, 
            g: (255.0 * color[1]) as u8, 
            b: (255.0 * color[2]) as u8 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    
    #[rstest]
    #[case(Color::BLACK,[0.0,0.0,0.0,1.0])]
    #[case(Color::WHITE,[1.0;4])]
    #[case(Color::BLUE,[0.2,0.2,1.0,1.0])]
    fn color_should_map_into_piston_color(#[case] color : Color, #[case] expected_piston_color : piston_window::types::Color) {
        let piston_color = piston_window::types::Color::from(color);
        assert_eq!(piston_color, expected_piston_color);
    }
    
    #[rstest]
    #[case([0.0;4], Color::BLACK)]
    #[case([1.0;4], Color::WHITE)]
    #[case([0.2,0.2,1.0,1.0], Color::BLUE)]
    fn color_should_map_from_piston_color(#[case] piston_color : piston_window::types::Color, #[case] expected_color : Color) {
        let color : Color = piston_color.into();
        assert_eq!(color, expected_color);
    }
}