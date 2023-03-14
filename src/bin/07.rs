use core::panic;
use std::num::ParseIntError;

use petgraph::{Graph, adj::NodeIndex};

pub fn part_one(input: &str) -> Option<u32> {
    let directory = build_dir_tree(input);
    // println!("Example node: {:?}", directory[3]);
    // Find all of the directories with a total size of
    // at most 100000. What is the sum of the total
    // sizes of those directories?
    Some(42)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
#[derive(Debug)]
enum Types {
    File,
    Dir,
}

#[derive(Debug)]
struct Node {
    name: String,
    typ: Types,
    size: u64,
}
fn build_dir_tree(input: &str) -> Graph<Node, &str> {
    let mut dir_graph = Graph::<Node, &str>::new();
    // Todo: build up tree gradually
    let lines = input.lines();
    // let mut current_node = "root";  // keep track of where we are in tree
    // add root to graph
    let mut root = Node {
        name: String::from("root"),
        typ: Types::Dir,
        size: 0,
    };
    let mut current_node = dir_graph.add_node(root);
    for l in lines {
        match l.contains("$ cd"){
            true => match l.contains("$ cd ..") {
                true => move_to_parent(),
                false => move_to_child(),
            }
            false => match l.contains("$ ls") {
                true => ,
                false => add_node_to_graph(),
            },
        }
    }

    // add root to graph
    // let root_node = Node {
    //     name: String::from("root"),
    //     typ: Types::Dir,
    //     size: 0,
    // };
    // let root = dir_graph.add_node(root_node);
    // add all other nodes to graph
    // let nodes = get_nodes(input);

    // Todo: build up tree gradually
    // let file_iter = input.lines().

    return dir_graph;
}

fn move_to_parent(node: NodeIndex){
    
}

fn move_to_child(){
    // check if child node exists.
    // if so, move current_node
    // if not, first add new node to graph, then move current node.
    todo!()}

fn add_node_to_graph(){
    todo!()
    // add node, update node.index to index of node in tree
}

fn get_nodes(input: &str) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    let split = input.lines();
    for chunk in split.into_iter() {
        let mut vec: Vec<Node> = chunk.lines().filter_map(|l| make_node(l).ok()).collect();
        nodes.append(&mut vec);
    }
    return nodes;
}
fn make_node(line: &str) -> Result<Node, String> {
    // make a noder from line if it starts with "dir" or an int
    let mut split_line = line.split_ascii_whitespace();
    let content = split_line.next().unwrap();

    match content.parse::<u64>() {
        Ok(number) => {
            let result = Node {
                name: split_line.next().unwrap().to_string(),
                typ: Types::File,
                size: number,
            };
            return Ok(result);
        }
        Err(_e) => match content {
            "dir" => {
                let result = Node {
                    name: split_line.next().unwrap().to_string(),
                    typ: Types::Dir,
                    size: 0,
                };
                return Ok(result);
            }
            _ => {
                let result = "Error!".to_string();
                return Err(result);
            }
        },
    }
}

// fn some_func(){//function that does what we want to find out}
fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
