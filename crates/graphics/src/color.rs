#[repr(C)]
#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    const RBGA_U8_FACTOR: f32 = 255.0f32.recip();

    pub const fn rbga_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: (r as f32) * Self::RBGA_U8_FACTOR,
            g: (g as f32) * Self::RBGA_U8_FACTOR,
            b: (b as f32) * Self::RBGA_U8_FACTOR,
            a: (a as f32) * Self::RBGA_U8_FACTOR,
        }
    }

    pub fn oklch(l: f32, c: f32, h: f32) -> Self {
        let h_rad = h.to_radians();
        let a = c * h_rad.cos();
        let b = c * h_rad.sin();

        let l_ = l + 0.396_337_78 * a + 0.215_803_76 * b;
        let m_ = l - 0.105_561_346 * a - 0.063_854_17 * b;
        let s_ = l - 0.089_484_18 * a - 1.291_485_5 * b;

        let l3 = l_ * l_ * l_;
        let m3 = m_ * m_ * m_;
        let s3 = s_ * s_ * s_;

        let r_lin = 4.076_741_7 * l3 - 3.307_711_6 * m3
            + 0.230_969_94 * s3;
        let g_lin = -1.268_438 * l3 + 2.609_757_4 * m3
            - 0.341_319_38 * s3;
        let b_lin = -0.0041960863 * l3 - 0.703_418_6 * m3
            + 1.707_614_7 * s3;

        let r = if r_lin <= 0.0031308 {
            12.92 * r_lin
        } else {
            1.055 * r_lin.powf(1.0 / 2.4) - 0.055
        };

        let g = if g_lin <= 0.0031308 {
            12.92 * g_lin
        } else {
            1.055 * g_lin.powf(1.0 / 2.4) - 0.055
        };

        let b = if b_lin <= 0.0031308 {
            12.92 * b_lin
        } else {
            1.055 * b_lin.powf(1.0 / 2.4) - 0.055
        };

        Self { r, g, b, a: 1.0 }
    }
}

