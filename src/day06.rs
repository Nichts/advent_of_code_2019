use std::fs::read_to_string;

use anyhow::Result;
use ego_tree::{NodeId, NodeMut, NodeRef, Tree};
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, line_ending};
use nom::combinator::map;
use nom::multi::separated_list;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Orbit<'s> {
    object: &'s str,
    satellite: &'s str,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Body<'s> {
    name: &'s str,
}

impl<'s> Body<'s> {
    fn new(name: &'s str) -> Self {
        Self { name }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Orbit>> {
    let pair = separated_pair(alphanumeric1, tag(")"), alphanumeric1);
    let map = map(pair, |(object, satellite)| Orbit { object, satellite });
    let parser = separated_list(line_ending, map);
    parser(input)
}

fn build_subtree<'n, 'b: 'n>(
    node: &'n mut NodeMut<'_, Body<'b>>,
    satellites: &HashMap<&'b str, Vec<&'b str>>,
    node_ids: &mut HashMap<&'b str, NodeId>,
) {
    if let Some(object_satellites) = satellites.get(node.value().name) {
        for satellite in object_satellites {
            let mut satellite_node = node.append(Body::new(satellite));
            node_ids.insert(satellite, satellite_node.id());
            build_subtree(&mut satellite_node, satellites, node_ids);
        }
    }
}

struct OrbitTree<'s> {
    tree: Tree<Body<'s>>,
    node_ids: HashMap<&'s str, NodeId>,
}

impl<'s> OrbitTree<'s> {
    fn build(input: &'s str) -> Result<Self> {
        let result = parse(input.trim()).map_err(|_| ::anyhow::anyhow!("Parse failed"))?;
        assert_eq!(result.0.len(), 0);
        let mut satellites: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut node_ids: HashMap<&str, NodeId> = HashMap::new();
        result.1.iter().for_each(|orbit| {
            satellites
                .entry(orbit.object)
                .or_insert_with(|| vec![])
                .push(orbit.satellite)
        });
        let mut tree = Tree::new(Body::new("COM"));
        {
            let mut node = tree.root_mut();
            node_ids.insert(node.value().name, node.id());
            build_subtree(&mut node, &satellites, &mut node_ids);
        }
        Ok(Self { tree, node_ids })
    }

    fn count(node: &NodeRef<Body>, depth: u64) -> u64 {
        let mut children = 0;
        node.children().for_each(|child| {
            children += Self::count(&child, depth + 1);
        });
        children + depth
    }

    fn total_orbits(&self) -> u64 {
        Self::count(&self.tree.root(), 0)
    }

    fn distance(&self, a: &str, b: &str) -> Result<usize> {
        let parents = |node| -> Result<_> {
            Ok(self
                .tree
                .get(
                    *self
                        .node_ids
                        .get(node)
                        .ok_or_else(|| ::anyhow::anyhow!("Node not found"))?,
                )
                .ok_or_else(|| ::anyhow::anyhow!("Node not found"))?
                .ancestors())
        };
        let parent_dist = parents(a)?
            .enumerate()
            .map(|(dist, node)| (node.value().name, dist))
            .collect::<HashMap<_, _>>();
        let dist = parents(b)?
            .enumerate()
            .filter_map(|(dist2, node)| {
                parent_dist
                    .get(&node.value().name)
                    .map(|dist1| dist1 + dist2)
            })
            .next()
            .map(|dist| dist)
            .ok_or_else(|| ::anyhow::anyhow!("Nodes don't have the same root"))?;

        Ok(dist)
    }
}

pub fn main() -> Result<()> {
    let input = read_to_string("data/day06.txt")?;
    let tree = OrbitTree::build(&input)?;
    println!("Part 1: {}", tree.total_orbits());
    println!("Part 2: {}", tree.distance("YOU", "SAN")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }

    #[test]
    fn test_p1() -> Result<()> {
        let input = "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";
        assert_eq!(OrbitTree::build(&input)?.total_orbits(), 42);
        Ok(())
    }

    #[test]
    fn test_p2() -> Result<()> {
        let input = "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
";
        assert_eq!(OrbitTree::build(&input)?.distance("YOU", "SAN")?, 4);
        Ok(())
    }
}
