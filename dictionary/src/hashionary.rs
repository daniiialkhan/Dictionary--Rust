pub mod hashionary {
    use std::collections::HashMap;

	pub struct Hashionary {
		pub words: HashMap<String, String>,
	}

	impl Hashionary {

		// Constructor => Make a new hashionary
		pub fn new() -> Hashionary {
			Hashionary {
				words: HashMap::new(),
			}
		}

		// Add a word to the hashionary
		pub fn add_word(&mut self, word: &str, meaning: &str) {
			// Check if word exists already
			if self.word_exists(word) {
				println!("Word already exists in hashionary");
				return;
			}
			// add it to the hashionary
			self.words.insert(word.to_string(), meaning.to_string());
		}

		// Check if word exists in hashionary
		pub fn word_exists(&self, word: &str) -> bool {
			self.words.contains_key(word)
		}

		// Get the meaning of a word
		pub fn get_meaning(&self, word: &str) -> Option<&str> {
			match self.words.get(word) {
				Some(meaning) => Some(meaning),
				None => {
					println!("Word not found in hashionary");
					None
				}
			}
		}

		// Get the meaning of a word in string
		pub fn get_meaning_str(&self, word: &str) -> &str {
			match self.words.get(word) {
				Some(meaning) => meaning,
				None => {
					println!("Word not found in hashionary");
					""
				}
			}
		}

		// List all words in hashionary
		pub fn list_words(&self) {
			println!("---------------------------------");
			println!("Listing all words in hashionary");
			println!("Word  =>  Meaning");
			println!("---------------------------------");
			for (word, meaning) in &self.words {
				println!("{} => {}", word, meaning);
			}
			println!("---------------------------------");
		}

		// Get recommendations for a word
		pub fn recommendations(&self, pattern: &str) -> Vec<String> {
			let mut recommendations: Vec<String> = Vec::new();
			for w in self.words.keys() {
				if w.contains(pattern) {
					recommendations.push(w.clone());
				}
			}
			recommendations
		}

	}
}