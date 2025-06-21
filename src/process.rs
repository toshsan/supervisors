use crate::supervisor::ProcessStatus;
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::{process::Command, task::JoinHandle};

pub struct ManagedProcess {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub autorestart: bool,
}

impl ManagedProcess {
    pub fn start_with_status_updater(
        self,
        status_map: Arc<Mutex<HashMap<String, ProcessStatus>>>,
    ) -> JoinHandle<()> {
        let command = self.command.clone();
        let args = self.args.clone();
        let autorestart = self.autorestart;
        let name = self.name;

        tokio::spawn(async move {
            loop {
                println!("[{}] Starting: {} {:?}", name, command, args);

                {
                    let mut map = status_map.lock().await;
                    if let Some(status) = map.get_mut(&name) {
                        status.running = true;
                    }
                }

                let mut child = Command::new(&command)
                    .args(&args)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("Failed to spawn process");

                let status = child.wait().await.expect("Failed to wait");
                println!("[{}] Exited with: {}", name, status);

                {
                    let mut map = status_map.lock().await;
                    if let Some(status) = map.get_mut(&name) {
                        status.running = false;
                    }
                }

                if !autorestart {
                    break;
                }

                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            }
        })
    }
}
