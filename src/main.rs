mod menus;
mod multiplexers;

use std::process::Command;

use cursive::Cursive;
use lazy_static::lazy_static;
use menus::{display_pick_list, display_warning};
use multiplexers::{get_sessions, Session};

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short, help = "Which multiplexer to use")]
    multiplexer: Multiplexer,
}

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum Multiplexer {
    Tmux,
    Screen,
    Zellij,
}

lazy_static! {
    static ref ARGS: Args = Args::parse();
    static ref SESSIONS: Vec<Session> = get_sessions(&ARGS.multiplexer);
}

fn main() {
    display_pick_list();
}

pub(crate) fn check_if_new(tui: &mut Cursive, selection: &str) {
    match selection.contains("New session") {
        true => {
            create_new_session(tui);
        }
        false => {
            check_if_available(tui, selection);
        }
    }
}

pub(crate) fn create_new_session(tui: &mut Cursive) {
    tui.quit();
    // Reset the terminal to rid ourselves of cursive
    let _ = Command::new("reset").status().unwrap();
    let mut command: Command;
    match ARGS.multiplexer {
        Multiplexer::Tmux => command = Command::new("tmux"),
        Multiplexer::Screen => {
            command = Command::new("screen");
        }
        Multiplexer::Zellij => {
            command = Command::new("zellij");
        }
    }
    command.status().expect("The multiplexer failed to start");
}

pub(crate) fn check_if_available(tui: &mut Cursive, selection: &str) {
    tui.pop_layer();
    let session = get_session_from_string(selection.to_string());
    match !session.available {
        true => {
            display_warning(tui, session);
        }
        false => {
            connect_to_session(tui, &session);
        }
    }
}

pub(crate) fn connect_to_session(tui: &mut Cursive, session: &Session) {
    tui.quit();
    // Reset the terminal to rid ourselves of cursive
    let _ = Command::new("reset").status().unwrap();
    let command: &mut Command;
    let mut binding;
    match ARGS.multiplexer {
        Multiplexer::Tmux => {
            let session_id = session.name.split(':').next().unwrap();
            binding = Command::new("tmux");
            command = binding.arg("attach").arg("-t").arg(session_id);
        }
        Multiplexer::Screen => {
            let session_id = session.name.split('.').next().unwrap();
            binding = Command::new("screen");
            command = binding.arg("-rD").arg(session_id);
        }
        Multiplexer::Zellij => {
            let session_id = session.name.split(' ').next().unwrap();
            binding = Command::new("zellij");
            command = binding
                .arg("options")
                .arg("--mirror-session")
                .arg("false")
                .arg("--attach-to-session")
                .arg("true")
                .arg("--session-name")
                .arg(session_id);
        }
    }
    command.status().expect("The multiplexer failed to start");
}

fn get_session_from_string(screen_name: String) -> Session {
    match ARGS.multiplexer {
        Multiplexer::Tmux => {
            let tmux_id = screen_name.split(':').next().unwrap();
            for session in SESSIONS.iter() {
                if session.name.contains(tmux_id) {
                    return session.clone();
                }
            }
            panic!("Could not find tmux session from the ID!");
        }
        Multiplexer::Screen => {
            let session_id = screen_name.split('.').next().unwrap();
            for session in SESSIONS.iter() {
                if session.name.contains(session_id) {
                    return session.clone();
                }
            }
            panic!("Could not find screen session from the ID!");
        }
        Multiplexer::Zellij => {
            let zellij_id = screen_name.split(' ').next().unwrap();
            for session in SESSIONS.iter() {
                if session.name.contains(zellij_id) {
                    return session.clone();
                }
            }
            panic!("Could not find zellij session from the ID!");
        }
    }
}
