use crate::puzzle::Puzzle;
use petgraph::graph::UnGraph;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::Result;
use std::collections::HashMap;

pub struct Day {
    input: String,
}

impl Puzzle for Day {
    fn solve_part_1(&self) -> String {
        let mut graph = UnGraph::new_undirected();
        let mut nodes = HashMap::new();
        for line in self.input.lines() {
            let mut parts = line.split(':');
            let node = parts.next().unwrap();
            let edges = parts.next().unwrap().split_whitespace();
            let node_index = *nodes
                .entry(node)
                .or_insert_with(|| graph.add_node(node.to_string()));
            for edge in edges {
                let edge_index = *nodes
                    .entry(edge)
                    .or_insert_with(|| graph.add_node(edge.to_string()));
                graph.add_edge(node_index, edge_index, 1);
                graph.add_edge(edge_index, node_index, 1);
            }
        }
        let min_cut: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));
        let partition_size = min_cut.unwrap().unwrap().1.len();
        (partition_size * (graph.node_count() - partition_size)).to_string()
    }

    fn solve_part_2(&self) -> String {
        "Day 25 has no part 2".to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        Box::new(Day {
            input: input.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let input = "jqt: rhn xhk nvd\n\
            rsh: frs pzl lsr\n\
            xhk: hfx\n\
            cmg: qnr nvd lhk bvb\n\
            rhn: xhk bvb hfx\n\
            bvb: xhk hfx\n\
            pzl: lsr hfx nvd\n\
            qnr: nvd\n\
            ntq: jqt hfx bvb xhk\n\
            nvd: lhk\n\
            lsr: lhk\n\
            rzs: qnr cmg lsr rsh\n\
            frs: qnr lhk lsr";
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "54");
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/25")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "495607");
    }
}
