#![cfg(feature = "lsp")]

use serde::Serialize;
use serde_json::json;
use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
	ops::Range,
};

use crate::scanner::TokenPosition;

#[derive(Serialize)]
pub enum SymbolKind {
	VARIABLE,
	FUNCTION,
	PSEUDO,
	ENUM,
	CONSTANT,
	MACRO,
	ARGUMENT
}

#[derive(Serialize)]
pub enum SymbolModifier {
	LOCAL, GLOBAL, STATIC
}

fn hash_string(string: &str) -> u64 {
	let mut hasher = DefaultHasher::new();
	string.hash(&mut hasher);
	hasher.finish()
}

pub fn send_symbol(
	token: &str,
	value: String,
	location: Range<TokenPosition>,
	kind: SymbolKind,
	modifiers: &[SymbolModifier],
) {
	println!(
		"DEFINITION {}",
		json!({
			"id": hash_string(token),
			"token": token,
			"value": value,
			"location": {
				"start": {
					"line": location.start.line,
					"column": location.start.column,
				},
				"end": {
					"line": location.end.line,
					"column": location.end.column,
				}
			},
			"kind": kind,
			"modifiers": modifiers
		})
	)
}

#[cfg(test)]
mod tests {
    use crate::lsp::hash_string;

	#[test]
	fn check_hash() {
		assert!(
			hash_string("test_string") == hash_string("test_string"),
			"hasing the same string twice gave different results!"
		);
		assert!(
			hash_string("test_string") != hash_string("test_strinh"),
			"somehow hashing 2 different strings gave the same result!"
		);
	}
}