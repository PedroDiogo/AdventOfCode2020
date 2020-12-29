use super::lib::*;
extern crate regex;
use self::regex::Regex;

extern crate petgraph;
use self::petgraph::graph::{DiGraph, Graph, NodeIndex};
use self::petgraph::graphmap::DiGraphMap;
use self::petgraph::visit::EdgeRef;

lazy_static! {
    static ref RE_TO: Regex = Regex::new(r"(?P<to_bag>\w+ \w+) bags contain").unwrap();
    static ref RE_FROM: Regex =
        Regex::new(r"(?P<quantity>\d+) (?P<from_bag>\w+ \w+) bags?").unwrap();
}

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day7.txt";
    let inputs = read_inputs(&filename);

    let mut graph = build_graph_from_inputs(&inputs);
    let start_node = graph
        .find_by_weight("shiny gold")
        .expect("Expected to find a Shiny Gold node");

    let part_one = Some(find_number_of_downstream_nodes(&graph, &start_node).to_string());

    graph.reverse();
    let part_two = Some(find_number_of_contained_nodes(&graph, &start_node).to_string());

    (part_one, part_two)
}

trait FindByWeight<N: Eq> {
    fn find_by_weight(&self, weight: N) -> Option<NodeIndex>;
}

impl<N: Eq, E> FindByWeight<N> for Graph<N, E> {
    fn find_by_weight(&self, weight: N) -> Option<NodeIndex> {
        self.node_indices()
            .find(|node_idx| self[*node_idx] == weight)
    }
}

fn build_graph_from_inputs(inputs: &str) -> DiGraph<&str, u32> {
    let graphmap: DiGraphMap<&str, u32> = inputs.lines().filter_map(parse_input_into_from_to).fold(
        DiGraphMap::new(),
        |mut graph, (from_vec, to)| {
            from_vec.iter().for_each(|(quantity, from)| {
                graph.add_edge(*from, to, *quantity);
            });
            graph
        },
    );

    graphmap.clone().into_graph::<u32>()
}

fn parse_input_into_from_to(input: &str) -> Option<(Vec<(u32, &str)>, &str)> {
    let to = RE_TO.captures(input).unwrap().name("to_bag");
    let from: Vec<(u32, &str)> = RE_FROM
        .captures_iter(input)
        .filter_map(|x| match (x.name("quantity"), x.name("from_bag")) {
            (Some(quantity), Some(from)) => Some((quantity, from)),
            _ => None,
        })
        .map(|(quantity, from)| (quantity.as_str().parse::<u32>().unwrap(), from.as_str()))
        .collect();

    match (to, from.is_empty()) {
        (Some(to), false) => Some((from, to.as_str())),
        _ => None,
    }
}

fn find_number_of_downstream_nodes(graph: &DiGraph<&str, u32>, start_node: &NodeIndex) -> u32 {
    let mut bfs = petgraph::visit::Bfs::new(&graph, *start_node);

    let mut node_count = 0;
    while bfs.next(&graph).is_some() {
        node_count += 1;
    }
    node_count - 1
}

fn find_number_of_contained_nodes(graph: &DiGraph<&str, u32>, start_node: &NodeIndex) -> u32 {
    graph
        .edges(*start_node)
        .map(|edge| edge.weight() * (find_number_of_contained_nodes(graph, &edge.target()) + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use self::petgraph::algo::is_isomorphic;
    use super::*;

    const TEST_INPUT_1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const TEST_INPUT_2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn test_parse_graph() {
        let mut graph = DiGraphMap::<&str, u32>::new();

        graph.add_edge("bright white", "light red", 0);
        graph.add_edge("muted yellow", "light red", 0);

        graph.add_edge("bright white", "dark orange", 0);
        graph.add_edge("muted yellow", "dark orange", 0);

        graph.add_edge("shiny gold", "bright white", 0);

        graph.add_edge("shiny gold", "muted yellow", 0);
        graph.add_edge("faded blue", "muted yellow", 0);

        graph.add_edge("dark olive", "shiny gold", 0);
        graph.add_edge("vibrant plum", "shiny gold", 0);

        graph.add_edge("faded blue", "dark olive", 0);
        graph.add_edge("dotted black", "dark olive", 0);

        graph.add_edge("faded blue", "vibrant plum", 0);
        graph.add_edge("dotted black", "vibrant plum", 0);

        let graph = graph.into_graph::<u32>();
        let input_graph = build_graph_from_inputs(&TEST_INPUT_1);

        assert_eq!(true, is_isomorphic(&graph, &input_graph));
    }

    #[test]
    fn test_parse_input_into_from_to() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let expected_output = Some((vec![(1, "bright white"), (2, "muted yellow")], "light red"));
        let empty_input = "faded blue bags contain no other bags.";

        assert_eq!(expected_output, parse_input_into_from_to(&input));
        assert_eq!(None, parse_input_into_from_to(&empty_input));
    }

    #[test]
    fn test_find_number_of_downstream_nodes() {
        let graphmap = build_graph_from_inputs(&TEST_INPUT_1);
        let start_node = graphmap.find_by_weight("shiny gold").unwrap();
        assert_eq!(4, find_number_of_downstream_nodes(&graphmap, &start_node));
    }

    #[test]
    fn test_find_number_of_contained_nodes() {
        let mut graphmap = build_graph_from_inputs(&TEST_INPUT_1);
        graphmap.reverse();

        let start_node = graphmap.find_by_weight("shiny gold").unwrap();
        assert_eq!(32, find_number_of_contained_nodes(&graphmap, &start_node));

        let mut graphmap = build_graph_from_inputs(&TEST_INPUT_2);
        graphmap.reverse();

        let start_node = graphmap.find_by_weight("shiny gold").unwrap();
        assert_eq!(126, find_number_of_contained_nodes(&graphmap, &start_node));
    }
}
