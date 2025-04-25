//! Implementation of Topological Sort using BFS (Kahn's Algorithm) for Directed Acyclic Graphs (DAGs).
//! Topological sort is used to solve dependencies of cells.

use std::collections::VecDeque;

/// Perform a topological sort on a directed graph represented as an adjacency list.
/// # Arguments
/// * `adj` - A reference to a vector of vectors representing the adjacency list of the graph.
/// * `cell` - The starting cell index (1-based).
/// * `indegree` - A mutable reference to a vector representing the indegree of each node.(zero initialized vector)
/// # Returns
/// A vector containing the topological order of the nodes. If a cycle is detected, the first element will be -1 else the first element will be the count of nodes in the connected component of cell.

pub fn topo_sort(adj: &[Vec<i32>], cell: i32, indegree: &mut [i32]) -> Vec<i32> {
    let mut q: VecDeque<i32> = VecDeque::new(); // queue initialization
    q.push_back(cell);
    let mut is_cycle = 0;
    let mut ct: i32 = 1;
    // run bfs to fill indegree and check for cycle
    while !q.is_empty() {
        if is_cycle == 1 {
            break;
        }
        let node = q.pop_front().unwrap();
        ct += 1;
        for c in &adj[node as usize] {
            if *c == cell {
                is_cycle = 1; // cycle detected
                break;
            }
            if indegree[*c as usize] == 0 {
                q.push_back(*c);
            }
            indegree[*c as usize] += 1;
        }
    }

    let mut res: Vec<i32> = vec![0; ct as usize];
    q.push_back(cell);
    if is_cycle == 1 {
        // cycle detected, make the first element of output array -1
        res[0] = -1;
        // we need to revert back changes in indegree
        while !q.is_empty() {
            let node = q.pop_front().unwrap();
            for c in &adj[node as usize] {
                if *c == cell {
                    break;
                }
                if indegree[*c as usize] > 0 {
                    q.push_back(*c);
                }
                indegree[*c as usize] = 0;
            }
        }
        return res;
    }
    // No cycle,so make first element of the output array to be count of cells in connected component of cell
    res[0] = ct - 1;
    let mut leng = 1;
    //Run Kahn's Algorithm
    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        res[leng as usize] = node;
        leng += 1;
        for c in &adj[node as usize] {
            indegree[*c as usize] -= 1;
            if indegree[*c as usize] == 0 {
                q.push_back(*c);
            }
        }
    }
    res
}
