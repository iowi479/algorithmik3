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
    M: MapTrait<String, Vec<MovieIndex>>,
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
    M: MapTrait<String, Vec<MovieIndex>>,
{
    pub fn new(file_path: &str) -> Self {
        let mut indices = M::new();
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

                for word in title.split_whitespace() {
                    let word = prepare_word(word);
                    if !word.is_empty() {
                        let mut index = MovieIndex::new(movies.len());
                        index.seen_in_title();

                        match indices.get_mut(&word) {
                            None => {
                                indices.insert(word.clone(), vec![index]);
                            }
                            Some(movie_indices) => {
                                if movie_indices.is_empty()
                                    || movie_indices.last().unwrap().movie_index != movies.len()
                                {
                                    movie_indices.push(index);
                                } else {
                                    movie_indices.last_mut().unwrap().seen_in_title();
                                }
                            }
                        }
                    }
                }

                for word in buffer[title_end..].split_whitespace() {
                    let word = prepare_word(word);
                    if !word.is_empty() {
                        let mut index = MovieIndex::new(movies.len());
                        index.seen_in_description();

                        match indices.get_mut(&word) {
                            None => {
                                indices.insert(word.clone(), vec![index]);
                            }
                            Some(movie_indices) => {
                                if movie_indices.is_empty()
                                    || movie_indices.last().unwrap().movie_index != movies.len()
                                {
                                    movie_indices.push(index);
                                } else {
                                    movie_indices.last_mut().unwrap().seen_in_description();
                                }
                            }
                        }
                    }
                }

                movies.push(title);
                buffer.clear();
            }
        }

        Self {
            indecies: indices,
            movies,
        }
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

            result = galloping_search_movie(&mut b, &mut result);
        }

        result.sort_unstable_by_key(|movie_index| movie_index.rank());

        result
            .iter()
            .rev()
            .map(|i| self.movies[i.movie_index].clone() + i.to_string().as_str())
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

#[derive(Debug, Clone)]
pub struct MovieIndex {
    pub movie_index: usize,
    pub rank_title: u8,
    pub rank_desc: u8,
}

impl MovieIndex {
    pub fn new(movie_index: usize) -> Self {
        Self {
            movie_index,
            rank_title: 0,
            rank_desc: 0,
        }
    }

    pub fn seen_in_description(&mut self) {
        if self.rank_desc < 255 {
            self.rank_desc += 1;
        }
    }

    pub fn seen_in_title(&mut self) {
        if self.rank_title < 255 {
            self.rank_title += 1;
        }
    }

    pub fn rank(&self) -> u8 {
        self.rank_desc.saturating_add(self.rank_title * 10)
    }

    pub fn combine(a: &MovieIndex, b: &MovieIndex) -> Self {
        let mut combined = a.clone();
        assert_eq!(combined.movie_index, b.movie_index);
        combined.rank_title = combined.rank_title.saturating_add(b.rank_title);
        combined.rank_desc = combined.rank_desc.saturating_add(b.rank_desc);
        combined
    }
}

impl std::fmt::Display for MovieIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\t\t{{ movie_index: {}, rank_title: {}, rank_desc: {} }}",
            self.movie_index, self.rank_title, self.rank_desc
        )
    }
}

impl Eq for MovieIndex {}

impl PartialEq for MovieIndex {
    fn eq(&self, other: &Self) -> bool {
        self.movie_index == other.movie_index
    }
}

impl Ord for MovieIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.movie_index.cmp(&other.movie_index)
    }
}

impl PartialOrd for MovieIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn galloping_search_movie(a: &[MovieIndex], b: &[MovieIndex]) -> Vec<MovieIndex> {
    let mut result = Vec::new();

    let mut pointer_a = 0;
    for item in b {
        // Galloping search
        let mut step = 1;
        while pointer_a + step < a.len() && a[pointer_a + step] < *item {
            step *= 2;
        }
        let start = pointer_a + step / 2;
        let end = std::cmp::min(pointer_a + step + 1, a.len());

        match a[start..end].binary_search(&item) {
            Ok(ai) => {
                let other = &a[start + ai];
                let index = MovieIndex::combine(item, other);
                result.push(index);
                pointer_a = start; // Move pointer_a to the end of the found range
            }
            Err(_) => {
                // If not found, continue searching in the next segment
                pointer_a = start;
            }
        }
    }

    result
}
