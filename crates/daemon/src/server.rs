//! IPC server for daemon communication

use std::path::PathBuf;
use tokio::net::UnixListener;
use tracing::{info, error};
use crate::state::DaemonState;
use threadbare_core::Result;

/// IPC server for the daemon
pub struct DaemonServer {
    socket_path: PathBuf,
    state: DaemonState,
}

impl DaemonServer {
    /// create a new daemon server
    pub fn new(socket_path: PathBuf, state: DaemonState) -> Self {
        Self {
            socket_path,
            state,
        }
    }

    /// start the IPC server
    pub async fn start(&self) -> Result<()> {
        // remove existing socket if it exists
        if self.socket_path.exists() {
            std::fs::remove_file(&self.socket_path)?;
        }

        // create Unix domain socket listener
        let listener = UnixListener::bind(&self.socket_path)?;
        info!("IPC server listening on: {}", self.socket_path.display());

        // accept connections in a loop
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let state = self.state.clone();
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(stream, state).await {
                            error!("Error handling connection: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
            }
        }
    }

    /// handle a single client connection
    async fn handle_connection(
        mut stream: tokio::net::UnixStream,
        _state: DaemonState,
    ) -> Result<()> {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let mut buffer = [0; 1024];
        
        loop {
            let n = stream.read(&mut buffer).await?;
            
            if n == 0 {
                // connection closed
                break;
            }

            // placeholder
            stream.write_all(&buffer[..n]).await?;
        }

        Ok(())
    }
}

