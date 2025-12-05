mod models;

use clap::Parser;
use models::command::*;
use rustyline::DefaultEditor;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();

    let start_repl = std::env::args().len() < 2;

    if start_repl {
        loop {
            let line = rl.readline("> ");
            let line = match line {
                Ok(l) => l.trim().to_string(),
                Err(_) => break,
            };

            if line == "exit" || line == "quit" {
                break;
            }

            let argv = shell_words::split(&line).unwrap_or_default();

            let mut argv_with_prog = vec!["minidb".to_string()];
            argv_with_prog.extend(argv);

            let cli_result = CLI::try_parse_from(&argv_with_prog);

            match cli_result {
                Ok(cli) => {
                    handle_cli_command(cli);
                }
                Err(err) => {
                    _ = err.print();
                }
            }
        }
    } else {
        let cli = CLI::parse();
        handle_cli_command(cli);
    }

    println!("Goodbye!");
}

fn handle_cli_command(cli: CLI) {
    match cli.command {
        Command::Load { ref path } => println!("loading {}", path.display()),
        Command::Dump { ref path } => println!("dumping {}", path.display()),
        Command::Save { ref path } => println!("saving {}", path.display()),
        Command::Create { subcommand } => match subcommand {
            CreateCommand::Table { name, database } => {
                println!("printing table: {}, into database: {}", name, database)
            }
            CreateCommand::Database { name } => println!("creating database {}", name),
        },
    }
}