impl Color {
    // Web (CSS) named colors:
    pub const ALICE_BLUE: Self =
        Self::rbga_u8(240, 248, 255, 255);
    pub const ANTIQUE_WHITE: Self =
        Self::rbga_u8(250, 235, 215, 255);
    pub const AQUA: Self = Self::rbga_u8(0, 255, 255, 255);
    pub const AQUAMARINE: Self =
        Self::rbga_u8(127, 255, 212, 255);
    pub const AZURE: Self = Self::rbga_u8(240, 255, 255, 255);
    pub const BEIGE: Self = Self::rbga_u8(245, 245, 220, 255);
    pub const BISQUE: Self = Self::rbga_u8(255, 228, 196, 255);
    pub const BLACK: Self = Self::rbga_u8(0, 0, 0, 255);
    pub const BLANCHED_ALMOND: Self =
        Self::rbga_u8(255, 235, 205, 255);
    pub const BLUE: Self = Self::rbga_u8(0, 0, 255, 255);
    pub const BLUE_VIOLET: Self =
        Self::rbga_u8(138, 43, 226, 255);
    pub const BROWN: Self = Self::rbga_u8(165, 42, 42, 255);
    pub const BURLY_WOOD: Self =
        Self::rbga_u8(222, 184, 135, 255);
    pub const CADET_BLUE: Self =
        Self::rbga_u8(95, 158, 160, 255);
    pub const CHARTREUSE: Self =
        Self::rbga_u8(127, 255, 0, 255);
    pub const CHOCOLATE: Self =
        Self::rbga_u8(210, 105, 30, 255);
    pub const CORAL: Self = Self::rbga_u8(255, 127, 80, 255);
    pub const CORNFLOWER_BLUE: Self =
        Self::rbga_u8(100, 149, 237, 255);
    pub const CORNSILK: Self =
        Self::rbga_u8(255, 248, 220, 255);
    pub const CRIMSON: Self = Self::rbga_u8(220, 20, 60, 255);
    pub const CYAN: Self = Self::rbga_u8(0, 255, 255, 255);
    pub const DARK_BLUE: Self = Self::rbga_u8(0, 0, 139, 255);
    pub const DARK_CYAN: Self = Self::rbga_u8(0, 139, 139, 255);
    pub const DARK_GOLDENROD: Self =
        Self::rbga_u8(184, 134, 11, 255);
    pub const DARK_GRAY: Self =
        Self::rbga_u8(169, 169, 169, 255);
    pub const DARK_GREY: Self =
        Self::rbga_u8(169, 169, 169, 255);
    pub const DARK_GREEN: Self = Self::rbga_u8(0, 100, 0, 255);
    pub const DARK_KHAKI: Self =
        Self::rbga_u8(189, 183, 107, 255);
    pub const DARK_MAGENTA: Self =
        Self::rbga_u8(139, 0, 139, 255);
    pub const DARK_OLIVE_GREEN: Self =
        Self::rbga_u8(85, 107, 47, 255);
    pub const DARK_ORANGE: Self =
        Self::rbga_u8(255, 140, 0, 255);
    pub const DARK_ORCHID: Self =
        Self::rbga_u8(153, 50, 204, 255);
    pub const DARK_RED: Self = Self::rbga_u8(139, 0, 0, 255);
    pub const DARK_SALMON: Self =
        Self::rbga_u8(233, 150, 122, 255);
    pub const DARK_SEA_GREEN: Self =
        Self::rbga_u8(143, 188, 143, 255);
    pub const DARK_SLATE_BLUE: Self =
        Self::rbga_u8(72, 61, 139, 255);
    pub const DARK_SLATE_GRAY: Self =
        Self::rbga_u8(47, 79, 79, 255);
    pub const DARK_SLATE_GREY: Self =
        Self::rbga_u8(47, 79, 79, 255);
    pub const DARK_TURQUOISE: Self =
        Self::rbga_u8(0, 206, 209, 255);
    pub const DARK_VIOLET: Self =
        Self::rbga_u8(148, 0, 211, 255);
    pub const DEEP_PINK: Self =
        Self::rbga_u8(255, 20, 147, 255);
    pub const DEEP_SKY_BLUE: Self =
        Self::rbga_u8(0, 191, 255, 255);
    pub const DIM_GRAY: Self =
        Self::rbga_u8(105, 105, 105, 255);
    pub const DIM_GREY: Self =
        Self::rbga_u8(105, 105, 105, 255);
    pub const DODGER_BLUE: Self =
        Self::rbga_u8(30, 144, 255, 255);
    pub const FIREBRICK: Self = Self::rbga_u8(178, 34, 34, 255);
    pub const FLORAL_WHITE: Self =
        Self::rbga_u8(255, 250, 240, 255);
    pub const FOREST_GREEN: Self =
        Self::rbga_u8(34, 139, 34, 255);
    pub const FUCHSIA: Self = Self::rbga_u8(255, 0, 255, 255);
    pub const GAINSBORO: Self =
        Self::rbga_u8(220, 220, 220, 255);
    pub const GHOST_WHITE: Self =
        Self::rbga_u8(248, 248, 255, 255);
    pub const GOLD: Self = Self::rbga_u8(255, 215, 0, 255);
    pub const GOLDENROD: Self =
        Self::rbga_u8(218, 165, 32, 255);
    pub const GRAY: Self = Self::rbga_u8(128, 128, 128, 255);
    pub const GREY: Self = Self::rbga_u8(128, 128, 128, 255);
    pub const GREEN: Self = Self::rbga_u8(0, 128, 0, 255);
    pub const GREEN_YELLOW: Self =
        Self::rbga_u8(173, 255, 47, 255);
    pub const HONEYDEW: Self =
        Self::rbga_u8(240, 255, 240, 255);
    pub const HOT_PINK: Self =
        Self::rbga_u8(255, 105, 180, 255);
    pub const INDIAN_RED: Self =
        Self::rbga_u8(205, 92, 92, 255);
    pub const INDIGO: Self = Self::rbga_u8(75, 0, 130, 255);
    pub const IVORY: Self = Self::rbga_u8(255, 255, 240, 255);
    pub const KHAKI: Self = Self::rbga_u8(240, 230, 140, 255);
    pub const LAVENDER: Self =
        Self::rbga_u8(230, 230, 250, 255);
    pub const LAVENDER_BLUSH: Self =
        Self::rbga_u8(255, 240, 245, 255);
    pub const LAWN_GREEN: Self =
        Self::rbga_u8(124, 252, 0, 255);
    pub const LEMON_CHIFFON: Self =
        Self::rbga_u8(255, 250, 205, 255);
    pub const LIGHT_BLUE: Self =
        Self::rbga_u8(173, 216, 230, 255);
    pub const LIGHT_CORAL: Self =
        Self::rbga_u8(240, 128, 128, 255);
    pub const LIGHT_CYAN: Self =
        Self::rbga_u8(224, 255, 255, 255);
    pub const LIGHT_GOLDENROD_YELLOW: Self =
        Self::rbga_u8(250, 250, 210, 255);
    pub const LIGHT_GRAY: Self =
        Self::rbga_u8(211, 211, 211, 255);
    pub const LIGHT_GREY: Self =
        Self::rbga_u8(211, 211, 211, 255);
    pub const LIGHT_GREEN: Self =
        Self::rbga_u8(144, 238, 144, 255);
    pub const LIGHT_PINK: Self =
        Self::rbga_u8(255, 182, 193, 255);
    pub const LIGHT_SALMON: Self =
        Self::rbga_u8(255, 160, 122, 255);
    pub const LIGHT_SEA_GREEN: Self =
        Self::rbga_u8(32, 178, 170, 255);
    pub const LIGHT_SKY_BLUE: Self =
        Self::rbga_u8(135, 206, 250, 255);
    pub const LIGHT_SLATE_GRAY: Self =
        Self::rbga_u8(119, 136, 153, 255);
    pub const LIGHT_SLATE_GREY: Self =
        Self::rbga_u8(119, 136, 153, 255);
    pub const LIGHT_STEEL_BLUE: Self =
        Self::rbga_u8(176, 196, 222, 255);
    pub const LIGHT_YELLOW: Self =
        Self::rbga_u8(255, 255, 224, 255);
    pub const LIME: Self = Self::rbga_u8(0, 255, 0, 255);
    pub const LIME_GREEN: Self =
        Self::rbga_u8(50, 205, 50, 255);
    pub const LINEN: Self = Self::rbga_u8(250, 240, 230, 255);
    pub const MAGENTA: Self = Self::rbga_u8(255, 0, 255, 255);
    pub const MAROON: Self = Self::rbga_u8(128, 0, 0, 255);
    pub const MEDIUM_AQUAMARINE: Self =
        Self::rbga_u8(102, 205, 170, 255);
    pub const MEDIUM_BLUE: Self = Self::rbga_u8(0, 0, 205, 255);
    pub const MEDIUM_ORCHID: Self =
        Self::rbga_u8(186, 85, 211, 255);
    pub const MEDIUM_PURPLE: Self =
        Self::rbga_u8(147, 112, 219, 255);
    pub const MEDIUM_SEA_GREEN: Self =
        Self::rbga_u8(60, 179, 113, 255);
    pub const MEDIUM_SLATE_BLUE: Self =
        Self::rbga_u8(123, 104, 238, 255);
    pub const MEDIUM_SPRING_GREEN: Self =
        Self::rbga_u8(0, 250, 154, 255);
    pub const MEDIUM_TURQUOISE: Self =
        Self::rbga_u8(72, 209, 204, 255);
    pub const MEDIUM_VIOLET_RED: Self =
        Self::rbga_u8(199, 21, 133, 255);
    pub const MIDNIGHT_BLUE: Self =
        Self::rbga_u8(25, 25, 112, 255);
    pub const MINT_CREAM: Self =
        Self::rbga_u8(245, 255, 250, 255);
    pub const MISTY_ROSE: Self =
        Self::rbga_u8(255, 228, 225, 255);
    pub const MOCCASIN: Self =
        Self::rbga_u8(255, 228, 181, 255);
    pub const NAVAJO_WHITE: Self =
        Self::rbga_u8(255, 222, 173, 255);
    pub const NAVY: Self = Self::rbga_u8(0, 0, 128, 255);
    pub const OLD_LACE: Self =
        Self::rbga_u8(253, 245, 230, 255);
    pub const OLIVE: Self = Self::rbga_u8(128, 128, 0, 255);
    pub const OLIVE_DRAB: Self =
        Self::rbga_u8(107, 142, 35, 255);
    pub const ORANGE: Self = Self::rbga_u8(255, 165, 0, 255);
    pub const ORANGE_RED: Self = Self::rbga_u8(255, 69, 0, 255);
    pub const ORCHID: Self = Self::rbga_u8(218, 112, 214, 255);
    pub const PALE_GOLDENROD: Self =
        Self::rbga_u8(238, 232, 170, 255);
    pub const PALE_GREEN: Self =
        Self::rbga_u8(152, 251, 152, 255);
    pub const PALE_TURQUOISE: Self =
        Self::rbga_u8(175, 238, 238, 255);
    pub const PALE_VIOLET_RED: Self =
        Self::rbga_u8(219, 112, 147, 255);
    pub const PAPAYA_WHIP: Self =
        Self::rbga_u8(255, 239, 213, 255);
    pub const PEACH_PUFF: Self =
        Self::rbga_u8(255, 218, 185, 255);
    pub const PERU: Self = Self::rbga_u8(205, 133, 63, 255);
    pub const PINK: Self = Self::rbga_u8(255, 192, 203, 255);
    pub const PLUM: Self = Self::rbga_u8(221, 160, 221, 255);
    pub const POWDER_BLUE: Self =
        Self::rbga_u8(176, 224, 230, 255);
    pub const PURPLE: Self = Self::rbga_u8(128, 0, 128, 255);
    pub const REBECCA_PURPLE: Self =
        Self::rbga_u8(102, 51, 153, 255);
    pub const RED: Self = Self::rbga_u8(255, 0, 0, 255);
    pub const ROSY_BROWN: Self =
        Self::rbga_u8(188, 143, 143, 255);
    pub const ROYAL_BLUE: Self =
        Self::rbga_u8(65, 105, 225, 255);
    pub const SADDLE_BROWN: Self =
        Self::rbga_u8(139, 69, 19, 255);
    pub const SALMON: Self = Self::rbga_u8(250, 128, 114, 255);
    pub const SANDY_BROWN: Self =
        Self::rbga_u8(244, 164, 96, 255);
    pub const SEA_GREEN: Self = Self::rbga_u8(46, 139, 87, 255);
    pub const SEA_SHELL: Self =
        Self::rbga_u8(255, 245, 238, 255);
    pub const SIENNA: Self = Self::rbga_u8(160, 82, 45, 255);
    pub const SILVER: Self = Self::rbga_u8(192, 192, 192, 255);
    pub const SKY_BLUE: Self =
        Self::rbga_u8(135, 206, 235, 255);
    pub const SLATE_BLUE: Self =
        Self::rbga_u8(106, 90, 205, 255);
    pub const SLATE_GRAY: Self =
        Self::rbga_u8(112, 128, 144, 255);
    pub const SLATE_GREY: Self =
        Self::rbga_u8(112, 128, 144, 255);
    pub const SNOW: Self = Self::rbga_u8(255, 250, 250, 255);
    pub const SPRING_GREEN: Self =
        Self::rbga_u8(0, 255, 127, 255);
    pub const STEEL_BLUE: Self =
        Self::rbga_u8(70, 130, 180, 255);
    pub const TAN: Self = Self::rbga_u8(210, 180, 140, 255);
    pub const TEAL: Self = Self::rbga_u8(0, 128, 128, 255);
    pub const THISTLE: Self = Self::rbga_u8(216, 191, 216, 255);
    pub const TOMATO: Self = Self::rbga_u8(255, 99, 71, 255);
    pub const TURQUOISE: Self =
        Self::rbga_u8(64, 224, 208, 255);
    pub const VIOLET: Self = Self::rbga_u8(238, 130, 238, 255);
    pub const WHEAT: Self = Self::rbga_u8(245, 222, 179, 255);
    pub const WHITE: Self = Self::rbga_u8(255, 255, 255, 255);
    pub const WHITE_SMOKE: Self =
        Self::rbga_u8(245, 245, 245, 255);
    pub const YELLOW: Self = Self::rbga_u8(255, 255, 0, 255);
    pub const YELLOW_GREEN: Self =
        Self::rbga_u8(154, 205, 50, 255);
}
