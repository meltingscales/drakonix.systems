use rand::Rng;
use std::collections::HashMap;

const SLUG_WORDS: &[&str] = &[
    "admin", "login", "api", "config", "backup", "data", "user", "auth",
    "secret", "private", "internal", "test", "debug", "staging", "prod",
    "system", "database", "cache", "redis", "mysql", "postgres", "mongo",
    "dashboard", "panel", "console", "manage", "settings", "control",
];

/// Generate a realistic-looking honeypot slug to attract bots
pub fn generate_honeypot_slug() -> String {
    let mut rng = rand::thread_rng();
    let word1 = SLUG_WORDS[rng.gen_range(0..SLUG_WORDS.len())];
    let word2 = SLUG_WORDS[rng.gen_range(0..SLUG_WORDS.len())];
    let num = rng.gen_range(100..999);
    format!("{}-{}-{}", word1, word2, num)
}

/// Generate multiple unique honeypot URLs
pub fn generate_honeypot_urls(count: usize) -> Vec<String> {
    (0..count)
        .map(|_| format!("/api/markov-babble/{}/gen", generate_honeypot_slug()))
        .collect()
}

/// Simple Markov chain text generator for creating honeypot content
#[derive(Clone)]
pub struct MarkovGenerator {
    chain: HashMap<String, Vec<String>>,
}

impl MarkovGenerator {
    pub fn new() -> Self {
        let mut chain = HashMap::new();

        // Load word list from embedded file
        let wordlist = include_str!("../static/wordlist.txt");
        let words: Vec<&str> = wordlist.lines().collect();

        // Build a more sophisticated markov chain using the dictionary
        // Create random phrases by connecting dictionary words
        let mut rng = rand::thread_rng();

        // Generate 1000 random "sentences" from the dictionary to build the chain
        for _ in 0..1000 {
            let sentence_len = rng.gen_range(5..15);
            let mut sentence_words = Vec::new();

            for _ in 0..sentence_len {
                if let Some(&word) = words.get(rng.gen_range(0..words.len())) {
                    sentence_words.push(word);
                }
            }

            // Build bigram chains from this random sentence
            for i in 0..sentence_words.len().saturating_sub(1) {
                let key = sentence_words[i].to_lowercase();
                let next = sentence_words[i + 1].to_string();
                chain.entry(key).or_insert_with(Vec::new).push(next);
            }
        }

        // Add some common technical jargon to make it look more "real"
        let tech_corpus = vec![
            "API endpoint database query optimization",
            "cloud infrastructure deployment pipeline",
            "machine learning neural network algorithm",
            "security authentication encryption protocol",
            "microservices containerization orchestration",
        ];

        for text in tech_corpus {
            let phrase_words: Vec<&str> = text.split_whitespace().collect();
            for i in 0..phrase_words.len().saturating_sub(1) {
                let key = phrase_words[i].to_lowercase();
                let next = phrase_words[i + 1].to_string();
                chain.entry(key).or_insert_with(Vec::new).push(next);
            }
        }

        MarkovGenerator { chain }
    }

    /// Generate a stream of markov babble text with occasional API links to waste threads
    pub fn generate(&self, seed: &str, word_count: usize) -> String {
        let mut rng = rand::thread_rng();
        let mut result = Vec::new();

        // Start with a seed word or random word from chain
        let mut current = if !seed.is_empty() && self.chain.contains_key(&seed.to_lowercase()) {
            seed.to_lowercase()
        } else {
            let keys: Vec<_> = self.chain.keys().collect();
            if keys.is_empty() {
                return "No corpus available".to_string();
            }
            keys[rng.gen_range(0..keys.len())].clone()
        };

        result.push(current.clone());

        for _ in 0..word_count {
            // 1/500 chance to insert a fake API endpoint link instead of a word
            // This wastes scraper threads by making them follow more honeypot links
            if rng.gen_bool(1.0 / 500.0) {
                let api_slug = generate_honeypot_slug();
                let api_link = format!("/api/markov-babble/{}/gen", api_slug);
                result.push(api_link);

                // Reset to a random word after the link
                let keys: Vec<_> = self.chain.keys().collect();
                current = keys[rng.gen_range(0..keys.len())].clone();
                continue;
            }

            if let Some(nexts) = self.chain.get(&current) {
                if nexts.is_empty() {
                    // Dead end, pick random word
                    let keys: Vec<_> = self.chain.keys().collect();
                    current = keys[rng.gen_range(0..keys.len())].clone();
                } else {
                    let next = &nexts[rng.gen_range(0..nexts.len())];
                    result.push(next.clone());
                    current = next.to_lowercase();
                }
            } else {
                // Word not in chain, pick random
                let keys: Vec<_> = self.chain.keys().collect();
                current = keys[rng.gen_range(0..keys.len())].clone();
                result.push(current.clone());
            }

            // Occasionally add punctuation for realism
            if rng.gen_bool(0.1) {
                let punct = [". ", ", ", "; ", "! ", "? "];
                let p = punct[rng.gen_range(0..punct.len())];
                if let Some(last) = result.last_mut() {
                    last.push_str(p.trim());
                }
            }
        }

        result.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markov_generation() {
        let gen = MarkovGenerator::new();
        let text = gen.generate("the", 50);
        assert!(!text.is_empty());
        assert!(text.len() > 50);
    }
}
