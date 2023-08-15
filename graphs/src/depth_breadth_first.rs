use std::collections::LinkedList;
use std::collections::{HashSet, VecDeque};

pub struct Graph {
    adjacency_list: Vec<LinkedList<usize>>,
}

impl Graph {
    pub fn new(vertices: usize) -> Self {
        let adjacency_list = vec![LinkedList::new(); vertices];
        Graph { adjacency_list }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.adjacency_list[from].push_back(to);
    }

    // pub fn dfs(&self, start_vertex: usize, visited: &mut HashSet<usize>) {
    //     visited.insert(start_vertex);
    //     println!("Visited vertex: {}", start_vertex);

    //     for &adjacent_vertex in &self.adjacency_list[start_vertex] {
    //         if !visited.contains(&adjacent_vertex) {
    //             // process neighbors LIFO via call stack
    //             self.dfs(adjacent_vertex, visited);
    //         }
    //     }
    // }

    pub fn dfs(&self, start_vertex: usize) {
        println!("DFS");
        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        visited.insert(start_vertex);
        stack.push(start_vertex);

        while !stack.is_empty() {
            let current_vertex = stack.pop().unwrap();
            println!("Visited vertex: {}", current_vertex);

            for &adjacent_vertex in &self.adjacency_list[current_vertex] {
                if !visited.contains(&adjacent_vertex) {
                    visited.insert(adjacent_vertex);
                    stack.push(adjacent_vertex);
                }
            }
        }
    }

    pub fn bfs(&self, start_vertex: usize) {
        println!("BFS");

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start_vertex);
        queue.push_back(start_vertex);

        while !queue.is_empty() {
            let current_vertex = queue.pop_front().unwrap();
            println!("Visited vertex: {}", current_vertex);

            for &adjacent_vertex in &self.adjacency_list[current_vertex] {
                if !visited.contains(&adjacent_vertex) {
                    visited.insert(adjacent_vertex);
                    queue.push_back(adjacent_vertex);
                }
            }
        }
    }
}
