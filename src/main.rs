use penrose::{
    builtin::{
        actions::{
            exit,
            floating::{sink_focused, MouseDragHandler, MouseResizeHandler},
            key_handler, modify_with, spawn,
        },
        hooks::SpacingHook,
    },
    core::{
        bindings::{
            click_handler, parse_keybindings_with_xmodmap, KeyEventHandler, MouseEventHandler,
            MouseState,
        },
        Config, WindowManager,
    },
    extensions::{actions::toggle_fullscreen, hooks::ewmh::add_ewmh_hooks},
    map,
    util::spawn_with_args,
    x11rb::RustConn,
    Result,
};
use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};

const TERM: &str = "alacritty";
const LAUNCHER: &str = "mydmenu_run";
const TAGS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
const FLAMESHOT: &str = "flameshot";
const FLAMESHOT_FULL_ARGS: [&str; 3] = ["full", "-p", "~/Pictures/screenshots"];
const FLAMESHOT_GUI_ARGS: [&str; 3] = ["gui", "-p", "~/Pictures/screenshots"];

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
        h.insert("M-F".to_owned(), toggle_fullscreen());
        h.insert(
            "M-z".to_owned(),
            key_handler(move |_, _| spawn_with_args(FLAMESHOT, &FLAMESHOT_FULL_ARGS)),
        );
        h.insert(
            "M-x".to_owned(),
            key_handler(move |_, _| spawn_with_args(FLAMESHOT, &FLAMESHOT_GUI_ARGS)),
        );

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

    let mut cfg = add_ewmh_hooks(Config::default());
    cfg.focused_border = "#d5c4a1".try_into().expect("invalid border color");
    cfg.border_width = 1;
    cfg.compose_or_set_layout_hook(SpacingHook {
        outer_px: 0,
        inner_px: 0,
        top_px: 0,
        bottom_px: 31,
    });

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;
    let wm = WindowManager::new(cfg, key_bindings, mouse_bindings(), conn)?;

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
