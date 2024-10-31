use penrose::{
    builtin::actions::{
        exit,
        floating::{sink_focused, MouseDragHandler, MouseResizeHandler},
        modify_with, spawn,
    },
    core::{
        bindings::{
            click_handler, parse_keybindings_with_xmodmap, KeyEventHandler, MouseEventHandler,
            MouseState,
        },
        Config, WindowManager,
    },
    extensions::hooks::ewmh::add_ewmh_hooks,
    map,
    x11rb::RustConn,
    Result,
};
use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};

const TERM: &str = "alacritty";

const LAUNCHER: &str = "mydmenu_run";

const TAGS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn raw_key_bindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = {
        let mut h = HashMap::new();
        h.insert("M-n".to_owned(), modify_with(|cs| cs.focus_down()));
        h.insert("M-p".to_owned(), modify_with(|cs| cs.focus_up()));
        h.insert("M-S-n".to_owned(), modify_with(|cs| cs.swap_down()));
        h.insert("M-S-p".to_owned(), modify_with(|cs| cs.swap_up()));
        h.insert("M-S-c".to_owned(), modify_with(|cs| cs.kill_focused()));
        h.insert("M-Tab".to_owned(), modify_with(|cs| cs.toggle_tag()));
        h.insert("C-S-space".to_owned(), spawn(LAUNCHER));
        h.insert("M-Return".to_owned(), spawn(TERM));
        h.insert("M-S-q".to_owned(), exit());

        h
    };

    for tag in TAGS.iter() {
        raw_bindings.extend([
            (
                format!("M-{tag}"),
                modify_with(move |client_set| client_set.focus_tag(tag)),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }

    raw_bindings
}

fn mouse_bindings() -> HashMap<MouseState, Box<dyn MouseEventHandler<RustConn>>> {
    use penrose::core::bindings::{
        ModifierKey::{Meta, Shift},
        MouseButton::{Left, Middle, Right},
    };

    map! {
        map_keys: |(button, modifiers)| MouseState { button, modifiers };

        (Left, vec![Shift, Meta]) => MouseDragHandler::boxed_default(),
        (Right, vec![Shift, Meta]) => MouseResizeHandler::boxed_default(),
        (Middle, vec![Shift, Meta]) => click_handler(sink_focused()),
    }
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;
    let wm = WindowManager::new(
        add_ewmh_hooks(Config::default()),
        key_bindings,
        mouse_bindings(),
        conn,
    )?;

    wm.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_parse_correctly_with_xmodmap() {
        parse_keybindings_with_xmodmap(raw_key_bindings()).unwrap();
    }
}
