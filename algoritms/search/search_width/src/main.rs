// #![warn(clippy::all, clippy::pedantic)]

// use rand::Rng;
// use std::collections::{HashMap, VecDeque};

// fn main() {
//     // let mut vector = VecDeque::new();
//     // vector.push_back(1);

//     let graph = create_graph(6);
//     // let test = 'a';
//     // println!("{:#?}", graph[&test]);
// }

// fn create_graph(nodes_count: usize) -> HashMap<char, Vec<char>> {
//     let mut graph = HashMap::new();

//     let nodes: Vec<char> = (b'a'..=b'z')
//         .take(nodes_count as usize)
//         .map(|c| c as char)
//         .collect();

//     // let mut nodes: Vec<char> = vec![];
//     // for c in ('a'..='z').take(nodes_count as usize).enumerate() {
//     //     nodes.push(c)
//     // }

//     let mut all_edges_count = rand::thread_rng().gen_range(nodes_count..nodes_count * 2);

//     for i in 0..nodes_count - 1 {
//         let node = nodes[i as usize];
//         let node_edges_count: usize;

//         if all_edges_count >= 3 {
//             node_edges_count = rand::thread_rng().gen_range(1..3);
//             all_edges_count -= node_edges_count;
//         } else if all_edges_count == 0 {
//             node_edges_count = 0;
//         } else {
//             node_edges_count = rand::thread_rng().gen_range(1..all_edges_count);
//             all_edges_count -= node_edges_count;
//         }

//         let mut node_edges: Vec<char> = vec![];

//         for _ in 0..node_edges_count {
//             let random_node = nodes[rand::thread_rng().gen_range(0..nodes_count) as usize];
//             if !node_edges.contains(&random_node) {
//                 node_edges.push(random_node);
//             }
//         }
//         graph.insert(node, node_edges);
//         // println!("{}", node);
//     }

//     graph

//     // let mut peoples_count = rand::thread_rng().gen_range(5..=10);
//     // graph.insert(String::from(peoples[i]), my_friends);
// }

#![warn(clippy::all, clippy::pedantic)]

use rand::Rng;
use std::collections::{HashMap, VecDeque};

fn main() {
    let graph = create_graph(6);
    println!("{:#?}", graph);
}

fn create_graph(nodes_count: usize) -> HashMap<char, Vec<char>> {
    let mut graph = HashMap::new();

    let nodes: Vec<char> = (b'a'..=b'z')
        .take(nodes_count as usize)
        .map(|c| c as char)
        .collect();

    let mut all_edges_count = 10;

    for i in 0..nodes_count - 1 {
        let node = nodes[i as usize];
        let node_edges_count: usize;

        if all_edges_count >= 3 {
            node_edges_count = rand::thread_rng().gen_range(1..3);
            all_edges_count -= node_edges_count;
        } else if all_edges_count == 0 {
            node_edges_count = 0;
        } else {
            node_edges_count = rand::thread_rng().gen_range(1..all_edges_count);
            all_edges_count -= node_edges_count;
        }

        let mut node_edges: Vec<char> = vec![];

        for _ in 0..node_edges_count {
            let random_node = nodes[rand::thread_rng().gen_range(0..=nodes_count) as usize];
            if !node_edges.contains(&random_node) {
                node_edges.push(random_node);
            }
        }
        graph.insert(node, node_edges);
    }

    graph
}
