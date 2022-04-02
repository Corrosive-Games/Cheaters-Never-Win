use crate::audio::{GameAudioOptions, GameAudioState};
use crate::{
    cheat_codes::{CheatCodeActivationResult, CheatCodesResource},
    game_states::GameStates,
    player::Player,
};

use super::CheatCodeActivatedEvent;
use super::{event::*, ConsoleData};
use bevy::prelude::*;

pub fn command_handler(
    mut cmd_reader: EventReader<SendCommandEvent>,
    mut print_to_console: EventWriter<PrintToConsoleEvent>,
    mut data: ResMut<ConsoleData>,
    mut game_state: ResMut<State<GameStates>>,
    mut cheat_codes_res: ResMut<CheatCodesResource>,
    mut player_query: Query<&mut Player>,
    mut ev_writer: EventWriter<CheatCodeActivatedEvent>,
    mut game_audio_state: ResMut<GameAudioState>,
) {
    for SendCommandEvent(command) in cmd_reader.iter() {
        // skip if the command is empty
        if command.is_empty() {
            return;
        }

        // extracting args
        let args: Vec<&str> = command.trim().split(' ').collect();

        // show the command entered by the user if it's not a clear
        if args[0] != "clear" {
            let mut user_input = String::from("> ");
            user_input.push_str(command.clone().trim());
            print_to_console.send(PrintToConsoleEvent(user_input));
        }

        // dispatch the command
        match args[0] {
            "clear" => data.lines.clear(),
            "help" => print_to_console.send(PrintToConsoleEvent(super::utils::display_help())),
            "cheat" => {
                if let Some(arg) = args.get(1) {
                    print_to_console.send(PrintToConsoleEvent(format!(
                        "Activating cheat code: <{}>...",
                        arg
                    )));

                    // assumes there is only one player in the game
                    let mut player = player_query.iter_mut().next().unwrap();

                    // check if entered code can be activated
                    let activation_res = cheat_codes_res.activate_code(arg, &mut player.inventory);
                    print_to_console.send(PrintToConsoleEvent(format!(
                        "Activation result: {}",
                        activation_res.repr()
                    )));

                    // if the code can be activated play sound and send event
                    if let CheatCodeActivationResult::Activated(kind) = activation_res {
                        ev_writer.send(CheatCodeActivatedEvent(kind));
                        game_audio_state.queue_sound(
                            "powerup-sound".to_owned(),
                            GameAudioOptions {
                                ..Default::default()
                            },
                        );
                    }
                } else {
                    print_to_console.send(PrintToConsoleEvent("Hey..idiot...".to_string()));
                }

                // TODO: remove
                let mut player = player_query.iter_mut().next().unwrap();
                println!("{:?}", player.inventory);
            }
            "exit" => {
                print_to_console.send(PrintToConsoleEvent("Closing session...".to_string()));
                game_state.pop().unwrap();
            }
            "log" => print_to_console.send(PrintToConsoleEvent(super::utils::display_random_log())),
            _ => {
                print_to_console.send(PrintToConsoleEvent(format!(
                    "Command \"{}\" not found.\nType \"help\" to print the list of available commands.",
                    args[0]
                )));
            }
        }
    }
}
