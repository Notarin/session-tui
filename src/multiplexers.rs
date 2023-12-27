use lazy_static::lazy_static;
use regex::Regex;
use std::process::Command;

use crate::Multiplexer;

#[derive(Debug, Clone)]
pub(crate) struct Session {
    pub(crate) name: String,
    pub(crate) available: bool,
}

pub(crate) fn get_sessions(multiplexer: &Multiplexer) -> Vec<Session> {
    let mut sessions = match multiplexer {
        Multiplexer::Tmux => get_tmux_sessions(),
        Multiplexer::Screen => get_screen_sessions(),
        Multiplexer::Zellij => get_zellij_sessions(),
    };

    for session in sessions.iter_mut() {
        *session = Session {
            name: remove_ansi_escape_codes(&session.name),
            available: session.available,
        };
    }
    sessions
}

lazy_static! {
    static ref ANSI_ESCAPE_CODE_REGEX: Regex =
        Regex::new(r"\x1B(?:\[[0-?]*[- /]*[@-~]|_[^\\]*;[^\\]*\\)").unwrap();
}

fn remove_ansi_escape_codes(s: &str) -> String {
    let re: &Regex = &ANSI_ESCAPE_CODE_REGEX;
    re.replace_all(s, "").to_string()
}

fn get_tmux_sessions() -> Vec<Session> {
    let raw_sessions: std::process::Output = Command::new("tmux")
        .arg("list-sessions")
        .output()
        .expect("failed to execute process");
    let raw_sessions: String = String::from_utf8(raw_sessions.stdout)
        .expect("tmux responded with a language that only the gods can understand");
    if raw_sessions.is_empty() {
        return Vec::new();
    };
    let raw_sessions = raw_sessions.split('\n');
    let mut sessions: Vec<Session> = Vec::new();
    for raw_session in raw_sessions {
        let mut available: bool = true;
        if raw_session.contains("(attached)") {
            available = false;
        }
        let session = Session {
            name: raw_session.to_string(),
            available,
        };
        sessions.push(session);
    }
    sessions
}

fn get_screen_sessions() -> Vec<Session> {
    let raw_sessions: std::process::Output = Command::new("screen")
        .arg("-ls")
        .output()
        .expect("failed to execute process");
    let raw_sessions: String = String::from_utf8(raw_sessions.stdout)
        .expect("screen responded with a language that only the gods can understand");
    if raw_sessions.contains("No Sockets found in") {
        return Vec::new();
    }
    let raw_sessions: Vec<String> = raw_sessions.split('\n').map(String::from).collect();
    // Remove the first and last line, as they are not sessions
    let raw_sessions = raw_sessions[1..raw_sessions.len() - 2].to_vec();
    let raw_sessions = raw_sessions.iter().map(|session| session.trim());
    let mut sessions: Vec<Session> = Vec::new();
    for raw_session in raw_sessions {
        let mut available: bool = false;
        if raw_session.contains("(Detached)") {
            available = true;
        }
        let session = Session {
            name: raw_session.to_string(),
            available,
        };
        sessions.push(session);
    }
    sessions
}

fn get_zellij_sessions() -> Vec<Session> {
    let raw_sessions: std::process::Output = Command::new("zellij")
        .arg("ls")
        .output()
        .expect("failed to execute process");
    let mut raw_sessions: String = String::from_utf8(raw_sessions.stdout)
        .expect("zellij responded with a language that only the gods can understand");
    raw_sessions = String::from(raw_sessions.trim());
    if raw_sessions.is_empty() {
        return Vec::new();
    };
    let raw_sessions: Vec<String> = raw_sessions.split('\n').map(String::from).collect();
    let mut sessions: Vec<Session> = Vec::new();
    for raw_session in raw_sessions {
        let available: bool = true;
        let session = Session {
            name: raw_session.to_string(),
            available,
        };
        sessions.push(session);
    }
    sessions
}
