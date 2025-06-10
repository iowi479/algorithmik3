use algorithmik3::search_engine::{InvertedIndecies, MovieIndex, NaiveSearchEngine};
use std::collections::{BTreeMap, HashMap};

type IndexHashMap = HashMap<String, Vec<MovieIndex>>;
type IndexBTreeMap = BTreeMap<String, Vec<MovieIndex>>;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <show result count> <query words>", args[0]);
        return;
    }

    let query_words: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();
    let num_results: usize = args[1].parse().unwrap_or(10);

    println!("HashMap Implementation:");
    let timeout = std::time::Instant::now();
    let indecies: InvertedIndecies<IndexHashMap> = InvertedIndecies::new("movies.txt");
    let elapsed = timeout.elapsed();
    println!("Indexing took: {:?}", elapsed);
    let timeout = std::time::Instant::now();
    let result = indecies.query(query_words);
    let elapsed = timeout.elapsed();
    println!("Querying took: {:?} for {} movies", elapsed, result.len());

    let query_words: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();

    println!("\nBTreeMap Implementation:");
    let timeout = std::time::Instant::now();
    let indecies: InvertedIndecies<IndexBTreeMap> = InvertedIndecies::new("movies.txt");
    let elapsed = timeout.elapsed();
    println!("Indexing took: {:?}", elapsed);
    let timeout = std::time::Instant::now();
    let result = indecies.query(query_words);
    let elapsed = timeout.elapsed();
    println!("Querying took: {:?} for {} movies", elapsed, result.len());

    let query_words: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();

    println!("\nNaive Implementation:");
    let timeout = std::time::Instant::now();
    let naive_search = NaiveSearchEngine::new("movies.txt");
    let elapsed = timeout.elapsed();
    println!("Parsing took: {:?}", elapsed);
    let timeout = std::time::Instant::now();
    let _naive_result = naive_search.query(query_words);
    let elapsed = timeout.elapsed();
    println!("Querying took: {:?} for {} movies", elapsed, result.len());

    println!("\n\n Showing the top {} results:", num_results);
    for (i, movie) in result.iter().enumerate().take(num_results) {
        println!("{}. {}", i, movie);
    }
}
