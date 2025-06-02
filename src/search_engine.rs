use crate::intersection::galloping_search;
use std::collections::{BTreeMap, HashMap};

pub trait MapTrait<K, V> {
    fn new() -> Self;
    fn get(&self, key: &K) -> Option<&V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    fn insert(&mut self, key: K, value: V);
}

impl<K, V> MapTrait<K, V> for HashMap<K, V>
where
    K: std::hash::Hash + Eq,
{
    fn new() -> Self {
        HashMap::new()
    }

    fn get(&self, key: &K) -> Option<&V> {
        HashMap::get(self, key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        HashMap::get_mut(self, key)
    }

    fn insert(&mut self, key: K, value: V) {
        HashMap::insert(self, key, value);
    }
}

impl<K, V> MapTrait<K, V> for BTreeMap<K, V>
where
    K: Ord,
{
    fn new() -> Self {
        BTreeMap::new()
    }

    fn get(&self, key: &K) -> Option<&V> {
        BTreeMap::get(self, key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        BTreeMap::get_mut(self, key)
    }

    fn insert(&mut self, key: K, value: V) {
        BTreeMap::insert(self, key, value);
    }
}

pub struct InvertedIndecies<M>
where
    M: MapTrait<String, Vec<usize>>,
{
    pub indecies: M,
    pub movies: Vec<String>,
}

fn prepare_word(word: &str) -> String {
    word.to_lowercase()
        .replace(|c: char| !c.is_alphanumeric(), "")
}

impl<M> InvertedIndecies<M>
where
    M: MapTrait<String, Vec<usize>>,
{
    pub fn new(file_path: &str) -> Self {
        let mut indecies = M::new();
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
                        match indecies.get_mut(&word) {
                            None => {
                                indecies.insert(word.clone(), vec![movies.len()]);
                            }
                            Some(indices) => {
                                if indices.is_empty() || *indices.last().unwrap() != movies.len() {
                                    indices.push(movies.len());
                                }
                            }
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

pub struct NaiveSearchEngine {
    movies: Vec<(String, String)>,
}

impl NaiveSearchEngine {
    pub fn new(file_path: &str) -> Self {
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

                let description = buffer[..].trim().to_string();

                movies.push((title, description));
                buffer.clear();
            }
        }

        Self { movies }
    }

    pub fn query(&self, words: Vec<&str>) -> Vec<String> {
        let mut result = Vec::new();

        'movie: for (title, description) in &self.movies {
            'query_word: for query_word in &words {
                let query_word = prepare_word(query_word);
                for word in description.split_whitespace() {
                    if prepare_word(word) == query_word {
                        continue 'query_word;
                    }
                }

                continue 'movie;
            }

            result.push(title.clone());
        }

        result
    }
}
