pub fn naive<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let mut pointer_a = 0;
    let mut pointer_b = 0;

    let mut result = Vec::new();

    while pointer_a < a.len() && pointer_b < b.len() {
        if a[pointer_a] < b[pointer_b] {
            pointer_a += 1;
        } else if a[pointer_a] > b[pointer_b] {
            pointer_b += 1;
        } else {
            result.push(a[pointer_a]);
            pointer_a += 1;
            pointer_b += 1;
        }
    }

    result
}

pub fn binary_search<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Ord + Copy,
{
    let mut result = Vec::new();

    for &item in b {
        // match binary_search_helper(a, item) {
        //     Some(_) => result.push(item),
        //     None => continue,
        match a.binary_search(&item) {
            Ok(_) => result.push(item),
            Err(_) => continue,
        }
    }

    result
}

use std::fmt::Display;

pub fn galloping_search<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Ord + Clone + Display,
{
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
            Ok(_) => {
                result.push(item.clone());
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

// fn binary_search_helper<T>(a: &[T], item: T) -> Option<usize>
// where
//     T: Ord,
// {
//     let mut low = 0;
//     let mut high = a.len();
//
//     while low < high {
//         let mid = (low + high) / 2;
//         if a[mid] == item {
//             return Some(mid);
//         } else if a[mid] < item {
//             low = mid + 1;
//         } else {
//             high = mid;
//         }
//     }
//
//     None
// }
