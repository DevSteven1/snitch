use std::env;
use std::process::ExitCode;

use snitch_adapters::OsProcessSupervisor;
use snitch_application::StartSupervisedProcess;
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
    let mut use_case = StartSupervisedProcess::new(&mut supervisor);

    match use_case.execute(&command) {
        Ok(process_id) => {
            println!("started process {}", process_id.value());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
