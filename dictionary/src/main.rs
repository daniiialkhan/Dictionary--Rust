
mod dictionary;
use dictionary::dictionary::*;
mod hashionary;
use hashionary::hashionary::*;

fn main() {
	let mut dict = Dictionary::new();
	dict.add_word("hello", "a greeting");
	dict.add_word("world", "the planet we live on");
	dict.add_word("hell", "a place of eternal doom");
	dict.add_word("helicopter", "a flying machine");
	dict.add_word("hello", "a greeting");
	dict.list_words();
	println!("Meaning of hello: {}", dict.get_meaning("hello").unwrap());
	println!("Meaning of world: {}", dict.get_meaning("world").unwrap());
	println!("Meaning of foo: {:?}", dict.get_meaning("foo"));

	let input = String::from("hel");
	println!("Words starting with {}: ", input);
	let rec = dict.recommendations(&input);
	for w in rec {
		println!("{}", w);
	}


	println!("---------------------------------");
	println!("Hashionary");
	println!("---------------------------------");

	let mut hash = Hashionary::new();
	hash.add_word("hello", "a greeting");
	hash.add_word("world", "the planet we live on");
	hash.add_word("hello", "a greeting");
	hash.add_word("hell", "a place of eternal doom");
	hash.add_word("helicopter", "a flying machine");
	
	println!("Meaning of hello: {}", hash.get_meaning_str("hello"));
	println!("Meaning of world: {}", hash.get_meaning_str("world"));
	println!("Meaning of foo: {:?}", hash.get_meaning("foo"));
	
	println!("Meaning of hello: {}", hash.get_meaning("hello").unwrap_or("Word not found in hashionary"));
	println!("Meaning of world: {}", hash.get_meaning("world").unwrap());
	println!("Meaning of foo: {:?}", hash.get_meaning("foo"));

	hash.list_words();

	let input = String::from("hel");
	println!("Words starting with {}: ", input);
	let rec = hash.recommendations(&input);
	for w in rec {
		println!("{}", w);
	}
	
	
}
