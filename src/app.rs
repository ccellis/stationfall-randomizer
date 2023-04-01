use std::num;

use rand::{thread_rng, random};
use rand::seq::SliceRandom;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Clone, Eq, Ord, PartialOrd)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Character {
    pub name: String,
    pub difficulty: Difficulty,
    pub is_robot: bool,
    pub is_officer: bool,
    pub cares_about_briefcase: bool,
    pub cares_about_artifact: bool,
    pub wants_contamintaion: bool,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    number_of_characters: usize,
    at_least_one_robot: bool,
    at_least_one_officer: bool,
    at_least_two_agents_per_goal: bool,
    preferred_maximum_difficulty: Difficulty,
    global_easy_character_list: Vec<Character>,
    global_medium_character_list: Vec<Character>,
    global_hard_character_list: Vec<Character>,
    randomized_character_list: Vec<Character>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            number_of_characters: 12,
            at_least_one_robot: true,
            at_least_one_officer: true,
            at_least_two_agents_per_goal: true,
            preferred_maximum_difficulty: Difficulty::Easy,
            global_easy_character_list: vec![
                Character{
                    name: "Astrochimp".to_string(),
                    difficulty: Difficulty::Easy,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: true,
                    cares_about_artifact: true,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Colonel".to_string(),
                    difficulty: Difficulty::Medium,
                    is_robot: false,
                    is_officer: true,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: true,
                },
                Character{
                    name: "Counselor".to_string(),
                    difficulty: Difficulty::Easy,
                    is_robot: false,
                    is_officer: true,
                    cares_about_briefcase: false,
                    cares_about_artifact: true,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Cyborg".to_string(),
                    difficulty: Difficulty::Easy,
                    is_robot: true,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Daredevil".to_string(),
                    difficulty: Difficulty::Easy,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: true,
                    wants_contamintaion: true,
                },
                Character{
                    name: "Engineer".to_string(),
                    difficulty: Difficulty::Easy,
                    is_robot: false,
                    is_officer: true,
                    cares_about_briefcase: false,
                    cares_about_artifact: true,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Exile".to_string(),
                    difficulty: Difficulty::Easy,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Inspector".to_string(),
                    difficulty: Difficulty::Easy,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: true,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Maintenance Clones".to_string(),
                    difficulty: Difficulty::Easy,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Medical".to_string(),
                    difficulty: Difficulty::Easy,
                    is_robot: true,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Security".to_string(),
                    difficulty: Difficulty::Easy,
                    is_robot: true,
                    is_officer: false,
                    cares_about_briefcase: true,
                    cares_about_artifact: true,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Station Chief".to_string(),
                    difficulty: Difficulty::Easy,
                    is_robot: false,
                    is_officer: true,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Stowaway".to_string(),
                    difficulty: Difficulty::Easy,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Troubleshooter".to_string(),
                    difficulty: Difficulty::Easy,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
            ],
            global_medium_character_list: vec![
                Character{
                    name: "Microbiologist".to_string(),
                    difficulty: Difficulty::Medium,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: true,
                },
                Character{
                    name: "Operative".to_string(),
                    difficulty: Difficulty::Medium,
                    is_robot: false,
                    is_officer: true,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Stranger".to_string(),
                    difficulty: Difficulty::Medium,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Boarder".to_string(),
                    difficulty: Difficulty::Medium,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: true,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Corpsicle".to_string(),
                    difficulty: Difficulty::Medium,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: true,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Doctor".to_string(),
                    difficulty: Difficulty::Medium,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: true,
                    cares_about_artifact: true,
                    wants_contamintaion: true,
                },
                Character{
                    name: "Drones".to_string(),
                    difficulty: Difficulty::Medium,
                    is_robot: true,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
            ],
            global_hard_character_list: vec![
                Character{
                    name: "Legal".to_string(),
                    difficulty: Difficulty::Hard,
                    is_robot: true,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Billionaire".to_string(),
                    difficulty: Difficulty::Hard,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Botanist".to_string(),
                    difficulty: Difficulty::Hard,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Consort".to_string(),
                    difficulty: Difficulty::Hard,
                    is_robot: true,
                    is_officer: false,
                    cares_about_briefcase: true,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Digital Assistant".to_string(),
                    difficulty: Difficulty::Hard,
                    is_robot: false,
                    is_officer: true,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character{
                    name: "Telepathic Rat".to_string(),
                    difficulty: Difficulty::Hard,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: true,
                    cares_about_artifact: true,
                    wants_contamintaion: false,
                },
            ],
            randomized_character_list: vec![]
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {number_of_characters,
            at_least_one_robot,
            at_least_one_officer,
            at_least_two_agents_per_goal,
            preferred_maximum_difficulty,
            global_easy_character_list,
            global_medium_character_list,
            global_hard_character_list,
            randomized_character_list
        } = self;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Config");

            ui.horizontal(|ui| {
                ui.label("Number of Characters");
                egui::ComboBox::from_id_source(1)
                    .selected_text(format!("{:?}", number_of_characters))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(number_of_characters, 12, "12 (2p, 3p, 4p)");
                        ui.selectable_value(number_of_characters, 14, "14 (5p)");
                        ui.selectable_value(number_of_characters, 15, "15 (6p)");
                        ui.selectable_value(number_of_characters, 17, "17 (7p)");
                        ui.selectable_value(number_of_characters, 19, "19 (8p)");
                        ui.selectable_value(number_of_characters, 20, "20 (9p)");
                    }
                );
            });

            ui.horizontal(|ui| {
                ui.label("Preferred Maximum Difficulty");
                egui::ComboBox::from_id_source(2)
                    .selected_text(format!("{:?}", preferred_maximum_difficulty))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(preferred_maximum_difficulty, Difficulty::Easy, "Easy");
                        ui.selectable_value(preferred_maximum_difficulty, Difficulty::Medium, "Medium");
                        ui.selectable_value(preferred_maximum_difficulty, Difficulty::Hard, "Hard");
                    }
                );
            });

            ui.horizontal(|ui| {
                ui.label("At Least One Officer");
                ui.add(egui::Checkbox::without_text(at_least_one_officer));
            });

            ui.horizontal(|ui| {
                ui.label("At Least One Robot");
                ui.add(egui::Checkbox::without_text(at_least_one_robot));
            });

            ui.horizontal(|ui| {
                ui.label("At Least Two Characters Per Goal");
                ui.add(egui::Checkbox::without_text(at_least_two_agents_per_goal));
            });

            if ui.button("Pick Characters").clicked() {
                let mut finished = false;

                while finished != true {
                    randomized_character_list.truncate(0);

                    let mut easy_characters: Vec<Character> = global_easy_character_list.clone();

                    let mut medium_characters: Vec<Character> = global_medium_character_list.clone();

                    let mut hard_characters: Vec<Character> = global_hard_character_list.clone();

                    match preferred_maximum_difficulty {
                        Difficulty::Easy => {
                            easy_characters.shuffle(&mut thread_rng());
                            medium_characters.shuffle(&mut thread_rng());
                            hard_characters.shuffle(&mut thread_rng());

                            randomized_character_list.append(&mut easy_characters);
                            randomized_character_list.append(&mut medium_characters);
                            randomized_character_list.append(&mut hard_characters);

                            randomized_character_list.truncate(*number_of_characters);
                        },
                        Difficulty::Medium => {
                            randomized_character_list.append(&mut easy_characters);
                            randomized_character_list.append(&mut medium_characters);

                            randomized_character_list.shuffle(&mut thread_rng());

                            hard_characters.shuffle(&mut thread_rng());

                            randomized_character_list.append(&mut hard_characters);

                            randomized_character_list.truncate(*number_of_characters);
                        },
                        Difficulty::Hard => {
                            randomized_character_list.append(&mut easy_characters);
                            randomized_character_list.append(&mut medium_characters);
                            randomized_character_list.append(&mut hard_characters);

                            randomized_character_list.shuffle(&mut thread_rng());

                            randomized_character_list.truncate(*number_of_characters);
                        },
                    }

                    let mut num_officers = 0;
                    let mut num_robots = 0;
                    let mut num_artifacts = 0;
                    let mut num_briefcases = 0;
                    let mut num_contaminants = 0;

                    for character in &mut *randomized_character_list {
                        if character.is_officer {
                            num_officers += 1;
                        }

                        if character.is_robot {
                            num_robots += 1;
                        }

                        if character.cares_about_artifact {
                            num_artifacts += 1;
                        }

                        if character.cares_about_briefcase {
                            num_briefcases += 1;
                        }

                        if character.wants_contamintaion {
                            num_contaminants += 1;
                        }
                    }

                    finished = true;

                    if num_officers == 0 && *at_least_one_officer {
                        finished = false;
                    }

                    if num_robots == 0 && *at_least_one_robot {
                        finished = false;
                    }

                    if *at_least_two_agents_per_goal {
                        if num_artifacts == 1 || num_briefcases == 1 || num_contaminants == 1 {
                            finished = false;
                        }
                    }
                }
                
                randomized_character_list.sort();
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Randomized Characters");

            for character in &mut *randomized_character_list {
                ui.label(character.name.as_str());
            }
            
            egui::warn_if_debug_build(ui);
        });
    }
}
