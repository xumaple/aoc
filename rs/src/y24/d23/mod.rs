use util::*;

pub mod a;
pub mod b;

pub type IntType = usize;

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
struct Computer(String);

impl GraphCursor for Computer {}

struct NetworkMap(UndirectedGraph<Computer>);

impl FromStr for NetworkMap {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = UndirectedGraph::new();
        s.lines().for_each(|line| {
            let (a, b) = line.ssplit_once('-');
            graph.add_edge(Computer(a.to_string()), Computer(b.to_string()));
        });
        Ok(Self(graph))
    }
}

impl NetworkMap {
    fn trip_cliques(start: Cursor<Computer>) -> impl Iterator<Item = Vec<usize>> {
        start.next().flat_map(move |adj1| {
            adj1.next()
                .filter_map(move |adj2| {
                    adj2.connected_indices().contains(&start.index).then(|| {
                        let mut v = vec![start.index, adj1.index, adj2.index];
                        v.sort();
                        v
                    })
                })
                .collect_vec()
                .into_iter()
        })
    }
    pub fn find_historian_trip_cliques(&self) -> IntType {
        (0..self.0.values.len())
            .filter_map(|index| {
                let cs = self.0.cursor(index);
                cs.val().0.starts_with('t').then(|| Self::trip_cliques(cs))
            })
            .flatten()
            .collect::<HashSet<_>>()
            .len()
    }
}

#[cfg(test)]
mod test_a {
    use super::a::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d23/sample.txt").unwrap()).unwrap(), 7);
    }

    // #[test]
    // fn offical() {
    //     assert_eq!(run(read("src/y24/d23/input.txt").unwrap()).unwrap(), 0);
    // }
}

#[cfg(test)]
mod test_b {
    use super::b::run;
    use util::read;

    #[test]
    fn sample() {
        assert_eq!(run(read("src/y24/d23/sample.txt").unwrap()).unwrap(), 0);
    }

    // #[test]
    // fn offical() {
    //     assert_eq!(run(read("src/y24/d23/input.txt").unwrap()).unwrap(), 0);
    // }
}
