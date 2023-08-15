use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Vertex {
    id: usize,
    distance: i32,
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance) // Min-heap
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra_shortest_path(graph: &[Vec<(usize, i32)>], start: usize) -> Vec<i32> {
    let mut distances: Vec<i32> = vec![std::i32::MAX; graph.len()];
    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::new();

    distances[start] = 0;
    heap.push(Vertex {
        id: start,
        distance: 0,
    });

    while let Some(Vertex { id, distance }) = heap.pop() {
        if visited.contains_key(&id) {
            continue;
        }

        visited.insert(id, distance);

        for &(neighbor, weight) in &graph[id] {
            let new_distance = distance + weight;
            if new_distance < distances[neighbor] {
                distances[neighbor] = new_distance;
                heap.push(Vertex {
                    id: neighbor,
                    distance: new_distance,
                });
            }
        }
    }

    distances
}
