mod dir;
mod graph;

use std::{fs};

use clap::Parser;
use crossterm::style::{Color, Stylize};
use graph::FsGraph;
use pyret_error::{
    miette::{self, IntoDiagnostic},
    PyretFile,
};
use pyret_interpreter::{
    io::Output,
    value::{PyretValueKind},
    Interpreter, PyretGraph,
};
use pyret_number::PyretNumber;
use rustyline::{error::ReadlineError, DefaultEditor};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    program: Option<String>,
}

fn main() -> miette::Result<()> {
    let args = Args::parse();

    let mut interpreter = Interpreter::new(FsGraph::default());

    interpreter
        .context
        .borrow_mut()
        .io
        .read(Box::new(handle_output));

    if let Err(error) = interpreter.import_trove("global") {
        eprintln!("{error:?}");
    }

    if let Some(program) = args.program {
        let name = fs::canonicalize(program).unwrap();

        let file_id = interpreter.graph.register(&name.to_string_lossy());

        print_values(&mut interpreter, file_id);
    } else {
        let mut rl = DefaultEditor::new().into_diagnostic()?;

        let history = &dir::join("repl.log");

        let _ = rl.load_history(history);

        println!(
            "{} {}\nexit using ctrl+d",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );

        let prompt = &"\u{203a}\u{203a}\u{203a} ".dim().to_string();

        loop {
            let readline = rl.readline(prompt);

            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str()).into_diagnostic()?;

                    let file_id = interpreter.graph.files.len();

                    interpreter.graph.files.push(PyretFile::new(
                        format!("repl://{file_id}").into_boxed_str(),
                        line.into_boxed_str(),
                    ));

                    print_values(&mut interpreter, file_id);
                }
                Err(ReadlineError::Interrupted) => eprintln!("exit using ctrl+d"),
                Err(ReadlineError::Eof) => break,
                Err(error) => {
                    eprintln!("Error: {error:?}");

                    break;
                }
            }
        }

        rl.save_history(history).into_diagnostic()?;
    }

    Ok(())
}

fn print_values(interpreter: &mut Interpreter<FsGraph>, file_id: usize) {
    match interpreter.interpret(file_id) {
        Ok(values) => {
            for value in values {
                if *value.kind != PyretValueKind::Nothing {
                    let color = match &*value.kind {
                        PyretValueKind::Number(number) => match number {
                            PyretNumber::Exact(_) => Color::Yellow,
                            PyretNumber::Rough(_) => Color::DarkYellow,
                        },
                        PyretValueKind::String(_) => Color::Cyan,
                        PyretValueKind::Boolean(_) => Color::DarkMagenta,
                        PyretValueKind::Function(_) => Color::Grey,
                        PyretValueKind::Nothing => unreachable!(),
                    };

                    println!("{}", value.to_string().with(color));
                }
            }
        }
        Err(errors) => {
            for error in errors {
                eprintln!("{:?}", error.into_report(&interpreter.graph));
            }
        }
    }
}

fn handle_output(output: Output) {
    match output {
        Output::Display(value) => {
            if *value.kind != PyretValueKind::Nothing {
                println!("{value}");
            }
        }
        Output::Print(value) => println!(
            "{}",
            value
                .lines()
                .map(|line| format!(" {line}"))
                .collect::<Vec<_>>()
                .join("\n")
        ),
        Output::Test { label, results } => {
            let passed = results.iter().filter(|result| result.passed).count();
            let total = results.len();

            let all_passed = passed == total;

            let color = if all_passed {
                Color::Green
            } else {
                Color::Yellow
            };

            if let Some(label) = label {
                println!("{}", label.underlined().with(color));
            }

            if all_passed {
                if total == 1 {
                    println!("{}", "The test in this block passed.".italic());
                } else {
                    println!(
                        "{}",
                        format!("All {total} tests in this block passed.").italic()
                    );
                }
            } else if passed == 0 {
                if total == 1 {
                    eprintln!("{}", "The test in this block failed.".italic());
                } else {
                    eprintln!(
                        "{}",
                        format!("All {total} tests in this block failed.").italic()
                    );
                }
            } else {
                eprintln!("{passed} out of {total} test passed in this block");
            }

            for (i, result) in results.iter().enumerate() {
                let i = i + 1;

                if result.passed {
                    println!(
                        "  {} {}",
                        format!("Test {i}:").underlined().green(),
                        "Passed".green()
                    );
                } else {
                    eprintln!(
                        "  {} {}",
                        format!("Test {i}:").underlined().yellow(),
                        "Failed".yellow()
                    );
                }
            }

            println!();
        }
    }
}

// use std::fs;

// use pyret_lexer::{
//     error::{ColorChoice, Config, StandardStream},
//     file::PyretFile,
//     Lexer,
// };

// fn main() {
//     let filename = fs::canonicalize("test.arr").expect("Could not find
// file");

//     let name = format!("file://{}", filename.to_string_lossy());
//     let source = fs::read_to_string(filename).expect("Could not read file");

//     let input = PyretFile::new(name, source);

//     // let lexer = Lexer::new();

//     match Lexer::lex(input) {
//         Ok(tokens) => {
//             dbg!(tokens);
//         }
//         Err(errors) => {
//             let writer = StandardStream::stderr(ColorChoice::Auto);
//             let config = Config::default();

//             for error in errors {
//                 error.emit(&writer, &config);
//             }
//         }
//     }
// }
