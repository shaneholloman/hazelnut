//! Theme system for Tidy TUI
//!
//! Supports multiple color themes with easy switching.

use ratatui::style::{Color, Modifier, Style};
use serde::{Deserialize, Serialize};

/// Available themes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    Catppuccin,
    Dracula,
    Nord,
    Gruvbox,
    TokyoNight,
    Monokai,
    Ocean,
    Sunset,
}

impl Theme {
    /// Get all available themes
    pub fn all() -> &'static [Theme] {
        &[
            Theme::Catppuccin,
            Theme::Dracula,
            Theme::Nord,
            Theme::Gruvbox,
            Theme::TokyoNight,
            Theme::Monokai,
            Theme::Ocean,
            Theme::Sunset,
        ]
    }

    /// Get the next theme in rotation
    pub fn next(&self) -> Theme {
        let themes = Self::all();
        let current = themes.iter().position(|t| t == self).unwrap_or(0);
        themes[(current + 1) % themes.len()]
    }

    /// Get theme name
    pub fn name(&self) -> &'static str {
        match self {
            Theme::Catppuccin => "Catppuccin Mocha",
            Theme::Dracula => "Dracula",
            Theme::Nord => "Nord",
            Theme::Gruvbox => "Gruvbox Dark",
            Theme::TokyoNight => "Tokyo Night",
            Theme::Monokai => "Monokai Pro",
            Theme::Ocean => "Ocean Deep",
            Theme::Sunset => "Sunset Glow",
        }
    }

    /// Load theme from config or use default
    pub fn load() -> anyhow::Result<Theme> {
        // TODO: Load from config file
        Ok(Theme::default())
    }

    /// Get the color palette for this theme
    pub fn colors(&self) -> ThemeColors {
        match self {
            Theme::Catppuccin => ThemeColors::catppuccin(),
            Theme::Dracula => ThemeColors::dracula(),
            Theme::Nord => ThemeColors::nord(),
            Theme::Gruvbox => ThemeColors::gruvbox(),
            Theme::TokyoNight => ThemeColors::tokyo_night(),
            Theme::Monokai => ThemeColors::monokai(),
            Theme::Ocean => ThemeColors::ocean(),
            Theme::Sunset => ThemeColors::sunset(),
        }
    }
}

/// Color palette for a theme
#[derive(Debug, Clone)]
pub struct ThemeColors {
    // Base colors
    pub bg: Color,
    pub bg_secondary: Color,
    pub bg_highlight: Color,
    pub fg: Color,
    pub fg_dim: Color,
    pub fg_muted: Color,

    // Accent colors
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,

    // Semantic colors
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,

    // UI elements
    pub border: Color,
    pub border_focus: Color,
    pub selection: Color,

    // Special
    pub logo_primary: Color,
    pub logo_secondary: Color,
}

impl ThemeColors {
    /// Catppuccin Mocha - Warm and cozy
    pub fn catppuccin() -> Self {
        Self {
            bg: Color::Rgb(30, 30, 46),
            bg_secondary: Color::Rgb(49, 50, 68),
            bg_highlight: Color::Rgb(69, 71, 90),
            fg: Color::Rgb(205, 214, 244),
            fg_dim: Color::Rgb(166, 173, 200),
            fg_muted: Color::Rgb(108, 112, 134),

            primary: Color::Rgb(137, 180, 250),   // Blue
            secondary: Color::Rgb(180, 190, 254), // Lavender
            accent: Color::Rgb(245, 194, 231),    // Pink

            success: Color::Rgb(166, 227, 161), // Green
            warning: Color::Rgb(249, 226, 175), // Yellow
            error: Color::Rgb(243, 139, 168),   // Red
            info: Color::Rgb(148, 226, 213),    // Teal

            border: Color::Rgb(69, 71, 90),
            border_focus: Color::Rgb(137, 180, 250),
            selection: Color::Rgb(88, 91, 112),

            logo_primary: Color::Rgb(203, 166, 247), // Mauve
            logo_secondary: Color::Rgb(137, 180, 250), // Blue
        }
    }

    /// Dracula - Dark and vibrant
    pub fn dracula() -> Self {
        Self {
            bg: Color::Rgb(40, 42, 54),
            bg_secondary: Color::Rgb(68, 71, 90),
            bg_highlight: Color::Rgb(98, 114, 164),
            fg: Color::Rgb(248, 248, 242),
            fg_dim: Color::Rgb(189, 147, 249),
            fg_muted: Color::Rgb(98, 114, 164),

            primary: Color::Rgb(189, 147, 249),   // Purple
            secondary: Color::Rgb(139, 233, 253), // Cyan
            accent: Color::Rgb(255, 121, 198),    // Pink

            success: Color::Rgb(80, 250, 123),  // Green
            warning: Color::Rgb(241, 250, 140), // Yellow
            error: Color::Rgb(255, 85, 85),     // Red
            info: Color::Rgb(139, 233, 253),    // Cyan

            border: Color::Rgb(68, 71, 90),
            border_focus: Color::Rgb(189, 147, 249),
            selection: Color::Rgb(68, 71, 90),

            logo_primary: Color::Rgb(255, 121, 198),   // Pink
            logo_secondary: Color::Rgb(189, 147, 249), // Purple
        }
    }

