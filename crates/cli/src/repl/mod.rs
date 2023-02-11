mod helper;
use crate::{dir, graph::FsGraph};
use ansi_term::{Color, Style};
use helper::MyHelper;
use pyret_error::term::{
    termcolor::{ColorChoice, StandardStream},
    Config,
};
use rustyline::{config::Configurer, error::ReadlineError, Editor};

pub fn start() {
    let dimmed = Color::White.dimmed();

    let exit_hint = dimmed.paint("Exit using Ctrl+D");

    println!(
        "{} v{}\n{exit_hint}",
        Style::new().bold().paint("Pyret Rust"),
        env!("CARGO_PKG_VERSION")
    );

    let mut editor = Editor::new().unwrap();

    editor.set_helper(Some(MyHelper::new()));

    let log = &dir::join("history.log");

    editor.set_auto_add_history(true);
    if log.exists() {
        editor.load_history(log).unwrap();
    }

    let prompt = &format!("{} ", dimmed.paint(">>>"));

    let mut graph = FsGraph::new();

    // let mut engine = PyretEngine::new();

    // thread::spawn(|| loop {
    //     let mut values = STDIO.lock().expect("Could not lock STDIO");
    //     let values = values.drain(..);

    //     for value in values {
    //         println!("{value}");
    //     }
    // });

    let mut i = 1_usize;

    let _writer = StandardStream::stderr(ColorChoice::Auto);
    let _config = Config::default();

    loop {
        match editor.readline(prompt) {
            Ok(source) => {
                let _file_id = graph.add(
                    format!("repl://{i}").into_boxed_str(),
                    Box::from(source.as_str()),
                );

                // if let Err(errors) = engine.compile(&mut graph, file_id, &source) {
                //     for error in errors {
                //         error.emit(&graph, &writer, &config);
                //     }
                // }
            }
            Err(ReadlineError::Interrupted) => eprintln!("{exit_hint}"),
            _ => break,
        }

        i += 1;
    }

    editor.save_history(log).unwrap();
}
