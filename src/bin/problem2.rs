use algorithmik3::search_engine::InvertedIndeciesHash;

fn main() {
    let timeout = std::time::Instant::now();
    let indecies = InvertedIndeciesHash::new("movies.txt");
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

    println!("Results:");
    for movie in result {
        println!("{}", movie);
    }
}
