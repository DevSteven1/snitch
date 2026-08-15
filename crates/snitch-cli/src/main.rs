use std::env;
use std::process::ExitCode;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use snitch_adapters::{InMemoryLogStore, OsProcessSupervisor};
use snitch_application::{StartSupervisedProcess, WaitForSupervisedProcess};
use snitch_domain::log::LogStore;
use snitch_domain::process::Command;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);

    let Some(program) = args.next() else {
        println!("snitch {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    };

    let command_args: Vec<String> = args.collect();
    let command = match Command::new(program, command_args) {
        Ok(command) => command,
        Err(error) => {
            eprintln!("invalid command: {error}");
            return ExitCode::FAILURE;
        }
    };

    let mut supervisor = OsProcessSupervisor::new();
    let log_store: Arc<dyn LogStore> = Arc::new(InMemoryLogStore::new());

    let process_id = {
        let mut start = StartSupervisedProcess::new(&mut supervisor, log_store.clone());
        match start.execute(&command) {
            Ok(process_id) => process_id,
            Err(error) => {
                eprintln!("{error}");
                return ExitCode::FAILURE;
            }
        }
    };
    println!("started process {}", process_id.value());

    let exit_status = {
        let mut wait = WaitForSupervisedProcess::new(&mut supervisor);
        match wait.execute(process_id) {
            Ok(exit_status) => exit_status,
            Err(error) => {
                eprintln!("{error}");
                return ExitCode::FAILURE;
            }
        }
    };

    // The stdout/stderr reader threads finish shortly after the child's
    // pipes close on exit; a short grace period lets them flush their
    // last lines into the log store before it is read here.
    sleep(Duration::from_millis(100));
    for line in log_store.lines_for(process_id) {
        println!("[{:?}] {}", line.stream(), line.content());
    }

    if exit_status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
