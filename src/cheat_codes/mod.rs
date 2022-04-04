use crate::player::Inventory;
use rand::seq::IteratorRandom;
use ron::de::from_bytes;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum CheatCodeKind {
    Jump,
    MoveLeft,
    SpeedBoost,
    Dash,
    // TODO: add double jump which is dependent on jump
    //DoubleJump,
    ExtraLife,
}

pub enum CheatCodeActivation {
    Multiple,
    Once,
}

pub struct CheatCode {
    pub kind: CheatCodeKind,
    // TODO: implement code rarity
    //pub rarity: CheatCodeRarity,
    pub text: String,
    // TODO: add cheat code dependencies
    //pub dependencies: Vec<CheatCodeKind>,
    pub image: String,
    pub help_text: String,
    pub activation: CheatCodeActivation,
    pub is_active: bool,
}

impl CheatCode {
    pub fn new(
        kind: CheatCodeKind,
        text: &str,
        image: String,
        help_text: String,
        activation: CheatCodeActivation,
        is_active: bool,
    ) -> Self {
        Self {
            kind,
            text: text.to_string(),
            image,
            help_text,
            activation,
            is_active,
        }
    }
}

pub enum CheatCodeActivationResult {
    NotFound,
    Activated(CheatCodeKind),
    AlreadyActivated(CheatCodeKind),
    InadequateInventory(CheatCodeKind),
}
impl CheatCodeActivationResult {
    pub fn repr(&self) -> String {
        match self {
            CheatCodeActivationResult::Activated(kind) => {
                return format!("[{:?}] successfully activated", kind)
            }
            CheatCodeActivationResult::AlreadyActivated(kind) => {
                return format!("[{:?}] is already active", kind)
            }
            CheatCodeActivationResult::NotFound => "Invalid code given".to_string(),
            CheatCodeActivationResult::InadequateInventory(kind) => {
                return format!("Inadequate inventory for [{:?}]", kind)
            }
        }
    }
}

pub struct CheatCodesResource {
    pub codes: HashMap<CheatCodeKind, CheatCode>,
}

impl CheatCodesResource {
    pub fn is_code_active(&self, kind: &CheatCodeKind) -> bool {
        self.codes.get(kind).unwrap().is_active
    }

    // attempt to activate code with given input text string
    pub fn activate_code(
        &mut self,
        text: &str,
        inventory: &mut Inventory,
    ) -> CheatCodeActivationResult {
        // iteration over all the existing codes
        for (_, code) in self.codes.iter_mut() {
            // check if text entered matches cheat code for CheatCodeKind
            if code.text.eq(&text.to_lowercase()) {
                // TODO: check if player has inventory for code
                if Self::check_inventory(code.text.clone(), inventory) {
                    // check if once code is already activated
                    match code.activation {
                        CheatCodeActivation::Multiple => {
                            // activate the code
                            code.is_active = true;
                            return CheatCodeActivationResult::Activated(code.kind);
                        }
                        CheatCodeActivation::Once => {
                            if code.is_active {
                                return CheatCodeActivationResult::AlreadyActivated(code.kind);
                            } else {
                                code.is_active = true;
                                return CheatCodeActivationResult::Activated(code.kind);
                            }
                        }
                    }
                } else {
                    return CheatCodeActivationResult::InadequateInventory(code.kind);
                }
            }
        }
        CheatCodeActivationResult::NotFound
    }

    fn check_inventory(code_text: String, inventory: &mut Inventory) -> bool {
        // first check words
        let code_words: Vec<&str> = code_text.split('-').collect();

        let mut found_words: Vec<&str> = Vec::new();
        for word in code_words.clone() {
            if let Some(&word_inventory) = inventory.words.get(word) {
                // if a number is found for it
                if word_inventory != 0 {
                    found_words.push(word);
                }
            }
        }

        // find remaining words after checking the player's word inventory
        let remaining_words: Vec<&str> = code_words
            .clone()
            .into_iter()
            .filter(|item| !found_words.contains(item))
            .collect();
        println!("remaining words: {:?}", remaining_words);
        // then check keycaps
        let mut found_keycaps: Vec<char> = Vec::new();
        for word in remaining_words {
            for c in word.chars() {
                println!("checking for {}", c);
                if let Some(&keycap_inventory) = inventory.keycaps.get(&c) {
                    if keycap_inventory != 0 {
                        found_keycaps.push(c);
                    } else {
                        println!("{} count is 0", c);
                        return false;
                    }
                } else {
                    println!("{} not found in inventory", c);
                    return false;
                }
            }
        }

        // remove found words and keycaps from inventory
        for word in found_words {
            *inventory.words.get_mut(word).unwrap() -= 1;
        }
        println!("found keycaps: {:?}", found_keycaps);
        for keycap in found_keycaps {
            *inventory.keycaps.get_mut(&keycap).unwrap() -= 1;
        }
        true
    }

    // create cheat codes resource
    pub fn new() -> Self {
        let mut codes: HashMap<CheatCodeKind, CheatCode> = HashMap::new();
        let mut word_list =
            from_bytes::<Vec<&str>>(include_bytes!("../../data/word_list.ron")).unwrap();

        insert_cheat(
            &mut word_list,
            &mut codes,
            CheatCodeKind::Jump,
            "jump.png",
            "Press the spacebar to jump.",
            CheatCodeActivation::Once,
            false,
        );

        insert_cheat(
            &mut word_list,
            &mut codes,
            CheatCodeKind::MoveLeft,
            "move_left.png",
            "Press 'A' to move left.",
            CheatCodeActivation::Once,
            false,
        );
        insert_cheat(
            &mut word_list,
            &mut codes,
            CheatCodeKind::SpeedBoost,
            "speed.png",
            "Movement enhanced.",
            CheatCodeActivation::Multiple,
            false,
        );

        insert_cheat(
            &mut word_list,
            &mut codes,
            CheatCodeKind::Dash,
            "dash.png",
            "Double tap 'D' to dash.",
            CheatCodeActivation::Once,
            false,
        );

        insert_cheat(
            &mut word_list,
            &mut codes,
            CheatCodeKind::ExtraLife,
            "extra_life.png",
            "Extra life granted.",
            CheatCodeActivation::Multiple,
            false,
        );

        for (_, code) in codes.iter() {
            println!("{:?}: {}", code.kind, code.text);
        }

        Self { codes }
    }
}

fn insert_cheat(
    word_list: &mut Vec<&str>,
    codes: &mut HashMap<CheatCodeKind, CheatCode>,
    kind: CheatCodeKind,
    image_path: &str,
    help_text: &str,
    activation: CheatCodeActivation,
    is_active: bool,
) {
    codes.insert(
        kind,
        CheatCode::new(
            kind,
            &generate_random_code(word_list),
            image_path.to_string(),
            help_text.to_string(),
            activation,
            is_active,
        ),
    );
}

// generate code from three words from word list
pub fn generate_random_code(word_list: &mut Vec<&str>) -> String {
    let mut random_code = "".to_string();
    for i in 0..3 {
        let wl = word_list.clone();
        let (j, word) = wl
            .iter()
            .enumerate()
            .choose(&mut rand::thread_rng())
            .unwrap();

        word_list.remove(j);
        random_code += word;

        if word_list.is_empty() {
            *word_list =
                from_bytes::<Vec<&str>>(include_bytes!("../../data/word_list.ron")).unwrap();
        }

        if i != 2 {
            random_code += "-";
        }
    }

    random_code.to_string()
}