    /// Nord - Cool and calm
    pub fn nord() -> Self {
        Self {
            bg: Color::Rgb(46, 52, 64),
            bg_secondary: Color::Rgb(59, 66, 82),
            bg_highlight: Color::Rgb(67, 76, 94),
            fg: Color::Rgb(236, 239, 244),
            fg_dim: Color::Rgb(216, 222, 233),
            fg_muted: Color::Rgb(76, 86, 106),

            primary: Color::Rgb(136, 192, 208),   // Frost
            secondary: Color::Rgb(129, 161, 193), // Frost darker
            accent: Color::Rgb(180, 142, 173),    // Aurora purple

            success: Color::Rgb(163, 190, 140), // Aurora green
            warning: Color::Rgb(235, 203, 139), // Aurora yellow
            error: Color::Rgb(191, 97, 106),    // Aurora red
            info: Color::Rgb(136, 192, 208),    // Frost

            border: Color::Rgb(59, 66, 82),
            border_focus: Color::Rgb(136, 192, 208),
            selection: Color::Rgb(67, 76, 94),

            logo_primary: Color::Rgb(136, 192, 208), // Frost
            logo_secondary: Color::Rgb(163, 190, 140), // Green
        }
    }

    /// Gruvbox Dark - Retro warm
    pub fn gruvbox() -> Self {
        Self {
            bg: Color::Rgb(40, 40, 40),
            bg_secondary: Color::Rgb(60, 56, 54),
            bg_highlight: Color::Rgb(80, 73, 69),
            fg: Color::Rgb(235, 219, 178),
            fg_dim: Color::Rgb(213, 196, 161),
            fg_muted: Color::Rgb(146, 131, 116),

            primary: Color::Rgb(131, 165, 152),  // Aqua
            secondary: Color::Rgb(184, 187, 38), // Yellow-green
            accent: Color::Rgb(211, 134, 155),   // Purple

            success: Color::Rgb(152, 151, 26), // Green
            warning: Color::Rgb(250, 189, 47), // Yellow
            error: Color::Rgb(251, 73, 52),    // Red
            info: Color::Rgb(131, 165, 152),   // Aqua

            border: Color::Rgb(80, 73, 69),
            border_focus: Color::Rgb(250, 189, 47),
            selection: Color::Rgb(80, 73, 69),

            logo_primary: Color::Rgb(254, 128, 25),   // Orange
            logo_secondary: Color::Rgb(250, 189, 47), // Yellow
        }
    }

    /// Tokyo Night - Modern dark
    pub fn tokyo_night() -> Self {
        Self {
            bg: Color::Rgb(26, 27, 38),
            bg_secondary: Color::Rgb(36, 40, 59),
            bg_highlight: Color::Rgb(41, 46, 66),
            fg: Color::Rgb(192, 202, 245),
            fg_dim: Color::Rgb(169, 177, 214),
            fg_muted: Color::Rgb(86, 95, 137),

            primary: Color::Rgb(122, 162, 247),   // Blue
            secondary: Color::Rgb(187, 154, 247), // Purple
            accent: Color::Rgb(255, 158, 100),    // Orange

            success: Color::Rgb(158, 206, 106), // Green
            warning: Color::Rgb(224, 175, 104), // Yellow
            error: Color::Rgb(247, 118, 142),   // Red
            info: Color::Rgb(125, 207, 255),    // Cyan

            border: Color::Rgb(41, 46, 66),
            border_focus: Color::Rgb(122, 162, 247),
            selection: Color::Rgb(52, 59, 88),

            logo_primary: Color::Rgb(187, 154, 247), // Purple
            logo_secondary: Color::Rgb(122, 162, 247), // Blue
        }
    }

    /// Monokai Pro - Classic dark
    pub fn monokai() -> Self {
        Self {
            bg: Color::Rgb(45, 42, 46),
            bg_secondary: Color::Rgb(55, 52, 56),
            bg_highlight: Color::Rgb(73, 72, 62),
            fg: Color::Rgb(252, 252, 250),
            fg_dim: Color::Rgb(199, 199, 199),
            fg_muted: Color::Rgb(117, 113, 94),

            primary: Color::Rgb(102, 217, 239),   // Cyan
            secondary: Color::Rgb(174, 129, 255), // Purple
            accent: Color::Rgb(255, 97, 136),     // Pink

            success: Color::Rgb(166, 226, 46),  // Green
            warning: Color::Rgb(230, 219, 116), // Yellow
            error: Color::Rgb(249, 38, 114),    // Red/Pink
            info: Color::Rgb(102, 217, 239),    // Cyan

            border: Color::Rgb(73, 72, 62),
            border_focus: Color::Rgb(255, 97, 136),
            selection: Color::Rgb(73, 72, 62),

            logo_primary: Color::Rgb(166, 226, 46),    // Green
            logo_secondary: Color::Rgb(102, 217, 239), // Cyan
        }
    }

