mod dir;

use std::fs;

use clap::Parser;
use crossterm::style::{Color, Stylize};
use pyret_interpreter::{fs::FsGraph, io::Output, value::PyretValue, Interpreter, PyretGraph};
use pyret_number::PyretNumber;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    program: String,
}

fn main() {
    let args = Args::parse();

    let mut graph = FsGraph::new();

    let name = fs::canonicalize(args.program).unwrap();

    let file_id = graph.register(&name.to_string_lossy());

    let mut interpreter = Interpreter::new(graph);

    interpreter
        .context
        .borrow_mut()
        .io
        .read(Box::new(handle_output));

    if let Err(error) = interpreter.import_trove("global") {
        eprintln!("{error:?}");
    }

    match interpreter.interpret(file_id) {
        Ok(values) => {
            for value in values.iter() {
                if value.as_ref() != &PyretValue::Nothing {
                    let color = match &**value {
                        PyretValue::Number(number) => match number {
                            PyretNumber::Exact(_) => Color::Yellow,
                            PyretNumber::Rough(_) => Color::DarkYellow,
                        },
                        PyretValue::String(_) => Color::Cyan,
                        PyretValue::Boolean(_) => Color::DarkMagenta,
                        PyretValue::Function(_) => Color::Reset,
                        PyretValue::Nothing => unreachable!(),
                    };

                    println!("{}", value.as_ref().to_string().with(color));
                }
            }
        }
        Err(errors) => {
            for error in errors {
                eprintln!("{:?}", error.into_report(&*interpreter.graph));
            }
        }
    }

    // Ok(())

    // repl::start();
}

fn handle_output(output: Output) {
    match output {
        Output::Display(value) => {
            if value.as_ref() != &PyretValue::Nothing {
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
