use criterion::{criterion_group, criterion_main, Criterion};
use pyret_lexer::lex;
use std::fs;

pub fn criterion_benchmark(c: &mut Criterion) {
    let filename = fs::canonicalize("what.arr").unwrap();

    let input = &fs::read_to_string(filename).unwrap();

    c.bench_function("lex", |b| b.iter(|| lex(input)));

    // match lexer.lex() {
    //     Ok(_tokens) => {
    //         // dbg!(tokens);
    //     }
    //     Err(errors) => {
    //         // dbg!(&errors);

    //         let file = SimpleFile::new(name, lexer.input);

    //         let writer = StandardStream::stderr(ColorChoice::Auto);
    //         let config = Config::default();

    //         for error in errors {
    //             error.emit(&file, &writer, &config);
    //         }
    //     }
    // }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
