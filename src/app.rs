use egui::{RichText, Vec2, Widget};
use strum::EnumCount;
use crate::types::*;

fn default_units(banner_type: BannerType) -> Option<Vec<Unit>> {
    match banner_type {
        BannerType::HeroFest => Some(vec![
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Red,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Blue,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Green,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Colorless,
                fourstar_focus: false,
            },
        ]),
        BannerType::Legendary => Some(vec![
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Red,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Red,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Red,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Blue,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Blue,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Blue,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Green,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Green,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Green,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Colorless,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Colorless,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Colorless,
                fourstar_focus: false,
            },
        ]),
        BannerType::LegendaryRemix | BannerType::DoubleSpecial => Some(vec![
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Red,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Red,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Blue,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Blue,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Green,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Green,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Colorless,
                fourstar_focus: false,
            },
            Unit {
                name: "".into(),
                placeholder_name: true,
                color: Color::Colorless,
                fourstar_focus: false,
            },
        ]),
        _ => None,
    }
}

fn current_banners() -> Vec<(String, usize, Vec<Unit>)> {
    vec![
        ("Custom banner".into(), 0, vec![]),
        (
            "Hero Fest".into(),
            1,
            vec![
                Unit {
                    name: "Múspell".into(),
                    placeholder_name: false,
                    color: Color::Red,
                    fourstar_focus: false,
                },
                Unit {
                    name: "Nifl".into(),
                    placeholder_name: false,
                    color: Color::Blue,
                    fourstar_focus: false,
                },
                Unit {
                    name: "Ascendant Fjorm".into(),
                    placeholder_name: false,
                    color: Color::Green,
                    fourstar_focus: false,
                },
                Unit {
                    name: "Ascendant Laegjarn".into(),
                    placeholder_name: false,
                    color: Color::Colorless,
                    fourstar_focus: false,
                },
            ],
        ),
        (
            "Risk and Reward".into(),
            0,
            vec![
                Unit {
                    name: "Thief Leila".into(),
                    placeholder_name: false,
                    color: Color::Red,
                    fourstar_focus: false,
                },
                Unit {
                    name: "Thief Cath".into(),
                    placeholder_name: false,
                    color: Color::Blue,
                    fourstar_focus: false,
                },
                Unit {
                    name: "Thief Sothe".into(),
                    placeholder_name: false,
                    color: Color::Green,
                    fourstar_focus: true,
                },
                Unit {
                    name: "Thief Nina".into(),
                    placeholder_name: false,
                    color: Color::Colorless,
                    fourstar_focus: false,
                },
            ],
        ),
        (
            "Legendary & Mythic Hero Remix".into(),
            4,
            vec![
                Unit {
                    name: "Hríd".into(),
                    placeholder_name: false,
                    color: Color::Red,
                    fourstar_focus: false,
                },
                Unit {
                    name: "Legendary Eirika".into(),
                    placeholder_name: false,
                    color: Color::Red,
                    fourstar_focus: false,
                },
                Unit {
                    name: "Naga".into(),
                    placeholder_name: false,
                    color: Color::Blue,
                    fourstar_focus: false,
                },
                Unit {
                    name: "Ascendant Idunn".into(),
                    placeholder_name: false,
                    color: Color::Blue,
                    fourstar_focus: false,
                },
                Unit {
                    name: "Thrasir".into(),
                    placeholder_name: false,
                    color: Color::Green,
                    fourstar_focus: false,
                },
                Unit {
                    name: "Gunnthrá".into(),
                    placeholder_name: false,
                    color: Color::Green,
                    fourstar_focus: false,
                },
                Unit {
                    name: "Legendary Alm".into(),
                    placeholder_name: false,
                    color: Color::Colorless,
                    fourstar_focus: false,
                },
                Unit {
                    name: "Ascendant Joshua".into(),
                    placeholder_name: false,
                    color: Color::Colorless,
                    fourstar_focus: false,
                },
            ],
        ),
        (
            "Focus: Heroes w/Dodge Skills".into(),
            0,
            vec![
                Unit {
                    name: "Fallen Ike".into(),
                    placeholder_name: false,
                    color: Color::Red,
                    fourstar_focus: false,
                },
                Unit {
                    name: "Say'ri".into(),
                    placeholder_name: false,
                    color: Color::Red,
                    fourstar_focus: false,
                },
                Unit {
                    name: "Female Kris".into(),
                    placeholder_name: false,
                    color: Color::Blue,
                    fourstar_focus: false,
                },
            ],
        ),
        (
            "Focus: Weekly Revival 35".into(),
            2,
            vec![
                Unit {
                    name: "Laegjarn".into(),
                    placeholder_name: false,
                    color: Color::Red,
                    fourstar_focus: false,
                },
                Unit {
                    name: "Ylgr".into(),
                    placeholder_name: false,
                    color: Color::Blue,
                    fourstar_focus: false,
                },
                Unit {
                    name: "Surtr".into(),
                    placeholder_name: false,
                    color: Color::Green,
                    fourstar_focus: false,
                },
            ],
        ),
        (
            "This one has a very long name so that I can test out how the UI looks".into(),
            0,
            vec![
                Unit {
                    name: "The unit also has a very long name, which is silly".into(),
                    placeholder_name: false,
                    color: Color::Blue,
                    fourstar_focus: false,
                },
                Unit {
                    name: "Not this one".into(),
                    placeholder_name: false,
                    color: Color::Green,
                    fourstar_focus: false,
                },
            ],
        ),
    ]
}

