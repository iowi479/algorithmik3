use algorithmik3::intersection;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <n>", args[0]);
        std::process::exit(1);
    }

    let n: usize = args[1].parse().expect("Please provide a valid number n");

    let mut a = Vec::with_capacity(n);
    for i in 0..n {
        a.push(i as u64);
    }

    let log2 = (n as f64).log2() as usize;
    for i in 0..=log2 {
        let mut b;
        if i == 0 {
            // For the first iteration, we use the entire range
            b = Vec::with_capacity(i);
            b.extend(0..n as u64);
        } else {
            // For subsequent iterations, we reduce the size of b
            let div = n / (2_usize.pow(i as u32));

            let mut rng = rand::rng();
            b = rand::seq::index::sample(&mut rng, n, div)
                .into_iter()
                .map(|x| x as u64)
                .collect();

            b.sort_unstable();
        }

        let time = std::time::Instant::now();
        let result_naive = intersection::naive(&a, &b);
        let duration_naive = time.elapsed();

        let time = std::time::Instant::now();
        let result_binary = intersection::binary_search(&a, &b);
        let duration_binary = time.elapsed();

        let time = std::time::Instant::now();
        let result_galloping = intersection::galloping_search(&a, &b);
        let duration_galloping = time.elapsed();

        let width = n.to_string().len();
        let time_width = 6;

        println!(
            "|B| = {:>width$}: Naive: {:>time_width$} ms, Binary Search: {:>time_width$} ms, Galloping Search: {:>time_width$} ms",
            b.len(),
            duration_naive.as_millis(),
            duration_binary.as_millis(),
            duration_galloping.as_millis(),
            width = width,
            time_width = time_width
        );

        if result_naive != result_binary || result_naive != result_galloping {
            println!("Naive result: {:?}", result_naive);
            println!("Binary Search result: {:?}", result_binary);
            println!("Galloping Search result: {:?}", result_galloping);
            eprintln!("Results do not match!");
            std::process::exit(1);
        }
    }
}
