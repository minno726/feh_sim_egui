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

impl std::fmt::Display for BannerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_name = match self {
            BannerType::Normal => "Normal (3%/3%)",
            BannerType::HeroFest => "Hero fest (5%/3%)",
            BannerType::WeeklyRevival => "Weekly revival (4%/2%)",
            BannerType::Legendary => "Legendary (8%/0%)",
            BannerType::LegendaryRemix => "Legendary Remix (6%/0%)",
            BannerType::DoubleSpecial => "Double Special (6%/0%)",
        };

        f.write_str(display_name)
    }
}