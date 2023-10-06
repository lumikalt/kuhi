use std::collections::HashMap;

pub struct Formatter {
    src: String,
    symbols: HashMap<&'static str, &'static str>,
}

impl Formatter {
    pub fn new(src: String) -> Self {
        let mut symbols = [
            ("infinity", "∞"),
            ("epsilon", "ε"),
            ("pi", "π"),
            ("tau", "τ"),
            ("alpha", "α"),
            ("iota", "ι"),

            (":", "↔"),
            ("`", "⁻"),

            ("*", "×"),
            ("%", "÷"),
            ("^", "ⁿ"),
            ("croot", "∛"),
            ("cbrt", "∛"),
            ("sqrt", "√"),
            ("root", "√"),
            ("_", "‿"),
        ];
        symbols.sort_by(|fst, snd| match fst.0.len() {
            x if x > snd.0.len() => std::cmp::Ordering::Greater,
            x if x < snd.0.len() => std::cmp::Ordering::Less,
            _ => std::cmp::Ordering::Equal,
        });

        Self {
            src,
            // Largest to smallest
            symbols: HashMap::from(symbols),
        }
    }

    pub fn format(&mut self) -> String {
        let mut src = self.src.clone();
        let symbols = self.symbols.clone();

        for (from, to) in symbols.into_iter() {
            src = src.replace(from, to);
        }

        src.clone()
    }
}
