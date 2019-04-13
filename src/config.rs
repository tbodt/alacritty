//! Configuration data for Alacritty.
use Rgb;
use ansi::CursorStyle;
use display::Offset;
use term::VisualBell;

/// Main config object
#[derive(Debug, SmartDefault)]
pub struct Config {
    /// Style of the cursor
    pub cursor_style: CursorStyle,

    /// Should draw bold text with brighter colors instead of bold font
    #[default = "false"]
    pub draw_bold_text_with_bright_colors: bool,

    /// Number of spaces in one tab
    #[default = "8"]
    pub tabspaces: usize,

    #[default = r#"",│`|:\"' ()[]{}<>".to_string()"#]
    pub semantic_escape_chars: String,

    /// Visual bell configuration
    pub visual_bell: VisualBell,

    /// Use dynamic title
    #[default = "true"]
    pub dynamic_title: bool,

    /// Terminal padding, in pixels
    #[default = "Offset { x: 2, y: 2 }"]
    pub padding: Offset<u8>,

    /// Terminal colors
    pub colors: Colors,
    /// Should use custom cursor colors
    pub custom_cursor_colors: bool,

    /// Background opacity from 0.0 to 1.0
    #[default = "1.0"]
    pub background_opacity: f32,

    /// Font configuration
    pub font: Font,

    /// Should show render timer
    #[default = "false"]
    pub render_timer: bool,
}

/// The 8-colors sections of config
#[derive(Debug)]
pub struct AnsiColors {
    pub black: Rgb,
    pub red: Rgb,
    pub green: Rgb,
    pub yellow: Rgb,
    pub blue: Rgb,
    pub magenta: Rgb,
    pub cyan: Rgb,
    pub white: Rgb,
}

#[derive(Debug)]
pub struct Colors {
    pub primary: PrimaryColors,
    pub cursor: CursorColors,
    pub normal: AnsiColors,
    pub bright: AnsiColors,
    pub dim: Option<AnsiColors>,
}

impl Default for Colors {
    fn default() -> Colors {
        Colors {
            primary: PrimaryColors::default(),
            cursor: CursorColors::default(),
            normal: AnsiColors {
                black: Rgb {r: 0x00, g: 0x00, b: 0x00},
                red: Rgb {r: 0xd5, g: 0x4e, b: 0x53},
                green: Rgb {r: 0xb9, g: 0xca, b: 0x4a},
                yellow: Rgb {r: 0xe6, g: 0xc5, b: 0x47},
                blue: Rgb {r: 0x7a, g: 0xa6, b: 0xda},
                magenta: Rgb {r: 0xc3, g: 0x97, b: 0xd8},
                cyan: Rgb {r: 0x70, g: 0xc0, b: 0xba},
                white: Rgb {r: 0xea, g: 0xea, b: 0xea},
            },
            bright: AnsiColors {
                black: Rgb {r: 0x66, g: 0x66, b: 0x66},
                red: Rgb {r: 0xff, g: 0x33, b: 0x34},
                green: Rgb {r: 0x9e, g: 0xc4, b: 0x00},
                yellow: Rgb {r: 0xe7, g: 0xc5, b: 0x47},
                blue: Rgb {r: 0x7a, g: 0xa6, b: 0xda},
                magenta: Rgb {r: 0xb7, g: 0x7e, b: 0xe0},
                cyan: Rgb {r: 0x54, g: 0xce, b: 0xd6},
                white: Rgb {r: 0xff, g: 0xff, b: 0xff},
            },
            dim: None,
        }
    }
}

#[derive(Debug, SmartDefault)]
pub struct CursorColors {
    #[default = "Rgb { r: 0, g: 0, b: 0 }"]
    pub text: Rgb,
    #[default = "Rgb { r: 0xff, g: 0xff, b: 0xff}"]
    pub cursor: Rgb,
}

#[derive(Debug, SmartDefault)]
pub struct PrimaryColors {
    #[default = "Rgb {r: 0, g: 0, b: 0 }"]
    pub background: Rgb,
    #[default = "Rgb { r: 0xea, g: 0xea, b: 0xea }"]
    pub foreground: Rgb,
}

