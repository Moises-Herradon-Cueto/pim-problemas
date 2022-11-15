use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Enunciado {
    pub raw: String,
    pub html: String,
}

impl Enunciado {
    #[must_use]
    pub fn new(raw: String) -> Self {
        let html = katex::render(&raw).unwrap_or_else(|_err| {
            // println!("Error parsing latex: {err:?}");
            String::new()
        });
        Self { raw, html }
    }

    #[must_use]
    pub const fn empty() -> Self {
        Self {
            raw: String::new(),
            html: String::new(),
        }
    }
}

impl Display for Enunciado {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.raw)
    }
}
