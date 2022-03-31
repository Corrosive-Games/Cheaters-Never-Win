use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub enum GameStates {
    Main,           // main runner game
    ConsoleLoading, // loading the console
    Console,        // console/terminal
    PauseMenu,      // in-game pause menu
    TabMenuLoading, // loading the journal
    TabMenu,        // journal/stats
    GameOver,       // game over menu
    MainMenu,       // main menu
}
