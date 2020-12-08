use super::lib::*;
extern crate regex;
use self::regex::Regex;

extern crate petgraph;
use self::petgraph::graphmap::DiGraphMap;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day7.txt";
    let inputs = read_inputs(&filename);

    let graph = build_graph_from_inputs(&inputs);
    let part_one = Some(find_number_of_downstream_nodes(&graph, "shiny gold").to_string());
    let part_two = None;

    (part_one, part_two)
}

fn build_graph_from_inputs(inputs: &str) -> DiGraphMap<&str, usize> {
    inputs.lines().filter_map(parse_input_into_from_to).fold(
        DiGraphMap::new(),
        |mut graph, (from_vec, to)| {
            from_vec.iter().for_each(|from| {
                graph.add_edge(from, to, 0);
            });
            graph
        },
    )
}

fn parse_input_into_from_to(input: &str) -> Option<(Vec<&str>, &str)> {
    let re_to = Regex::new(r"(\w+ \w+) bags contain").unwrap();
    let to = re_to.captures(input).unwrap().get(1);
    let re_from = Regex::new(r"\d+ (\w+ \w+) bags?").unwrap();
    let from: Vec<&str> = re_from
        .captures_iter(input)
        .filter_map(|x| x.get(1))
        .map(|x| x.as_str())
        .collect();

    match (to, from.is_empty()) {
        (Some(to), false) => Some((from, to.as_str())),
        _ => None,
    }
}

fn find_number_of_downstream_nodes(graph: &DiGraphMap<&str, usize>, start: &str) -> usize {
    let graph = graph.clone().into_graph::<usize>();

    let start_node = graph
        .node_indices()
        .find(|node_idx| graph.node_weight(*node_idx) == Some(&start))
        .expect("Expected to find a start node");

    let mut bfs = petgraph::visit::Bfs::new(&graph, start_node);

    let mut node_count = 0;
    while bfs.next(&graph).is_some() {
        node_count += 1;
    }
    node_count - 1
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

    #[test]
    fn test_parse_graph() {
        let mut graph = DiGraphMap::<&str, usize>::new();

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

        let graph = graph.into_graph::<usize>();
        let input_graph = &build_graph_from_inputs(&TEST_INPUT_1).into_graph();

        assert_eq!(true, is_isomorphic(&graph, &input_graph));
    }

    #[test]
    fn test_parse_input_into_from_to() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let expected_output = Some((vec!["bright white", "muted yellow"], "light red"));
        let empty_input = "faded blue bags contain no other bags.";

        assert_eq!(expected_output, parse_input_into_from_to(&input));
        assert_eq!(None, parse_input_into_from_to(&empty_input));
    }

    #[test]
    fn test_find_number_of_downstream_nodes() {
        let graphmap = build_graph_from_inputs(&TEST_INPUT_1);
        assert_eq!(4, find_number_of_downstream_nodes(&graphmap, "shiny gold"));
    }
}
