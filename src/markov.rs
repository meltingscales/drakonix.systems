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

        // Seed corpus - mix of technical jargon, Lorem ipsum, and random words
        let corpus = vec![
            "The quick brown fox jumps over the lazy dog",
            "Lorem ipsum dolor sit amet consectetur adipiscing elit",
            "Artificial intelligence machine learning neural networks deep learning",
            "Blockchain cryptocurrency decentralized distributed ledger technology",
            "Cloud computing microservices containerization orchestration",
            "Quantum computing superposition entanglement qubits algorithms",
            "Software engineering design patterns architecture refactoring",
            "Database normalization indexing optimization query performance",
            "Security encryption authentication authorization cryptography",
            "Network protocol TCP IP HTTP REST API endpoint",
            "Frontend backend fullstack development deployment pipeline",
            "Agile scrum kanban sprint retrospective standup planning",
            "Data science analytics visualization insights predictive modeling",
            "Mobile responsive progressive web application native hybrid",
            "Version control git repository commit branch merge pull request",
            "Testing unit integration end-to-end continuous integration delivery",
            "Infrastructure as code terraform ansible kubernetes docker",
            "Monitoring logging metrics observability tracing debugging",
            "Performance optimization caching scalability load balancing",
            "User experience interface design accessibility usability",
        ];

        // Build bigram markov chain
        for text in corpus {
            let words: Vec<&str> = text.split_whitespace().collect();
            for i in 0..words.len().saturating_sub(1) {
                let key = words[i].to_lowercase();
                let next = words[i + 1].to_string();
                chain.entry(key).or_insert_with(Vec::new).push(next);
            }
        }

        MarkovGenerator { chain }
    }

    /// Generate a stream of markov babble text
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
