use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use kuhi::{
    formatter::Formatter,
    parser::{parse, Loc},
    vm::Env,
};
use rustyline::DefaultEditor;

fn main() -> anyhow::Result<()> {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();
    let mut editor = DefaultEditor::new()?;
    let _ = editor.load_history("history.txt");

    let mut env = Env::new(vec![]);
    let mut loc = Loc {
        start: 0,
        end: 0,
        line: 1,
        column: 1,
    };

    let mut full_input = String::new();

    // REPL
    loop {
        let mut input = editor.readline(&format!("{}> ", loc.line))?;

        if input == ":q" {
            editor.save_history("history.txt")?;
            break;
        }

        input = Formatter::new(input).format();

        editor.add_history_entry(input.clone())?;

        input.push('\n');
        full_input.push_str(&input.clone());

        let file = SimpleFile::new("<repl>", full_input.clone());

        let tokens = match parse(&input, &mut loc) {
            Ok(tokens) => {
                // dbg!(tokens)
                tokens
            }
            Err(err) => {
                dbg!(loc.clone());
                dbg!(full_input);
                let start = err.1.start;
                let end = err.1.end + 1;

                let diagnostic = Diagnostic::error()
                    .with_message("Syntax error")
                    .with_labels(vec![
                        Label::primary((), start..end).with_message(err.0.to_string())
                    ])
                    .with_notes(vec![err.0.note()]);

                term::emit(&mut writer.lock(), &config, &file, &diagnostic)?;

                break;
            }
        };

        match env.repurpose(&tokens).run() {
            Ok(_) => println!("{env}"),
            Err(err) => {
                let start = err.1.start;
                let end = err.1.end + 1;

                let diagnostic = Diagnostic::error()
                    .with_message("Runtime error")
                    .with_labels(vec![
                        Label::primary((), start..end).with_message(err.0.to_string())
                    ])
                    .with_notes(vec![err.0.note()]);

                term::emit(&mut writer.lock(), &config, &file, &diagnostic)?;

                break;
            }
        }
    }

    editor.save_history("history.txt")?;

    Ok(())
}
