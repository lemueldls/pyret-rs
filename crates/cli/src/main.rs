mod bindings;
mod dir;
mod error;
mod inspector;
mod repl;
mod runtime;

#[macro_use]
extern crate clap;

use runtime::Runtime;

use std::{fs, path::PathBuf, sync::mpsc::channel, time::Duration, time::Instant};

use pyret_transpiler::Transpiler;

use ansi_term::{Color, Style};
use clap::{Parser, Subcommand};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use rand::{seq::SliceRandom, thread_rng};

#[derive(Parser)]
#[clap(name = "Pyret Rust", version)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Build(BuildCommand),
    Run(RunCommand),

    Eval(EvalCommand),

    /// Read Eval Print Loop
    Repl,
}

// /// Build a Pyret program
// #[derive(Args)]
// struct BuildCommand {
//     /// Program to run
//     #[clap(value_name = "FILE")]
//     program: String,

//     /// File to compile the output into
//     #[clap(short, long, value_name = "PATH")]
//     outfile: Option<String>,

//     /// Transpile code without running it
//     #[clap(short, long)]
//     transpile_only: bool,

//     /// Watch for file changes
//     #[clap(short, long)]
//     watch: bool,
// }

/// Run a Pyret program
#[derive(Args)]
struct RunCommand {
    /// Program to run
    #[clap(value_name = "FILE")]
    program: String,

    /// File to compile the output into
    #[clap(short, long, value_name = "PATH")]
    outfile: Option<String>,

    /// Transpile code without running it
    #[clap(short, long)]
    transpile_only: bool,

    /// Watch for file changes
    #[clap(short, long)]
    watch: bool,
}

/// Evaluate Pyret from the command line.
#[derive(Args)]
struct EvalCommand {
    /// Code to evaluate
    code: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Run(ref args) => {
            run(args);

            if args.watch {
                // Create a channel to receive the events.
                let (sender, receiver) = channel();

                // Create a watcher object, delivering debounced events.
                // The notification back-end is selected based on the platform.
                let mut watcher = watcher(sender, Duration::from_millis(1)).unwrap();

                // Add a path to be watched. All files and directories at that path and
                // below will be monitored for changes.
                let path = PathBuf::from(&args.program).canonicalize().unwrap();
                watcher.watch(path, RecursiveMode::Recursive).unwrap();

                loop {
                    match receiver.recv() {
                        Ok(event) => {
                            if let DebouncedEvent::Write(..) = event {
                                run(args)
                            }
                        }
                        Err(error) => {
                            eprintln!("{}\n{error:?}", Color::Red.paint("Error watching program:"))
                        }
                    }
                }
            }
        }
        Commands::Eval(args) => eval(args),
        Commands::Repl => repl::start(),
    }
}

fn run(args: &RunCommand) {
    match fs::read_to_string(&args.program) {
        Ok(code) => {
            let file = PathBuf::from(&args.program).canonicalize().unwrap();

            let filename = format!("file://{}", file.to_string_lossy());

            let mut transpiler = Transpiler::default();

            let start = Instant::now();
            let code = transpiler.transpile(filename.clone(), code);
            let duration = start.elapsed();

            let mut rng = thread_rng();

            match code {
                Ok(code) => {
                    // Check if it should write the compiler output into a file
                    if let Some(path) = &args.outfile {
                        let path = PathBuf::from(path);

                        // Keep the convention of .jarr file extensions
                        let extension = "jarr";

                        let outfile = if path.is_dir() {
                            // If the output is a folder, write a file in that
                            // directory with the same name, but different extension
                            path.join(file.file_name().unwrap())
                                .with_extension(extension)
                        } else if path.extension().is_some() {
                            path
                        } else {
                            path.with_extension(extension)
                        };

                        let write = fs::write(outfile, code.clone());
                        if let Err(err) = write {
                            eprintln!("{err}");
                        }
                    }

                    // Print a nice message when we're not running the file.
                    if args.transpile_only {
                        let ok = ["Ahoy", "Arr", "Begad", "Blimey", "Sail ho"];

                        println!(
                            "{}",
                            Color::Green.paint(format!(
                                "\n{}! {}",
                                ok.choose(&mut rng).unwrap(),
                                Style::new()
                                    .bold()
                                    .paint(format!("Done in {}ms", duration.as_millis()))
                            ))
                        );
                    } else {
                        execute(filename, code);
                    }
                }
                Err(error) => {
                    let err = [
                        "Aaaarrrrgggghhhh",
                        "Avast ye",
                        "Blow me down",
                        "Shiver me timbers",
                        "Sink me",
                    ];

                    eprintln!(
                        "\n{} {}",
                        Color::Red.paint(format!("{}!", err.choose(&mut rng).unwrap())),
                        error
                    );
                }
            };
        }
        Err(..) => eprintln!("{}", Color::Red.paint("Could not read file")),
    }
}

fn eval(args: EvalCommand) {
    let filename = String::from("eval://");

    let code = {
        let mut transpiler = Transpiler::default();

        transpiler.transpile(filename.clone(), args.code)
    };

    match code {
        Ok(code) => execute(filename, code),
        Err(err) => eprintln!("{}", err),
    }
}

fn execute(filename: String, code: String) {
    let mut runtime = Runtime::new();

    let result = runtime.execute_script(&filename, &code);
    if let Err(error) = result {
        eprintln!("{error}")
    };
}
