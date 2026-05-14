//! frostgfx IPC client.
//!
//! Spawns `frostgfx-engine-stub` as a child process, drives it through a
//! short scripted session, prints what comes back. In a real app this would
//! be a long-lived client owning the engine subprocess.

use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command as Proc, Stdio};

use anyhow::{anyhow, Context};
use frostgfx_ipc_shared::{ClientFrame, Command, Message, Status};

fn engine_binary() -> anyhow::Result<PathBuf> {
    // Sibling binary in the same cargo target dir.
    let me = std::env::current_exe().context("current_exe")?;
    let dir = me
        .parent()
        .ok_or_else(|| anyhow!("no parent dir for exe"))?;
    let mut path = dir.join("frostgfx-engine-stub");
    if cfg!(windows) {
        path.set_extension("exe");
    }
    if !path.exists() {
        return Err(anyhow!(
            "engine-stub binary not found at {}: build it with `cargo build -p frostgfx-engine-stub`",
            path.display()
        ));
    }
    Ok(path)
}

fn main() -> anyhow::Result<()> {
    let bin = engine_binary()?;
    println!("spawning engine: {}", bin.display());

    let mut child = Proc::new(&bin)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .context("spawn engine-stub")?;

    let mut stdin = child.stdin.take().ok_or_else(|| anyhow!("no stdin"))?;
    let stdout = child.stdout.take().ok_or_else(|| anyhow!("no stdout"))?;
    let mut reader = BufReader::new(stdout);

    let session = [
        Command::Initialize {
            window_title: "frostgfx-ipc-poc".into(),
            width: 1280,
            height: 720,
            headless: true,
        },
        Command::LoadScene {
            scene_uri: "scenes/hello.scene.xml".into(),
            preview_only: false,
        },
        Command::UpdateCamera {
            pos: [0.0, 1.7, -5.0],
            rot: [0.0, 0.0, 0.0],
            fov: 60.0,
            znear: 0.1,
            zfar: 1000.0,
        },
        Command::GetState,
        Command::Shutdown,
    ];

    for (i, cmd) in session.into_iter().enumerate() {
        let id = (i + 1) as u64;
        let frame = ClientFrame { id, command: cmd };
        let line = serde_json::to_string(&frame)?;
        println!("→ {line}");
        stdin.write_all(line.as_bytes())?;
        stdin.write_all(b"\n")?;
        stdin.flush()?;

        // Drain every message that came back since the previous send. The
        // engine may interleave Event messages with the Response we asked for.
        let want_id = id;
        loop {
            let mut buf = String::new();
            let n = reader.read_line(&mut buf)?;
            if n == 0 {
                break; // engine closed stdout
            }
            let trimmed = buf.trim();
            if trimmed.is_empty() {
                continue;
            }
            let msg: Message = serde_json::from_str(trimmed)
                .with_context(|| format!("parse engine message: {trimmed}"))?;
            println!("← {trimmed}");
            if let Message::Response { id, status, .. } = &msg {
                if *id == want_id {
                    if !matches!(status, Status::Ok) {
                        return Err(anyhow!("command {id} failed: {msg:?}"));
                    }
                    break;
                }
            }
        }
    }

    drop(stdin); // signal EOF
    let status = child.wait()?;
    println!("engine exited: {status}");
    Ok(())
}
