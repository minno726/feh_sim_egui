use strum_macros::{FromRepr, EnumCount};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumCount)]
pub enum Color {
    Red,
    Blue,
    Green,
    Colorless,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromRepr, EnumCount)]
pub enum BannerType {
    Normal,
    HeroFest,
    WeeklyRevival,
    Legendary,
    LegendaryRemix,
    DoubleSpecial,
}

impl BannerType {
    pub fn display_name(&self) -> &'static str {
        match self {
            BannerType::Normal => "Normal (3%/3%)",
            BannerType::HeroFest => "Hero fest (5%/3%)",
            BannerType::WeeklyRevival => "Weekly revival (4%/2%)",
            BannerType::Legendary => "Legendary (8%/0%)",
            BannerType::LegendaryRemix => "Legendary Remix (6%/0%)",
            BannerType::DoubleSpecial => "Double Special (6%/0%)",
        }
    }
}