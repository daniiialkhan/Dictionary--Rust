
pub mod dictionary {

	pub struct Word {
		pub word: String,
		pub meaning: String,
	}
	pub struct Dictionary {
		pub words : Vec<Word>,
	}

	impl Dictionary {

		// Constructor => Make a new dictionary
		pub fn new() -> Dictionary {
			Dictionary {
				words: Vec::new(),
			}
		}

		// Add a word to the dictionary
		pub fn add_word(&mut self, word: &str, meaning: &str) {
			// Check if word exists already
			if self.word_exists(word) {
				println!("Word already exists in dictionary");
				return;
			} 
			// make a new Word object
			let new_word = Word {
				word: word.to_string(),
				meaning: meaning.to_string(),
			};
			// add it to the dictionary
			self.words.push(new_word);
		}

		pub fn word_exists(&self, word:&str) -> bool {
			for w in &self.words {
				if w.word == word {
					return true;
				}
			}
			false
		}

		pub fn get_meaning(&self, word: &str) -> Option<&str> {
			for w in &self.words {
				if w.word == word {
					return Some(&w.meaning);
				}
			}
			println!("Word not found in dictionary");
			None
		}

		pub fn list_words(&self) {
			println!("---------------------------------");
			println!("Listing all words in dictionary");
			println!("Word  =>  Meaning");
			println!("---------------------------------");
			for w in &self.words {
				println!("{} => {}", w.word, w.meaning)
			}
			println!("---------------------------------");
		}

		// input parts of a word and match it with parts of existing words

		pub fn recommendations(&self, pattern: &str) -> Vec<String> {
			let mut recommendations: Vec<String> = Vec::new();
			for w in &self.words {
				if w.word.contains(pattern) {
					recommendations.push(w.word.clone());
				}
			}
			recommendations
		}
		






	}

}