#[derive(Clone)]
struct Unit {
    name: String,
    placeholder_name: bool,
    color: Color,
    fourstar_focus: bool,
}

pub struct TemplateApp {
    banner_type: BannerType,
    banner_selected: usize,
    banner_units: Vec<Unit>,

    goal_is_multiple: bool,
    goal_simple: (usize, usize, usize),
    /// false = target # of copies, true = orb budget
    goal_simple_mode: bool,
    goal_multiple: Vec<(String, bool, usize)>,
    /// false = target any, true = target all
    goal_multiple_mode: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            banner_type: BannerType::Normal,
            banner_selected: 0,
            banner_units: Vec::new(),
            goal_is_multiple: false,
            goal_simple: (0, 1, 100),
            goal_simple_mode: false,
            goal_multiple: Vec::new(),
            goal_multiple_mode: false,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::set_basic_style(cc);
        Default::default()
    }

    fn set_basic_style(cc: &eframe::CreationContext<'_>) {
        use egui::FontFamily;
        use egui::FontId;
        use egui::TextStyle::*;
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(30.0, FontFamily::Proportional)),
            (
                Name("Heading2".into()),
                FontId::new(25.0, FontFamily::Proportional),
            ),
            (
                Name("Context".into()),
                FontId::new(23.0, FontFamily::Proportional),
            ),
            (Body, FontId::new(18.0, FontFamily::Proportional)),
            (Monospace, FontId::new(18.0, FontFamily::Monospace)),
            (Button, FontId::new(18.0, FontFamily::Proportional)),
            (Small, FontId::new(14.0, FontFamily::Proportional)),
        ]
        .into();
        cc.egui_ctx.set_style(style);
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                let TemplateApp {
                    banner_type,
                    banner_selected: selected_banner,
                    banner_units,
                    goal_is_multiple,
                    goal_simple,
                    goal_simple_mode,
                    goal_multiple,
                    goal_multiple_mode: goal_complex_mode,
                } = self;

                let current_banners = current_banners();

                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.collapsing(RichText::new("Banner").heading(), |ui| {
                            if egui::ComboBox::from_id_source("Selected banner")
                                .width(200.0)
                                .selected_text(format!("{:?}", banner_type))
                                .show_index(ui, selected_banner, current_banners.len(), |i| {
                                    current_banners[i].0.clone()
                                })
                                .changed()
                            {
                                if *selected_banner != 0 {
                                    *banner_type = BannerType::from_repr(current_banners[*selected_banner].1).unwrap();
                                    *banner_units = current_banners[*selected_banner].2.clone();
                                }
                                *goal_multiple = banner_units
                                    .iter()
                                    .map(|unit| (unit.name.clone(), false, 0))
                                    .collect();
                            }
                            ui.add_enabled_ui(*selected_banner == 0, |ui| {
                                let mut selected = *banner_type as usize;
                                if egui::ComboBox::from_id_source("Banner type")
                                    .width(200.0)
                                    .selected_text(format!("{:?}", banner_type))
                                    .show_index(ui, &mut selected, BannerType::COUNT, |i| {
                                        format!("{}", BannerType::from_repr(i).unwrap())
                                    })
                                    .changed()
                                {
                                    *banner_type = BannerType::from_repr(selected).unwrap();
                                    if let Some(new_units) = default_units(*banner_type) {
                                        *banner_units = new_units;
                                    }
                                }
                                let mut to_remove = None;
                                let mut lowest_free_placeholder = [1; 4];
                                for (idx, unit) in banner_units.iter_mut().enumerate() {
                                    // Auto-name the units that the user hasn't given an explicit name to
                                    if unit.placeholder_name {
                                        unit.name = format!(
                                            "{:?} {}",
                                            unit.color, lowest_free_placeholder[unit.color as usize]
                                        );
                                        lowest_free_placeholder[unit.color as usize] += 1;
                                    }
                                    ui.group(|ui| {
                                        ui.horizontal(|ui| {
                                            if ui
                                                .add(
                                                    egui::TextEdit::singleline(&mut unit.name)
                                                        .desired_width(200.0),
                                                )
                                                .changed()
                                            {
                                                // Stop auto-naming them if the user entered a name
                                                unit.placeholder_name = false;
                                            }
                                            if ui.button("X").clicked() {
                                                to_remove = Some(idx);
                                            }
                                        });
                                        ui.horizontal(|ui| {
                                            let mut selected = unit.color as usize;
                                            if egui::ComboBox::from_id_source(("banner_unit", idx))
                                                .selected_text(format!("{:?}", unit.color))
                                                .show_index(
                                                    ui,
                                                    &mut selected,
                                                    Color::COUNT,
                                                    |i| format!("{:?}", Color::from_repr(i).unwrap()),
                                                ).changed() {
                                                    unit.color = Color::from_repr(selected).unwrap();
                                            }
                                            let can_have_fourstar_focus =
                                                *banner_type == BannerType::Normal || *banner_type == BannerType::DoubleSpecial;
                                            unit.fourstar_focus =
                                                unit.fourstar_focus && can_have_fourstar_focus;
                                            ui.add_enabled(
                                                can_have_fourstar_focus,
                                                egui::Checkbox::new(
                                                    &mut unit.fourstar_focus,
                                                    "4* focus?",
                                                ),
                                            );
                                        })
                                    });
                                }
                                if let Some(idx) = to_remove {
                                    banner_units.remove(idx);
                                }
                                ui.group(|ui| {
                                    ui.horizontal(|ui| {
                                        if ui.button("Add unit").clicked() {
                                            banner_units.push(Unit {
                                                name: "".into(),
                                                placeholder_name: true,
                                                color: Color::Red,
                                                fourstar_focus: false,
                                            });
                                        }
                                    });
                                });
                            });
                        });

                        ui.collapsing(RichText::new("Goal").heading(), |ui| {
                            if banner_units.len() == 0 {
                                ui.label("You need to add units to the banner.");
                                return;
                            }

                            ui.horizontal(|ui| {
                                ui.radio_value(goal_is_multiple, false, "Single");
                                ui.radio_value(goal_is_multiple, true, "Multiple");
                            });
                            if !*goal_is_multiple {
                                if goal_simple.0 >= banner_units.len() {
                                    goal_simple.0 = 0;
                                }
                                egui::ComboBox::from_label("Unit")
                                    .width(200.0)
                                    .selected_text(format!(
                                        "{:?}",
                                        banner_units[dbg!(goal_simple.0)].name
                                    ))
                                    .show_index(ui, &mut goal_simple.0, banner_units.len(), |i| {
                                        banner_units[i].name.clone()
                                    });
                                ui.horizontal(|ui| {
                                    ui.radio_value(goal_simple_mode, false, "# of copies");
                                    ui.radio_value(goal_simple_mode, true, "Orb budget");
                                });
                                if !*goal_simple_mode {
                                    // # goal
                                    ui.horizontal(|ui| {
                                        if ui.button("1 copy").clicked() {
                                            goal_simple.1 = 1;
                                        }
                                        egui::DragValue::new(&mut goal_simple.1)
                                            .clamp_range(1..=11)
                                            .ui(ui);
                                        if ui.button("to +10").clicked() {
                                            goal_simple.1 = 11;
                                        }
                                    });
                                } else {
                                    // orb budget
                                    // 10k orbs gets you a >99% chance of a +10 of every unit
                                    // on a legendary banner, so I doubt anyone will need to go higher than that
                                    egui::DragValue::new(&mut goal_simple.2)
                                        .clamp_range(5..=10000)
                                        .ui(ui);
                                }
                            } else {
                                if banner_units.len() != goal_multiple.len() {
                                    *goal_multiple = banner_units
                                        .iter()
                                        .map(|unit| (unit.name.clone(), false, 0))
                                        .collect();
                                }
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        for (idx, unit) in goal_multiple.iter_mut().enumerate() {
                                            unit.0 = banner_units[idx].name.clone();
                                            if ui.checkbox(&mut unit.1, unit.0.clone()).changed() {
                                                if unit.1 && unit.2 == 0 {
                                                    unit.2 = 1;
                                                } else if !unit.1 && unit.2 > 0 {
                                                    unit.2 = 0;
                                                }
                                            }
                                            ui.add_space(2.0); // to align with the slightly-taller DragValue elements
                                        }
                                    });
                                    ui.vertical(|ui| {
                                        for unit in goal_multiple {
                                            ui.horizontal(|ui| {
                                                ui.add_visible(
                                                    unit.1,
                                                    egui::DragValue::new(&mut unit.2)
                                                        .clamp_range(1..=99),
                                                );
                                                ui.add_visible(unit.1, egui::Label::new("copies"));
                                            });
                                        }
                                    });
                                });
                                ui.horizontal(|ui| {
                                    ui.radio_value(goal_complex_mode, false, "All of these");
                                    ui.radio_value(goal_complex_mode, true, "Any of these");
                                });
                            }
                        });
                        egui::warn_if_debug_build(ui);
                    });
            });
    }

    fn max_size_points(&self) -> Vec2 {
        Vec2::new(800.0, f32::INFINITY)
    }
}
