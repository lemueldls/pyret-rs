use crate::{dir, runtime::Runtime};

use ansi_term::{Color, Style};
use rustyline::{config::Configurer, Editor};

use pyret_transpiler::Transpiler;

pub fn start() {
    let grey = Color::White.dimmed();

    println!(
        "{} v{}\n{}",
        Style::new().bold().paint("Pyret Rust"),
        env!("CARGO_PKG_VERSION"),
        grey.paint("Exit using Ctrl+D")
    );

    let mut runtime = Runtime::new();

    let mut editor = Editor::<()>::new();

    let log = &dir::join("history.log");

    editor.set_auto_add_history(true);
    if log.exists() {
        editor.load_history(log).unwrap();
    }

    let prompt = &format!("{} ", grey.paint(">>>"));

    let location = "repl://";

    let mut transpiler = Transpiler::default();

    while let Ok(line) = editor.readline(prompt) {
        match transpiler.transpile(location.to_owned(), line) {
            Ok(code) => match runtime.execute_script(location, &code) {
                Ok(_output) => {}
                Err(error) => eprintln!("{}", Color::Red.paint(error.to_string())),
            },
            Err(error) => eprintln!("{error}"),
        };
    }

    editor.save_history(log).unwrap();
}
