use std::cmp;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Vertex(usize);
type Weight = usize;

#[derive(Default, Debug)]
struct Graph {
    num_vertices: usize,
    edges: HashMap<Vertex, HashMap<Vertex, Weight>>,
}

impl Graph {
    fn add_edge(&mut self, v1: Vertex, v2: Vertex, w: usize) {
        if v1.0 == v2.0 {
            return;
        }

        self.edges
            .entry(v1)
            .or_default()
            .entry(v2)
            .and_modify(|old_weight| *old_weight = cmp::min(*old_weight, w))
            .or_insert(w);
        self.edges
            .entry(v2)
            .or_default()
            .entry(v1)
            .and_modify(|old_weight| *old_weight = cmp::min(*old_weight, w))
            .or_insert(w);
    }

    fn shortest_path(&self, start: Vertex, end: Vertex) -> Option<Vec<Vertex>> {
        if !self.edges.contains_key(&start) {
            return None;
        }
        // println!("Finding path {:?} {:?}", start, end);

        #[derive(Clone, Copy)]
        struct CacheItem {
            path_length: Weight,
            best_parent: Option<Vertex>,
        }

        let mut best_parent: HashMap<Vertex, CacheItem> = HashMap::new();

        #[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
        struct QueueItem {
            path_length: Reverse<Weight>,
            vertex: Vertex,
        }

        let mut priority_queue: BinaryHeap<QueueItem> = BinaryHeap::new();
        priority_queue.push(QueueItem {
            path_length: Reverse(0),
            vertex: start,
        });
        best_parent.insert(
            start,
            CacheItem {
                path_length: 0,
                best_parent: None,
            },
        );

        let mut visited: HashSet<Vertex> = HashSet::new();

        while let Some(top) = priority_queue.pop() {
            // println!("Top is {:?}", top);
            if visited.contains(&top.vertex) {
                continue;
            }

            let current_item = best_parent.get(&top.vertex).copied().unwrap();

            for (neighbor, neighbor_weight) in self.edges.get(&top.vertex).unwrap().iter() {
                // println!("Neighbor {:?}", neighbor);
                if visited.contains(neighbor) {
                    continue;
                }

                let length_to_neighbor = current_item.path_length + neighbor_weight;

                let neighbor_entry = best_parent.get(neighbor).copied();

                if neighbor_entry.is_none_or(|x| x.path_length > length_to_neighbor) {
                    best_parent.insert(
                        *neighbor,
                        CacheItem {
                            best_parent: Some(top.vertex),
                            path_length: length_to_neighbor,
                        },
                    );

                    priority_queue.push(QueueItem {
                        path_length: Reverse(length_to_neighbor),
                        vertex: *neighbor,
                    });
                }
            }

            visited.insert(top.vertex);

            if top.vertex == end {
                // println!("Found exit");
                break;
            }
        }

        if best_parent.get(&end).is_none() {
            return None;
        }

        let mut result = Vec::new();
        let mut last = Some(end);
        loop {
            match last {
                Some(vertex) => {
                    result.push(vertex);
                    last = best_parent.get(&vertex).unwrap().best_parent;
                }
                None => break,
            };
        }
        result.reverse();
        Some(result)
    }
}

fn parse_input() -> Result<Graph, Box<dyn Error>> {
    let mut result = Graph::default();

    let mut lines = io::stdin().lock().lines();

    let first_line = lines.next().expect("Should have line")?;
    let mut first_line_nums = first_line
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().expect("Should be numbers"));

    result.num_vertices = first_line_nums.next().unwrap();
    let num_edges = first_line_nums.next().unwrap();

    for _ in 0..num_edges {
        let line = lines.next().expect("Should have line")?;
        let mut line_nums = line.split_ascii_whitespace();

        let v1 = line_nums.next().unwrap().parse::<usize>().unwrap();
        let v2 = line_nums.next().unwrap().parse::<usize>().unwrap();
        let w = line_nums.next().unwrap().parse::<Weight>().unwrap();

        result.add_edge(Vertex(v1), Vertex(v2), w);
    }

    Ok(result)
}

fn main() {
    let graph = parse_input().unwrap();

    // println!("Graph: {:#?}", graph);
    // println!(
    //     "Shortest path: {:#?}",
    //     graph.shortest_path(Vertex(1), Vertex(graph.num_vertices))
    // )
    match graph.shortest_path(Vertex(1), Vertex(graph.num_vertices)) {
        None => {
            println!("-1");
        }
        Some(path) => {
            for num in path {
                print!("{} ", num.0)
            }
            println!();
        }
    }
}
