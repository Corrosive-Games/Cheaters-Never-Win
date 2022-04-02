use rand::seq::IteratorRandom;
use std::collections::HashMap;

const WORD_LIST: [&'static str; 12] = [
    "till", "rich", "weak", "mode", "upon", "core", "dawn", "tiny", "zero", "kick", "back", "show",
];

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
    InadequateKeycaps(CheatCodeKind),
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
            CheatCodeActivationResult::NotFound => "invalid code given".to_string(),
            CheatCodeActivationResult::InadequateKeycaps(kind) => {
                return format!("Inadequate keycaps  for [{:?}]", kind)
            }
        }
    }
}

pub struct CheatCodesResource {
    pub codes: HashMap<CheatCodeKind, CheatCode>,
}

impl CheatCodesResource {
    // attempt to activate code with given input text string
    pub fn activate_code(&mut self, text: &str) -> CheatCodeActivationResult {
        // iteration over all the existing codes
        for (_, code) in self.codes.iter_mut() {
            // check if text entered matches cheat code for CheatCodeKind
            if code.text.eq(&text.to_lowercase()) {
                // check if once code is already activated
                match code.activation {
                    CheatCodeActivation::Multiple => {
                        // activate the code
                        code.is_active = true;
                        return CheatCodeActivationResult::Activated(code.kind.clone());
                    }
                    CheatCodeActivation::Once => {
                        if code.is_active {
                            return CheatCodeActivationResult::AlreadyActivated(code.kind.clone());
                        } else {
                            code.is_active = true;
                            return CheatCodeActivationResult::Activated(code.kind.clone());
                        }
                    }
                }
            }
        }
        CheatCodeActivationResult::NotFound
    }

    // create cheat codes resource
    pub fn new() -> Self {
        let mut codes: HashMap<CheatCodeKind, CheatCode> = HashMap::new();
        let mut word_list: Vec<&str> = WORD_LIST.to_vec();

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
            println!("{}", code.text);
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
pub fn generate_random_code(mut word_list: &mut Vec<&str>) -> String {
    let mut random_code = "".to_string();
    for i in 0..3 {
        let wl = word_list.clone();
        let (j, word) = wl
            .iter()
            .enumerate()
            .choose(&mut rand::thread_rng())
            .unwrap();

        word_list.remove(j);
        random_code += &word;

        if word_list.is_empty() {
            *word_list = WORD_LIST.to_vec();
        }

        if i != 2 {
            random_code += "-";
        }
    }

    random_code.to_string()
}
