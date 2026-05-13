//! frostgfx engine *stub* process.
//!
//! Reads newline-delimited JSON commands from stdin, writes
//! newline-delimited JSON responses + events to stdout. Stderr is reserved
//! for human-readable logs (the client doesn't parse it).
//!
//! Replace this binary with the real frostgfx engine compiled in
//! "stdio-IPC" mode; the wire protocol stays identical.

use std::io::{self, BufRead, Write};

use anyhow::Context;
use frostgfx_ipc_shared::{ClientFrame, Command, EngineState, Message, Status};

struct EngineStub {
    state: EngineState,
}

impl EngineStub {
    fn new() -> Self {
        Self { state: EngineState::Created }
    }

    fn handle(&mut self, cmd: Command, id: u64, out: &mut impl Write) -> anyhow::Result<bool> {
        let (msg, keep_running) = match cmd {
            Command::Initialize { window_title, width, height, headless } => {
                eprintln!("[engine] initialize: {window_title} {width}x{height} headless={headless}");
                self.state = EngineState::Ready;
                (response_state(id, Status::Ok, self.state), true)
            }
            Command::LoadScene { scene_uri, preview_only } => {
                eprintln!("[engine] load_scene: {scene_uri} preview={preview_only}");
                if matches!(self.state, EngineState::Created) {
                    (response_err(id, "initialize before load_scene"), true)
                } else {
                    self.state = EngineState::SceneLoaded;
                    write_line(out, &state_changed_event(self.state))?;
                    (response_state(id, Status::Ok, self.state), true)
                }
            }
            Command::UpdateCamera { .. } => {
                if self.state == EngineState::Created {
                    (response_err(id, "engine not ready"), true)
                } else {
                    (Message::Response { id, status: Status::Ok, error: None, state: None }, true)
                }
            }
            Command::GetState => (response_state(id, Status::Ok, self.state), true),
            Command::Shutdown => {
                self.state = EngineState::ShuttingDown;
                (response_state(id, Status::Ok, self.state), false)
            }
        };
        write_line(out, &msg)?;
        Ok(keep_running)
    }
}

fn response_state(id: u64, status: Status, state: EngineState) -> Message {
    Message::Response { id, status, error: None, state: Some(state) }
}

fn response_err(id: u64, msg: &str) -> Message {
    Message::Response {
        id,
        status: Status::Error,
        error: Some(msg.into()),
        state: None,
    }
}

fn state_changed_event(state: EngineState) -> Message {
    Message::Event {
        name: "StateChanged".into(),
        payload: serde_json::json!({ "new_state": state }),
    }
}

fn write_line(out: &mut impl Write, msg: &Message) -> anyhow::Result<()> {
    let s = serde_json::to_string(msg).context("serialize message")?;
    out.write_all(s.as_bytes())?;
    out.write_all(b"\n")?;
    out.flush()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    let mut engine = EngineStub::new();

    eprintln!("[engine] ready, waiting for commands on stdin");

    for line in stdin.lock().lines() {
        let line = line.context("read stdin")?;
        if line.trim().is_empty() {
            continue;
        }
        let frame: ClientFrame = match serde_json::from_str(&line) {
            Ok(f) => f,
            Err(e) => {
                write_line(&mut stdout, &response_err(0, &format!("parse error: {e}")))?;
                continue;
            }
        };
        let id = frame.id;
        if !engine.handle(frame.command, id, &mut stdout)? {
            break;
        }
    }

    eprintln!("[engine] shutdown clean");
    Ok(())
}
