use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use kuhi::{parser::parse, vm::Env};
use rustyline::DefaultEditor;

fn main() -> anyhow::Result<()> {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();
    let _editor = DefaultEditor::new();

    let input =
        "+ 1.2 2 1";
    let input = dbg!(input);

    let tokens = match parse(input) {
        Ok(tokens) => {
            dbg!(tokens)
        }
        Err(err) => {
            let file = SimpleFile::new("<repl>", input);
            let start = err.1.offset;

            let diagnostic = Diagnostic::error()
                .with_message("Syntax error")
                .with_labels(vec![
                    Label::primary((), start..start).with_message(err.0.to_string())
                ])
                .with_notes(vec![err.0.note()]);

            term::emit(&mut writer.lock(), &config, &file, &diagnostic)?;

            return Ok(());
        }
    };

    let mut env = Env::new(&tokens);
    match env.run() {
        Ok(_) => println!(">> {:?}", env.stack),
        Err(err) => {
            let file = SimpleFile::new("<repl>", input);
            let start = err.1.offset;

            let diagnostic = Diagnostic::error()
                .with_message("Runtime error")
                .with_labels(vec![
                    Label::primary((), start..start).with_message(err.0.to_string())
                ])
                .with_notes(vec![err.0.note()]);

            term::emit(&mut writer.lock(), &config, &file, &diagnostic)?;
        }
    }

    Ok(())
}
