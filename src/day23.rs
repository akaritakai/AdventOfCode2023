use crate::puzzle::Puzzle;
use std::collections::{HashMap, HashSet};

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        let mut island = parse_island(&self.input);
        island.reduce_graph();
        island.longest_path_length().to_string()
    }

    fn solve_part_2(&self) -> String {
        let input = self.input.replace(['^', 'v', '<', '>'], ".");
        let mut island = parse_island(&input);
        island.reduce_graph();
        island.longest_path_length().to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Vertex {
    row: usize,
    col: usize,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Edge {
    src: Vertex,
    dst: Vertex,
    weight: usize,
}
struct Island {
    incoming_edges: HashMap<Vertex, HashSet<Edge>>,
    outgoing_edges: HashMap<Vertex, HashSet<Edge>>,
    start: Vertex,
    end: Vertex,
}

impl Island {
    fn reduce_graph(&mut self) {
        let mut modified = true;
        while modified {
            modified = false;
            for vertex in self.vertices() {
                if !self.vertex_exists(vertex) {
                    continue; // This vertex was reduced already
                }
                if vertex == self.start || vertex == self.end {
                    continue; // Don't reduce the start and end vertices
                }
                modified |= self.reduce_vertex(vertex);
            }
        }
        self.simplify_start_and_end();
    }

    fn reduce_vertex(&mut self, vertex: Vertex) -> bool {
        // Vertices of degree 2 can be reduced i.e.
        //   x <---m---> y <---n---> z
        // can be reduced to
        //   x <---m+n---> z
        let incoming_neighbors = self.incoming_neighbors_of(vertex);
        let outgoing_neighbors = self.outgoing_neighbors_of(vertex);
        if incoming_neighbors.len() != 2
            || outgoing_neighbors.len() != 2
            || incoming_neighbors != outgoing_neighbors
        {
            return false; // This vertex does not have degree 2
        }
        let neighbors = incoming_neighbors.iter().collect::<Vec<_>>();
        let src = *neighbors[0];
        let dst = *neighbors[1];
        if self.get_edge(src, dst).is_some() {
            return false; // This vertex already exists
        }
        let weight =
            self.get_edge(src, vertex).unwrap().weight + self.get_edge(vertex, dst).unwrap().weight;
        self.remove_edge(src, vertex);
        self.remove_edge(vertex, src);
        self.remove_edge(vertex, dst);
        self.remove_edge(dst, vertex);
        self.add_edge(src, dst, weight);
        self.add_edge(dst, src, weight);
        if !self.vertex_exists(vertex) {
            self.incoming_edges.remove(&vertex);
            self.outgoing_edges.remove(&vertex);
        }
        true
    }

    fn simplify_start_and_end(&mut self) {
        // Remove all incoming edges to the start vertex
        let start_incoming = self.incoming_edges.get(&self.start).unwrap().clone();
        for edge in start_incoming {
            self.remove_edge(edge.src, edge.dst);
        }
        let start_outgoing = self.outgoing_edges.get(&self.start).unwrap().clone();
        if start_outgoing.len() == 1 {
            let node = start_outgoing.iter().next().unwrap().dst;
            // Remove all incoming edges to the node except the one from the start vertex
            let node_incoming = self.incoming_edges.get(&node).unwrap().clone();
            for edge in node_incoming {
                if edge.src != self.start {
                    self.remove_edge(edge.src, edge.dst);
                }
            }
        }
        // Remove all outgoing edges from the end vertex
        let outgoing = self.outgoing_edges.get(&self.end).unwrap().clone();
        for edge in outgoing {
            self.remove_edge(edge.src, edge.dst);
        }
        let end_incoming = self.incoming_edges.get(&self.end).unwrap().clone();
        if end_incoming.len() == 1 {
            let node = end_incoming.iter().next().unwrap().src;
            // Remove all outgoing edges from the node except the one to the end vertex
            let node_outgoing = self.outgoing_edges.get(&node).unwrap().clone();
            for edge in node_outgoing {
                if edge.dst != self.end {
                    self.remove_edge(edge.src, edge.dst);
                }
            }
        }
    }

    fn get_edge(&self, src: Vertex, dst: Vertex) -> Option<Edge> {
        if let Some(edges) = self.outgoing_edges.get(&src) {
            for edge in edges {
                if edge.dst == dst {
                    return Some(*edge);
                }
            }
        }
        None
    }

    fn vertex_exists(&self, vertex: Vertex) -> bool {
        if let Some(edges) = self.incoming_edges.get(&vertex) {
            if !edges.is_empty() {
                return true;
            }
        }
        if let Some(edges) = self.outgoing_edges.get(&vertex) {
            if !edges.is_empty() {
                return true;
            }
        }
        false
    }

    fn vertices(&self) -> HashSet<Vertex> {
        let mut vertices = HashSet::new();
        for vertex in self.incoming_edges.keys() {
            vertices.insert(*vertex);
        }
        for vertex in self.outgoing_edges.keys() {
            vertices.insert(*vertex);
        }
        vertices
    }

    fn incoming_neighbors_of(&self, vertex: Vertex) -> HashSet<Vertex> {
        let mut neighbors = HashSet::new();
        if let Some(edges) = self.incoming_edges.get(&vertex) {
            for edge in edges {
                neighbors.insert(edge.src);
            }
        }
        neighbors
    }

    fn outgoing_neighbors_of(&self, vertex: Vertex) -> HashSet<Vertex> {
        let mut neighbors = HashSet::new();
        if let Some(edges) = self.outgoing_edges.get(&vertex) {
            for edge in edges {
                neighbors.insert(edge.dst);
            }
        }
        neighbors
    }

    fn add_edge(&mut self, src: Vertex, dst: Vertex, weight: usize) {
        self.incoming_edges
            .get_mut(&dst)
            .unwrap()
            .insert(Edge { src, dst, weight });
        self.outgoing_edges
            .get_mut(&src)
            .unwrap()
            .insert(Edge { src, dst, weight });
    }

    fn remove_edge(&mut self, src: Vertex, dst: Vertex) {
        self.incoming_edges
            .get_mut(&dst)
            .unwrap()
            .retain(|edge| edge.src != src);
        self.outgoing_edges
            .get_mut(&src)
            .unwrap()
            .retain(|edge| edge.dst != dst);
    }

    fn longest_path_length(&self) -> usize {
        let rows = self.end.row + 1;
        let cols = self.end.col + 1;

        let mut memo: Vec<Vec<Option<usize>>> = vec![vec![None; cols]; rows];
        let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
        self.longest_path_length_impl(self.start, 0, &mut visited, &mut memo);
        memo[self.end.row][self.end.col].unwrap()
    }

    fn longest_path_length_impl(
        &self,
        vertex: Vertex,
        distance: usize,
        visited: &mut Vec<Vec<bool>>,
        memo: &mut Vec<Vec<Option<usize>>>,
    ) {
        if let Some(best) = memo[vertex.row][vertex.col] {
            if distance < best {
                return;
            }
        }
        if visited[vertex.row][vertex.col] {
            return;
        }
        if vertex == self.end {
            memo[vertex.row][vertex.col] = Some(distance);
            return;
        }
        visited[vertex.row][vertex.col] = true;
        if let Some(edges) = self.outgoing_edges.get(&vertex) {
            for edge in edges {
                self.longest_path_length_impl(edge.dst, distance + edge.weight, visited, memo);
            }
        }
        visited[vertex.row][vertex.col] = false;
    }
}

fn parse_island(input: &str) -> Island {
    // First, parse the input into a Vec<Vec<char>>
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (rows, cols) = (grid.len(), grid[0].len());

    // Add all the nodes to the graph
    let mut incoming_edges = HashMap::new();
    let mut outgoing_edges = HashMap::new();
    for (row, line) in grid.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c != '#' {
                incoming_edges.insert(Vertex { row, col }, HashSet::new());
                outgoing_edges.insert(Vertex { row, col }, HashSet::new());
            }
        }
    }

    // Add all the edges.
    let mut add_edge = |src: Vertex, dst: Vertex| {
        incoming_edges.get_mut(&dst).unwrap().insert(Edge {
            src,
            dst,
            weight: 1,
        });
        outgoing_edges.get_mut(&src).unwrap().insert(Edge {
            src,
            dst,
            weight: 1,
        });
    };
    for row1 in 0..rows {
        for col1 in 0..cols {
            if grid[row1][col1] == '#' {
                continue;
            }
            let node1 = Vertex {
                row: row1,
                col: col1,
            };
            for (drow, dcol) in &[(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let (row2, col2) = (row1 as i32 + drow, col1 as i32 + dcol);
                if row2 < 0
                    || row2 >= rows as i32
                    || col2 < 0
                    || col2 >= cols as i32
                    || grid[row2 as usize][col2 as usize] == '#'
                {
                    continue;
                }
                let node2 = Vertex {
                    row: row2 as usize,
                    col: col2 as usize,
                };
                match grid[row1][col1] {
                    '.' => {
                        add_edge(node1, node2);
                    }
                    '^' if *drow == -1 => {
                        add_edge(node1, node2);
                    }
                    'v' if *drow == 1 => {
                        add_edge(node1, node2);
                    }
                    '<' if *dcol == -1 => {
                        add_edge(node1, node2);
                    }
                    '>' if *dcol == 1 => {
                        add_edge(node1, node2);
                    }
                    _ => {}
                };
            }
        }
    }

    Island {
        incoming_edges,
        outgoing_edges,
        start: Vertex { row: 0, col: 1 },
        end: Vertex {
            row: rows - 1,
            col: cols - 2,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "#.#####################\n\
            #.......#########...###\n\
            #######.#########.#.###\n\
            ###.....#.>.>.###.#.###\n\
            ###v#####.#v#.###.#.###\n\
            ###.>...#.#.#.....#...#\n\
            ###v###.#.#.#########.#\n\
            ###...#.#.#.......#...#\n\
            #####.#.#.#######.#.###\n\
            #.....#.#.#.......#...#\n\
            #.#####.#.#.#########v#\n\
            #.#...#...#...###...>.#\n\
            #.#.#v#######v###.###v#\n\
            #...#.>.#...>.>.#.###.#\n\
            #####v#.#.###v#.#.###.#\n\
            #.....#...#...#.#.#...#\n\
            #.#########.###.#.#.###\n\
            #...###...#...#...#.###\n\
            ###.###.#.###v#####v###\n\
            #...#...#.#.>.>.#.>.###\n\
            #.###.###.#.###.#.#v###\n\
            #.....###...###...#...#\n\
            #####################.#";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "94");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/23")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "2334");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "#.#####################\n\
            #.......#########...###\n\
            #######.#########.#.###\n\
            ###.....#.>.>.###.#.###\n\
            ###v#####.#v#.###.#.###\n\
            ###.>...#.#.#.....#...#\n\
            ###v###.#.#.#########.#\n\
            ###...#.#.#.......#...#\n\
            #####.#.#.#######.#.###\n\
            #.....#.#.#.......#...#\n\
            #.#####.#.#.#########v#\n\
            #.#...#...#...###...>.#\n\
            #.#.#v#######v###.###v#\n\
            #...#.>.#...>.>.#.###.#\n\
            #####v#.#.###v#.#.###.#\n\
            #.....#...#...#.#.#...#\n\
            #.#########.###.#.#.###\n\
            #...###...#...#...#.###\n\
            ###.###.#.###v#####v###\n\
            #...#...#.#.>.>.#.>.###\n\
            #.###.###.#.###.#.#v###\n\
            #.....###...###...#...#\n\
            #####################.#";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "154");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/23")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "6422");
    }
}
