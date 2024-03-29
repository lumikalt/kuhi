use std::collections::HashMap;

pub struct Formatter {
    src: String,
    symbols: HashMap<&'static str, &'static str>,
}

impl Formatter {
    pub fn new(src: String) -> Self {
        let mut symbols = [
            ("_", "‿"),
            ("infinity", "∞"),
            ("epsilon", "ε"),
            ("pi", "π"),
            ("tau", "τ"),
            ("alpha", "α"),
            ("iota", "ι"),
            (":", "↕"),
            ("flip", "↕"),
            ("swap", "↔"),
            ("`", "⁻"),
            ("*", "×"),
            ("%", "÷"),
            ("pow", "ⁿ"),
            ("log", "ₙ"),
            ("croot", "∛"),
            ("cbrt", "∛"),
            ("sqrt", "√"),
            ("root", "√"),
            ("sin", "◯"),
            ("sinh", "ⓔ"),
            ("sins", "Ⓞ"),
            ("inverse", "⁻¹"),
            // ("exp", "exp"),
            // ("abs", "abs"),
            // ("floor", "floor"),
            // ("ceil", "ceil"),
            // ("round", "round"),
            // ("trunc", "trunc"),
            // ("sign", "sign"),
            // ("gamma", "Γ"),
            // ("digamma", "ψ"),
            // ("beta", "β"),
            // ("zeta", "ζ"),
            // ("erf", "erf"),
            // ("erfc", "erfc"),
            // ("erfcinv", "erfcinv"),
            // ("erfinv", "erfinv"),
            // ("gamma_inc", "Γ"),
            // ("gamma_inc_inv", "Γ⁻¹"),
            // ("beta_inc", "β"),
            // ("beta_inc_inv", "β⁻¹"),
            // ("zeta", "ζ"),
            // ("zeta_inv", "ζ⁻¹"),
            // ("polygamma", "ψ"),
            // ("polygamma_inv", "ψ⁻¹"),
            // ("digamma", "ψ"),
            // ("digamma_inv", "ψ⁻¹"),
            ("factorial", "!"),
        ];
        symbols.sort_by(|fst, snd| match fst.0.len() {
            x if x > snd.0.len() => std::cmp::Ordering::Less,
            x if x < snd.0.len() => std::cmp::Ordering::Greater,
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
