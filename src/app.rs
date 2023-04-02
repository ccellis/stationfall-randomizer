use core::cmp::Ordering;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone, Eq, Ord, PartialOrd)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone, Eq, Ord, PartialOrd)]
pub enum AggressionLevel {
    Peaceful,
    Random,
    Aggressive,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Character {
    pub name: String,
    pub difficulty: Difficulty,
    pub aggression: f32,
    pub is_robot: bool,
    pub is_officer: bool,
    pub cares_about_briefcase: bool,
    pub cares_about_artifact: bool,
    pub wants_contamintaion: bool,
}

impl Eq for Character {}

impl Ord for Character {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Character {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Character {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
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
    preferred_aggression_level: AggressionLevel,
    show_detailed_character_info: bool,
    found_character_list: bool,
    #[serde(skip)]
    global_easy_character_list: Vec<Character>,
    #[serde(skip)]
    global_medium_character_list: Vec<Character>,
    #[serde(skip)]
    global_hard_character_list: Vec<Character>,
    #[serde(skip)]
    randomized_character_list: Vec<Character>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            number_of_characters: 15,
            at_least_one_robot: true,
            at_least_one_officer: true,
            at_least_two_agents_per_goal: true,
            preferred_maximum_difficulty: Difficulty::Hard,
            preferred_aggression_level: AggressionLevel::Random,
            show_detailed_character_info: false,
            found_character_list: true,
            global_easy_character_list: vec![
                Character {
                    name: "Astrochimp".to_string(),
                    difficulty: Difficulty::Easy,
                    aggression: 0.0,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: true,
                    cares_about_artifact: true,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Counselor".to_string(),
                    difficulty: Difficulty::Easy,
                    aggression: 0.0,
                    is_robot: false,
                    is_officer: true,
                    cares_about_briefcase: false,
                    cares_about_artifact: true,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Cyborg".to_string(),
                    difficulty: Difficulty::Easy,
                    aggression: 1.0,
                    is_robot: true,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Daredevil".to_string(),
                    difficulty: Difficulty::Easy,
                    aggression: 1.0,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: true,
                    wants_contamintaion: true,
                },
                Character {
                    name: "Engineer".to_string(),
                    difficulty: Difficulty::Easy,
                    aggression: 1.0,
                    is_robot: false,
                    is_officer: true,
                    cares_about_briefcase: false,
                    cares_about_artifact: true,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Exile".to_string(),
                    difficulty: Difficulty::Easy,
                    aggression: 0.0,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Inspector".to_string(),
                    difficulty: Difficulty::Easy,
                    aggression: 0.5,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: true,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Maintenance Clones".to_string(),
                    difficulty: Difficulty::Easy,
                    aggression: 0.5,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Medical".to_string(),
                    difficulty: Difficulty::Easy,
                    aggression: 0.0,
                    is_robot: true,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Security".to_string(),
                    difficulty: Difficulty::Easy,
                    aggression: 1.0,
                    is_robot: true,
                    is_officer: false,
                    cares_about_briefcase: true,
                    cares_about_artifact: true,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Station Chief".to_string(),
                    difficulty: Difficulty::Easy,
                    aggression: 0.0,
                    is_robot: false,
                    is_officer: true,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Stowaway".to_string(),
                    difficulty: Difficulty::Easy,
                    aggression: 1.0,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Troubleshooter".to_string(),
                    difficulty: Difficulty::Easy,
                    aggression: 0.5,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
            ],
            global_medium_character_list: vec![
                Character {
                    name: "Microbiologist".to_string(),
                    difficulty: Difficulty::Medium,
                    aggression: 0.0,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: true,
                },
                Character {
                    name: "Colonel".to_string(),
                    difficulty: Difficulty::Medium,
                    aggression: 0.5,
                    is_robot: false,
                    is_officer: true,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: true,
                },
                Character {
                    name: "Operative".to_string(),
                    difficulty: Difficulty::Medium,
                    aggression: 1.0,
                    is_robot: false,
                    is_officer: true,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Stranger".to_string(),
                    difficulty: Difficulty::Medium,
                    aggression: 0.0,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Boarder".to_string(),
                    difficulty: Difficulty::Medium,
                    aggression: 1.0,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: true,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Corpsicle".to_string(),
                    difficulty: Difficulty::Medium,
                    aggression: 0.5,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: true,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Doctor".to_string(),
                    difficulty: Difficulty::Medium,
                    aggression: 0.0,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: true,
                    cares_about_artifact: true,
                    wants_contamintaion: true,
                },
                Character {
                    name: "Drones".to_string(),
                    difficulty: Difficulty::Medium,
                    aggression: 0.5,
                    is_robot: true,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
            ],
            global_hard_character_list: vec![
                Character {
                    name: "Legal".to_string(),
                    difficulty: Difficulty::Hard,
                    aggression: 0.0,
                    is_robot: true,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Billionaire".to_string(),
                    difficulty: Difficulty::Hard,
                    aggression: 0.0,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Botanist".to_string(),
                    difficulty: Difficulty::Hard,
                    aggression: 1.0,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Consort".to_string(),
                    difficulty: Difficulty::Hard,
                    aggression: 0.0,
                    is_robot: true,
                    is_officer: false,
                    cares_about_briefcase: true,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Digital Assistant".to_string(),
                    difficulty: Difficulty::Hard,
                    aggression: 0.0,
                    is_robot: false,
                    is_officer: true,
                    cares_about_briefcase: false,
                    cares_about_artifact: false,
                    wants_contamintaion: false,
                },
                Character {
                    name: "Telepathic Rat".to_string(),
                    difficulty: Difficulty::Hard,
                    aggression: 1.0,
                    is_robot: false,
                    is_officer: false,
                    cares_about_briefcase: true,
                    cares_about_artifact: true,
                    wants_contamintaion: false,
                },
            ],
            randomized_character_list: vec![],
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
        let Self {
            number_of_characters,
            at_least_one_robot,
            at_least_one_officer,
            at_least_two_agents_per_goal,
            preferred_maximum_difficulty,
            preferred_aggression_level,
            show_detailed_character_info,
            found_character_list,
            global_easy_character_list,
            global_medium_character_list,
            global_hard_character_list,
            randomized_character_list,
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
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Preferred Maximum Difficulty");
                egui::ComboBox::from_id_source(2)
                    .selected_text(match preferred_maximum_difficulty {
                        Difficulty::Easy => "Easy",
                        Difficulty::Medium => "Medium",
                        Difficulty::Hard => "Hard",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(preferred_maximum_difficulty, Difficulty::Easy, "Easy");
                        ui.selectable_value(
                            preferred_maximum_difficulty,
                            Difficulty::Medium,
                            "Medium",
                        );
                        ui.selectable_value(preferred_maximum_difficulty, Difficulty::Hard, "Hard");
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Preferred Aggression Level");
                egui::ComboBox::from_id_source(3)
                    .selected_text(match preferred_aggression_level {
                        AggressionLevel::Aggressive => "Aggressive",
                        AggressionLevel::Random => "Random",
                        AggressionLevel::Peaceful => "Peaceful",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            preferred_aggression_level,
                            AggressionLevel::Aggressive,
                            "Aggressive",
                        );
                        ui.selectable_value(
                            preferred_aggression_level,
                            AggressionLevel::Random,
                            "Random",
                        );
                        ui.selectable_value(
                            preferred_aggression_level,
                            AggressionLevel::Peaceful,
                            "Peaceful",
                        );
                    });
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
                ui.label("At Least Two Characters Per Goal ");
                ui.add(egui::Checkbox::without_text(at_least_two_agents_per_goal));
            });

            ui.horizontal(|ui| {
                ui.label("Show Detailed Character Info");
                ui.add(egui::Checkbox::without_text(show_detailed_character_info));
            });

            if ui.button("Pick Characters").clicked() {
                let mut finished = false;
                let mut num_tries = 0;

                while !finished {
                    randomized_character_list.truncate(0);

                    num_tries += 1;
                    if num_tries > 1000 {
                        break;
                    }

                    let mut easy_characters: Vec<Character> = global_easy_character_list.clone();

                    let mut medium_characters: Vec<Character> =
                        global_medium_character_list.clone();

                    let mut hard_characters: Vec<Character> = global_hard_character_list.clone();

                    match preferred_maximum_difficulty {
                        Difficulty::Easy => {
                            easy_characters.shuffle(&mut thread_rng());
                            medium_characters.shuffle(&mut thread_rng());
                            hard_characters.shuffle(&mut thread_rng());

                            randomized_character_list.append(&mut easy_characters);
                            randomized_character_list.append(&mut medium_characters);
                            randomized_character_list.append(&mut hard_characters);
                        }
                        Difficulty::Medium => {
                            randomized_character_list.append(&mut easy_characters);
                            randomized_character_list.append(&mut medium_characters);

                            randomized_character_list.shuffle(&mut thread_rng());

                            hard_characters.shuffle(&mut thread_rng());

                            randomized_character_list.append(&mut hard_characters);
                        }
                        Difficulty::Hard => {
                            randomized_character_list.append(&mut easy_characters);
                            randomized_character_list.append(&mut medium_characters);
                            randomized_character_list.append(&mut hard_characters);

                            randomized_character_list.shuffle(&mut thread_rng());
                        }
                    }
                    randomized_character_list.truncate(*number_of_characters);

                    let mut num_officers = 0;
                    let mut num_robots = 0;
                    let mut num_artifacts = 0;
                    let mut num_briefcases = 0;
                    let mut num_contaminants = 0;
                    let mut aggresssion = 0.0;

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

                        aggresssion += character.aggression;
                    }

                    finished = true;

                    if num_officers == 0 && *at_least_one_officer {
                        finished = false;
                    }

                    if num_robots == 0 && *at_least_one_robot {
                        finished = false;
                    }

                    if *at_least_two_agents_per_goal
                        && (num_artifacts == 1 || num_briefcases == 1 || num_contaminants == 1)
                    {
                        finished = false;
                    }

                    aggresssion /= *number_of_characters as f32;

                    if (*preferred_aggression_level == AggressionLevel::Aggressive
                        && aggresssion < 0.43)
                        || (*preferred_aggression_level == AggressionLevel::Peaceful
                            && aggresssion > 0.43)
                    {
                        finished = false;
                    }
                }

                if num_tries > 1000 {
                    *found_character_list = false;
                } else {
                    randomized_character_list.sort();
                    *found_character_list = true;
                }
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
            if *found_character_list && !randomized_character_list.is_empty() {
                if !*show_detailed_character_info {
                    for character in &mut *randomized_character_list {
                        ui.label(character.name.as_str());
                    }
                } else {
                    egui::Grid::new("answer").striped(true).show(ui, |ui| {
                        ui.label("");
                        ui.label("Difficulty");
                        ui.label("Officer");
                        ui.label("Robot");
                        ui.label("Artifact");
                        ui.label("Briefcase");
                        ui.label("Contamination");
                        ui.label("Aggression");
                        ui.end_row();

                        for character in &mut *randomized_character_list {
                            ui.label(character.name.as_str());
                            ui.label(match character.difficulty {
                                Difficulty::Easy => "Easy",
                                Difficulty::Medium => "Medium",
                                Difficulty::Hard => "Hard",
                            });
                            ui.label(match character.is_officer {
                                true => "💳",
                                false => "",
                            });
                            ui.label(match character.is_robot {
                                true => "⚙",
                                false => "",
                            });
                            ui.label(match character.cares_about_artifact {
                                true => "💎",
                                false => "",
                            });
                            ui.label(match character.cares_about_briefcase {
                                true => "💼",
                                false => "",
                            });
                            ui.label(match character.wants_contamintaion {
                                true => "☣",
                                false => "",
                            });
                            ui.label(character.aggression.to_string());
                            ui.end_row();
                        }
                    });
                }
            } else if !*found_character_list {
                ui.label("Could not find character list with given parameters.");
                ui.label("Adjust maximum difficulty or aggression.");
            }

            egui::warn_if_debug_build(ui);
        });
    }
}
