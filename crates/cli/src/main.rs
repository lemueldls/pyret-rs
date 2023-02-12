mod dir;
mod graph;

use std::fs;

use graph::FsGraph;
// use pyret_error::miette;
use pyret_interpreter::{trove, Interpreter, PyretGraph};
// use pyret_error::term::{
//     termcolor::{ColorChoice, StandardStream},
//     Config,
// };

fn main()
// -> miette::Result<()>
{
    // miette::set_hook(Box::new(|_| {}))?;

    let mut graph = FsGraph::new();

    let name = fs::canonicalize("test.arr").unwrap();

    let file_id = graph.register(&name.to_string_lossy());

    let mut interpreter = Interpreter::new(graph);

    interpreter
        .context
        .borrow_mut()
        .io
        .read_out(Box::new(|value| {
            println!("{value}");
        }));

    interpreter.use_context::<trove::Global>();

    match interpreter.interpret(file_id) {
        Ok(values) => {
            for value in values.iter() {
                println!("{value}");
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
