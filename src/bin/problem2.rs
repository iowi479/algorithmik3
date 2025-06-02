use std::collections::{BTreeMap, HashMap};

use algorithmik3::search_engine::{InvertedIndecies, NaiveSearchEngine};

type IndexHashMap = HashMap<String, Vec<usize>>;
type IndexBTreeMap = BTreeMap<String, Vec<usize>>;

fn main() {
    println!("HashMap Implementation:");
    let timeout = std::time::Instant::now();
    let indecies: InvertedIndecies<IndexHashMap> = InvertedIndecies::new("movies.txt");
    let elapsed = timeout.elapsed();
    println!("Indexing took: {:?}", elapsed);

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <query words>", args[0]);
        return;
    }

    let query_words: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();

    let timeout = std::time::Instant::now();
    let result = indecies.query(query_words);
    let elapsed = timeout.elapsed();
    println!("Querying took: {:?} for {} movies", elapsed, result.len());

    println!("\nBTreeMap Implementation:");
    let timeout = std::time::Instant::now();
    let indecies: InvertedIndecies<IndexBTreeMap> = InvertedIndecies::new("movies.txt");
    let elapsed = timeout.elapsed();
    println!("Indexing took: {:?}", elapsed);

    let query_words: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();

    let timeout = std::time::Instant::now();
    let result = indecies.query(query_words);
    let elapsed = timeout.elapsed();
    println!("Querying took: {:?} for {} movies", elapsed, result.len());

    println!("\nNaive Implementation:");
    let timeout = std::time::Instant::now();
    let naive_search = NaiveSearchEngine::new("movies.txt");
    let elapsed = timeout.elapsed();
    println!("Parsing took: {:?}", elapsed);

    let query_words: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();

    let timeout = std::time::Instant::now();
    let result = naive_search.query(query_words);
    let elapsed = timeout.elapsed();
    println!("Querying took: {:?} for {} movies", elapsed, result.len());
}
