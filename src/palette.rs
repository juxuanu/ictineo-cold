mod utils {
    use iced::Color;

    // Function to normalize RGBA values from 0-255 to 0-1.0
    pub const fn normalize(r: u8, g: u8, b: u8, a: f32) -> Color {
        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a,
        }
    }
}

pub(crate) mod light_palette {
    use crate::palette::utils::normalize;
    use iced::Color;

    // Accent Colors
    pub(crate) const ACCENT: Color = normalize(28, 113, 216, 1.0);
    pub(crate) const ACCENT_BG: Color = normalize(53, 132, 228, 1.0);
    pub(crate) const ACCENT_FG: Color = normalize(255, 255, 255, 1.0);

    // Destructive Colors
    pub(crate) const DESTRUCTIVE: Color = normalize(192, 28, 40, 1.0);
    pub(crate) const DESTRUCTIVE_BG: Color = normalize(224, 27, 36, 1.0);
    pub(crate) const DESTRUCTIVE_FG: Color = normalize(255, 255, 255, 1.0);

    // Success Colors
    pub(crate) const SUCCESS: Color = normalize(27, 133, 83, 1.0);
    pub(crate) const SUCCESS_BG: Color = normalize(46, 194, 126, 1.0);
    pub(crate) const SUCCESS_FG: Color = normalize(255, 255, 255, 1.0);

    // Warning Colors
    pub(crate) const WARNING: Color = normalize(156, 110, 3, 1.0);
    pub(crate) const WARNING_BG: Color = normalize(229, 165, 10, 1.0);
    pub(crate) const WARNING_FG: Color = normalize(0, 0, 0, 0.8);

    // View Colors
    pub(crate) const VIEW_FG: Color = normalize(255, 255, 255, 1.0);
    pub(crate) const VIEW_BG: Color = normalize(0, 0, 0, 0.8);
}

pub(crate) mod dark_palette {
    use crate::palette::utils::normalize;
    use iced::Color;

    // Accent Colors
    pub(crate) const ACCENT: Color = normalize(120, 174, 237, 1.0);
    pub(crate) const ACCENT_BG: Color = normalize(53, 132, 228, 1.0);
    pub(crate) const ACCENT_FG: Color = normalize(255, 255, 255, 1.0);

    // Destructive Colors
    pub(crate) const DESTRUCTIVE: Color = normalize(255, 123, 99, 1.0);
    pub(crate) const DESTRUCTIVE_BG: Color = normalize(192, 28, 40, 1.0);
    pub(crate) const DESTRUCTIVE_FG: Color = normalize(255, 255, 255, 1.0);

    // Success Colors
    pub(crate) const SUCCESS: Color = normalize(143, 240, 164, 1.0);
    pub(crate) const SUCCESS_BG: Color = normalize(38, 162, 105, 1.0);
    pub(crate) const SUCCESS_FG: Color = normalize(255, 255, 255, 1.0);

    // View Colors
    pub(crate) const VIEW_FG: Color = normalize(0, 0, 0, 0.8);
    pub(crate) const VIEW_BG: Color = normalize(30, 30, 30, 1.0);
}
