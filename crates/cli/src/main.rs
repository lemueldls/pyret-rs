mod dir;
mod graph;

use std::fs;

// use pyret_error::term::{
//     termcolor::{ColorChoice, StandardStream},
//     Config,
// };
use clap::Parser;
use graph::FsGraph;
// use pyret_error::miette;
use pyret_interpreter::{io::Output, trove, value::PyretValue, Interpreter, PyretGraph};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    program: String,
}

fn main()
// -> miette::Result<()>
{
    // miette::set_hook(Box::new(|_| {}))?;

    let args = Args::parse();

    let mut graph = FsGraph::new();

    let name = fs::canonicalize(args.program).unwrap();

    let file_id = graph.register(&name.to_string_lossy());

    let mut interpreter = Interpreter::new(graph);

    interpreter
        .context
        .borrow_mut()
        .io
        .read(Box::new(|output| match output {
            Output::Display(value) => {
                if value != &PyretValue::Nothing {
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
            Output::Test => {}
        }));

    if let Err(error) = interpreter.use_context("global") {
        eprintln!("{error:?}");
    }

    match interpreter.interpret(file_id) {
        Ok(values) => {
            for value in values.iter() {
                if value.as_ref() != &PyretValue::Nothing {
                    println!("{value}");
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
