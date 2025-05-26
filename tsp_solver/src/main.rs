use std::collections::HashMap;
use std::usize::MAX;

fn tsp(
    dist: &Vec<Vec<usize>>,
    pos: usize,
    visited: Vec<bool>,
    memo: &mut HashMap<(usize, Vec<bool>), (usize, Vec<usize>)>,
) -> (usize, Vec<usize>) {
    let n = dist.len();

    // Base case: all cities visited
    if visited.iter().all(|&x| x) {
        return (dist[pos][0], vec![0]);
    }

    // Memoization
    if let Some(result) = memo.get(&(pos, visited.clone())) {
        return result.clone();
    }

    let mut min_cost = MAX;
    let mut best_path = Vec::new();

    for next in 0..n {
        if !visited[next] && dist[pos][next] > 0 {
            let mut new_visited = visited.clone();
            new_visited[next] = true;

            let (cost, mut path) = tsp(dist, next, new_visited, memo);
            let total_cost = dist[pos][next] + cost;

            if total_cost < min_cost {
                min_cost = total_cost;
                path.insert(0, next);
                best_path = path;
            }
        }
    }

    memo.insert((pos, visited.clone()), (min_cost, best_path.clone()));
    (min_cost, best_path)
}

fn run_case(case_number: usize, dist: Vec<Vec<usize>>) {
    let n = dist.len();
    let mut visited = vec![false; n];
    visited[0] = true;

    let mut memo = HashMap::new();
    let (min_cost, mut path) = tsp(&dist, 0, visited, &mut memo);

    path.insert(0, 0); 

    println!("--- Case {} ---", case_number);
    println!("Minimum tour cost: {}", min_cost);
    println!("Path taken: {:?}\n", path);
}

fn main() {
    let test_cases = vec![
        vec![
            vec![0, 10, 15, 20],
            vec![10, 0, 35, 25],
            vec![15, 35, 0, 30],
            vec![20, 25, 30, 0],
        ],
        vec![
            vec![0, 29, 20, 21],
            vec![29, 0, 15, 17],
            vec![20, 15, 0, 28],
            vec![21, 17, 28, 0],
        ],
        vec![
            vec![0, 5, 9, 10],
            vec![5, 0, 6, 4],
            vec![9, 6, 0, 8],
            vec![10, 4, 8, 0],
        ],
    ];

    for (i, dist) in test_cases.into_iter().enumerate() {
        run_case(i + 1, dist);
    }
}
