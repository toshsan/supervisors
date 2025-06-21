use crate::config::Config;
use crate::process::ManagedProcess;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Clone)]
pub struct ProcessStatus {
    pub name: String,
    pub running: bool,
}

#[derive(Clone)]
pub struct SupervisorState {
    pub statuses: Arc<Mutex<HashMap<String, ProcessStatus>>>,
}

pub type ProcessMap = Arc<Mutex<HashMap<String, tokio::task::JoinHandle<()>>>>;

pub async fn run_supervisor(config: &Config) -> SupervisorState {
    let process_map: ProcessMap = Arc::new(Mutex::new(HashMap::new()));
    let status_map: Arc<Mutex<HashMap<String, ProcessStatus>>> =
        Arc::new(Mutex::new(HashMap::new()));

    for (name, program) in &config.programs {
        let cmd = program.command.clone();
        let args = program.args.clone().unwrap_or_default();
        let autorestart = program.autorestart.unwrap_or(false);

        let status_entry = ProcessStatus {
            name: name.clone(),
            running: true,
        };
        status_map.lock().await.insert(name.clone(), status_entry);

        let status_map_clone = status_map.clone();

        let proc = ManagedProcess {
            name: name.clone(),
            command: cmd,
            args,
            autorestart,
        };

        let handle = proc.start_with_status_updater(status_map_clone);
        process_map.lock().await.insert(name.clone(), handle);
    }

    SupervisorState {
        statuses: status_map,
    }
}
