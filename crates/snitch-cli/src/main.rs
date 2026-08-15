use std::env;
use std::process::ExitCode;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use snitch_adapters::{InMemoryLogStore, OsProcessSupervisor};
use snitch_application::StartSupervisedProcess;
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
    let mut use_case = StartSupervisedProcess::new(&mut supervisor, log_store.clone());

    match use_case.execute(&command) {
        Ok(process_id) => {
            println!("started process {}", process_id.value());

            sleep(Duration::from_millis(500));
            for line in log_store.lines_for(process_id) {
                println!("[{:?}] {}", line.stream(), line.content());
            }

            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