    /// Ocean Deep - Cool blue depths
    pub fn ocean() -> Self {
        Self {
            bg: Color::Rgb(15, 25, 40),
            bg_secondary: Color::Rgb(25, 40, 60),
            bg_highlight: Color::Rgb(35, 55, 80),
            fg: Color::Rgb(220, 235, 250),
            fg_dim: Color::Rgb(180, 200, 220),
            fg_muted: Color::Rgb(100, 130, 160),

            primary: Color::Rgb(80, 180, 230),    // Ocean blue
            secondary: Color::Rgb(100, 220, 200), // Teal
            accent: Color::Rgb(255, 180, 100),    // Coral

            success: Color::Rgb(80, 200, 150),  // Sea green
            warning: Color::Rgb(255, 200, 100), // Sandy
            error: Color::Rgb(255, 100, 120),   // Coral red
            info: Color::Rgb(100, 200, 255),    // Light blue

            border: Color::Rgb(40, 60, 90),
            border_focus: Color::Rgb(80, 180, 230),
            selection: Color::Rgb(35, 55, 80),

            logo_primary: Color::Rgb(100, 220, 200),  // Teal
            logo_secondary: Color::Rgb(80, 180, 230), // Blue
        }
    }

    /// Sunset Glow - Warm twilight
    pub fn sunset() -> Self {
        Self {
            bg: Color::Rgb(35, 25, 35),
            bg_secondary: Color::Rgb(50, 35, 50),
            bg_highlight: Color::Rgb(70, 50, 70),
            fg: Color::Rgb(255, 240, 230),
            fg_dim: Color::Rgb(230, 200, 190),
            fg_muted: Color::Rgb(150, 120, 130),

            primary: Color::Rgb(255, 150, 100),   // Orange
            secondary: Color::Rgb(255, 120, 150), // Pink
            accent: Color::Rgb(200, 150, 255),    // Purple

            success: Color::Rgb(150, 220, 130), // Soft green
            warning: Color::Rgb(255, 220, 100), // Golden
            error: Color::Rgb(255, 100, 100),   // Red
            info: Color::Rgb(150, 200, 255),    // Sky blue

            border: Color::Rgb(80, 60, 80),
            border_focus: Color::Rgb(255, 150, 100),
            selection: Color::Rgb(70, 50, 70),

            logo_primary: Color::Rgb(255, 150, 100), // Orange
            logo_secondary: Color::Rgb(255, 120, 150), // Pink
        }
    }

    // Style helpers

    /// Default text style
    pub fn text(&self) -> Style {
        Style::default().fg(self.fg)
    }

    /// Dimmed text style
    pub fn text_dim(&self) -> Style {
        Style::default().fg(self.fg_dim)
    }

    /// Muted text style
    pub fn text_muted(&self) -> Style {
        Style::default().fg(self.fg_muted)
    }

    /// Primary accent style
    pub fn text_primary(&self) -> Style {
        Style::default().fg(self.primary)
    }

    /// Secondary accent style
    pub fn text_secondary(&self) -> Style {
        Style::default().fg(self.secondary)
    }

    /// Success style
    pub fn text_success(&self) -> Style {
        Style::default().fg(self.success)
    }

    /// Warning style
    pub fn text_warning(&self) -> Style {
        Style::default().fg(self.warning)
    }

    /// Error style
    pub fn text_error(&self) -> Style {
        Style::default().fg(self.error)
    }

    /// Info style
    pub fn text_info(&self) -> Style {
        Style::default().fg(self.info)
    }

    /// Block border style
    pub fn block(&self) -> Style {
        Style::default().fg(self.border)
    }

    /// Focused block border style
    pub fn block_focus(&self) -> Style {
        Style::default().fg(self.border_focus)
    }

    /// Selected item style
    pub fn selected(&self) -> Style {
        Style::default()
            .bg(self.selection)
            .fg(self.fg)
            .add_modifier(Modifier::BOLD)
    }

    /// Tab style
    pub fn tab(&self) -> Style {
        Style::default().fg(self.fg_muted)
    }

    /// Active tab style
    pub fn tab_active(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Key hint style (for shortcuts)
    pub fn key_hint(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    /// Logo primary style
    pub fn logo_style_primary(&self) -> Style {
        Style::default()
            .fg(self.logo_primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Logo secondary style
    pub fn logo_style_secondary(&self) -> Style {
        Style::default()
            .fg(self.logo_secondary)
            .add_modifier(Modifier::BOLD)
    }
}
