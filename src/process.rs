use crate::supervisor::ProcessStatus;
use std::collections::HashMap;
use std::sync::Arc;
use std::{fs::OpenOptions, path::PathBuf, process::Stdio};
use tokio::sync::Mutex;
use tokio::{process::Command, task::JoinHandle};

pub struct ManagedProcess {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub autorestart: bool,
    pub logfile: Option<String>,
}

impl ManagedProcess {
    pub fn start_with_status_updater(
        self,
        status_map: Arc<Mutex<HashMap<String, ProcessStatus>>>,
    ) -> JoinHandle<()> {
        let command = self.command.clone();
        let args = self.args.clone();
        let autorestart = self.autorestart;
        let logfile = self.logfile.clone();
        let name = self.name.clone();

        tokio::spawn(async move {
            loop {
                println!("[{}] Starting: {} {:?}", name, command, args);

                let (stdout, stderr) = if let Some(path) = logfile.as_ref() {
                    let log_path = PathBuf::from(path);
                    let log_file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&log_path)
                        .expect("Failed to open logfile");

                    let stdout = Stdio::from(log_file.try_clone().expect("clone stdout"));
                    let stderr = Stdio::from(log_file);
                    (stdout, stderr)
                } else {
                    (Stdio::inherit(), Stdio::inherit()) // ✅ Fallback to stderr/stdout
                };

                {
                    let mut map = status_map.lock().await;
                    if let Some(status) = map.get_mut(&name) {
                        status.running = true;
                    }
                }

                let mut child = Command::new(&command)
                    .args(&args)
                    .stdout(stdout)
                    .stderr(stderr)
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
