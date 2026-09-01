//! Color palette (light/dark) and the Theme enum. Split out of main.rs by
//! split_main.sh.
//!
//! Both variants lean dark and high-contrast on purpose - this installer
//! hands someone a censorship-circumvention browser, not a spreadsheet, so
//! the palette favors near-black voids and a hot neon-violet accent over
//! the pastel/whitespace look of a typical SaaS UI. "Light" (`palette`) is
//! the dimmer "Dusk" variant; "Dark" (`palette_dark`) drops further into
//! near-black "Void". Deliberately no green anywhere - it's the reflexive
//! "hacker terminal" cliche and clashes with the violet accent, so success
//! states use an electric cyan-blue instead.

pub(crate) mod palette {
    #![allow(dead_code)]
    use egui::Color32;

    pub const PURPLE_DARK: Color32 = Color32::from_rgb(26, 6, 42);
    pub const PURPLE: Color32 = Color32::from_rgb(168, 22, 255);
    pub const PURPLE_SOFT: Color32 = Color32::from_rgb(52, 20, 78);
    pub const BG: Color32 = Color32::from_rgb(16, 12, 22);
    pub const SURFACE: Color32 = Color32::from_rgb(26, 20, 34);
    pub const BORDER: Color32 = Color32::from_rgb(54, 42, 72);
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(238, 232, 245);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(158, 148, 172);
    pub const SUCCESS: Color32 = Color32::from_rgb(56, 189, 248);
    pub const ERROR: Color32 = Color32::from_rgb(255, 59, 92);
    pub const GOLD: Color32 = Color32::from_rgb(232, 178, 64);
    pub const WARNING: Color32 = Color32::from_rgb(255, 138, 61);
    pub const WARNING_SOFT: Color32 = Color32::from_rgb(54, 34, 16);
    pub const SUCCESS_SOFT: Color32 = Color32::from_rgb(14, 40, 56);
}

pub(crate) mod palette_dark {
    use egui::Color32;

    pub const BG: Color32 = Color32::from_rgb(8, 6, 12);
    pub const SURFACE: Color32 = Color32::from_rgb(18, 15, 24);
    pub const BORDER: Color32 = Color32::from_rgb(42, 34, 56);
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(245, 240, 250);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(145, 138, 158);
    pub const PURPLE_SOFT: Color32 = Color32::from_rgb(40, 14, 58);
    pub const WARNING_SOFT: Color32 = Color32::from_rgb(42, 26, 12);
    pub const SUCCESS_SOFT: Color32 = Color32::from_rgb(10, 30, 44);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Theme {
    Light,
    Dark,
}