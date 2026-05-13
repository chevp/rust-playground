//! Wire types shared by the IPC client and engine-stub.
//!
//! These mirror the subset of frostgfx commands / responses / events that the
//! POC exercises. Mapping back to the C++ API:
//!
//! | This crate              | frostgfx C++ type                       |
//! |-------------------------|-----------------------------------------|
//! | `Command::Initialize`   | `coregfx::api::CmdInitialize`           |
//! | `Command::LoadScene`    | `coregfx::api::CmdLoadScene`            |
//! | `Command::UpdateCamera` | `coregfx::api::CmdUpdateCamera`         |
//! | `Command::GetState`     | `coregfx::api::CmdGetState`             |
//! | `Command::Shutdown`     | `coregfx::api::CmdShutdown`             |
//! | `Response`              | `coregfx::api::Response`                |
//! | `Event`                 | `coregfx::api::FrostEvent` (planned)    |

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Command {
    Initialize {
        window_title: String,
        width: u32,
        height: u32,
        #[serde(default)]
        headless: bool,
    },
    LoadScene {
        scene_uri: String,
        #[serde(default)]
        preview_only: bool,
    },
    UpdateCamera {
        pos: [f32; 3],
        rot: [f32; 3],
        fov: f32,
        znear: f32,
        zfar: f32,
    },
    GetState,
    Shutdown,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum EngineState {
    Created,
    Ready,
    SceneLoaded,
    Running,
    ShuttingDown,
    Destroyed,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Ok,
    Error,
    NotReady,
    NotFound,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Message {
    /// Response to a specific client command (correlated by `id`).
    Response {
        id: u64,
        status: Status,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        error: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        state: Option<EngineState>,
    },
    /// Asynchronous engine-side event, not tied to any command.
    Event { name: String, payload: serde_json::Value },
}

/// A command annotated with the id used to correlate the response.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientFrame {
    pub id: u64,
    #[serde(flatten)]
    pub command: Command,
}
