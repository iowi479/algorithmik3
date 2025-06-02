use crate::intersection::galloping_search;
use std::collections::HashMap;

pub struct InvertedIndeciesHash {
    pub indecies: HashMap<String, Vec<usize>>,
    pub movies: Vec<String>,
}

fn prepare_word(word: &str) -> String {
    word.to_lowercase()
        .replace(|c: char| !c.is_alphanumeric(), "")
}

impl InvertedIndeciesHash {
    pub fn new(file_path: &str) -> Self {
        let mut indecies = HashMap::new();
        let mut movies = Vec::new();

        let mut buffer = String::with_capacity(8 * 1024);

        if let Ok(file) = std::fs::File::open(file_path) {
            use std::io::{BufRead, BufReader};
            let mut reader = BufReader::new(file);

            while let Ok(bytes_read) = reader.read_line(&mut buffer) {
                if bytes_read == 0 {
                    break; // EOF
                }

                let title_end = buffer.find('\t').unwrap();
                let title = buffer[..title_end].to_string();

                let words = buffer.split_whitespace();

                for word in words {
                    let word = prepare_word(word);
                    if !word.is_empty() {
                        let i = indecies.entry(word).or_insert_with(Vec::new);
                        if i.is_empty() || *i.last().unwrap() != movies.len() {
                            i.push(movies.len());
                        }
                    }
                }

                movies.push(title);
                buffer.clear();
            }
        }

        Self { indecies, movies }
    }

    pub fn query(&self, words: Vec<&str>) -> Vec<String> {
        let mut result = Vec::new();
        let mut b = Vec::new();

        if let Some(first_word) = words.first() {
            let first_word = prepare_word(first_word);
            if let Some(indices) = self.indecies.get(&first_word) {
                result.extend_from_slice(indices);
            }
        }

        for word in &words[1..] {
            b.clear();

            let word = prepare_word(word);
            if let Some(indices) = self.indecies.get(&word) {
                b.extend_from_slice(indices);
            }

            result = galloping_search(&mut b, &mut result);
        }

        result
            .iter()
            .map(|&i| self.movies[i].clone())
            .collect::<Vec<String>>()
    }
}
