use crate::toast::ShowToast;
use bevy::prelude::*;
use std::time::Duration;

// The introductory tutorial shown to the player at the beginning of a run
pub fn prelude_text(mut toasts: EventWriter<ShowToast>) {
    // empty to avoid issues
    toasts.send(ShowToast {
        value: "Press 'D' to move forward".to_string(),
        duration: Duration::from_secs(3),
    });
    toasts.send(ShowToast {
        value: "Press TAB to open journal".to_string(),
        duration: Duration::from_secs(3),
    });
    toasts.send(ShowToast {
        value: "Collect letters".to_string(),
        duration: Duration::from_secs(3),
    });

    toasts.send(ShowToast {
        value: "Press `E` to interact with terminal".to_string(),
        duration: Duration::from_secs(3),
    });
    toasts.send(ShowToast {
        value: "Use \"cheat <code>\" command...".to_string(),
        duration: Duration::from_secs(2),
    });
    toasts.send(ShowToast {
        value: "...to spend letters...".to_string(),
        duration: Duration::from_secs(2),
    });
    toasts.send(ShowToast {
        value: "...and activate abilities!".to_string(),
        duration: Duration::from_secs(2),
    });
}
