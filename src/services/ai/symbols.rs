use std::collections::HashSet;

#[derive(Clone, Default)]
pub struct SymbolExtractor {
    common_symbols: HashSet<String>,
}

impl SymbolExtractor {
    pub fn new() -> Self {
        let mut common_symbols = HashSet::new();
        for symbol in &[
            "water", "fire", "flying", "falling", "teeth", "death", "snake", "chase", "house",
            "animal", "school", "baby", "car", "moon", "sun", "star", "ocean", "mountain", "door",
            "key", "bridge", "path", "shadow", "light", "mother", "father", "child", "god",
            "devil", "money", "exam", "naked", "wedding", "funeral", "birthday",
        ] {
            common_symbols.insert(symbol.to_string());
        }

        SymbolExtractor { common_symbols }
    }

    pub fn with_symbol(mut self, symbol: &str) -> Self {
        self.common_symbols.insert(symbol.to_lowercase());
        self
    }

    pub fn with_symbols(mut self, symbols: Vec<&str>) -> Self {
        for symbol in symbols {
            self.common_symbols.insert(symbol.to_lowercase());
        }
        self
    }

    pub fn extract_symbols(&self, text: &str) -> Vec<String> {
        let text = text.to_lowercase();

        self.common_symbols
            .iter()
            .filter(|symbol| text.contains(symbol.as_str()))
            .cloned()
            .collect()
    }
}
