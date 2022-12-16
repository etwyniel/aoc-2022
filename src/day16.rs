use aoc_framework::*;
use std::{borrow::Cow, collections::HashSet, fmt::Write};

use fxhash::FxHashMap;

pub struct Day16;

impl_day!(Day16::{Part1, Part2}: 2022[16], r"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
");

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct StateId(u16);

const OPEN_OFFSET: u16 = 26 * 26;
const STEPS_PT1: u8 = 30;

impl std::fmt::Debug for StateId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = if self.0 > OPEN_OFFSET {
            f.write_str("open ")?;
            self.0 - OPEN_OFFSET
        } else {
            self.0
        };
        f.write_char(((id / 26) as u8 + b'A') as char)?;
        f.write_char(((id % 26) as u8 + b'A') as char)
    }
}

impl StateId {
    fn is_open(self) -> bool {
        self.0 >= OPEN_OFFSET
    }

    fn open(self) -> Self {
        StateId(self.0 + OPEN_OFFSET)
    }
}

#[derive(Debug, Clone)]
struct Adj {
    rate: u8,
    dests: Vec<(StateId, u8)>,
}

fn parse_id(s: &str) -> StateId {
    let bytes = s.as_bytes();
    StateId((bytes[0] - b'A') as u16 * 26 + (bytes[1] - b'A') as u16)
}

#[derive(PartialEq, Eq, Hash)]
struct SearchState<'a> {
    opened: Cow<'a, Vec<StateId>>,
    step: u8,
    current: StateId,
}

#[derive(PartialEq, Eq, Hash)]
struct SearchState2<'a> {
    opened: Cow<'a, Vec<StateId>>,
    step: u8,
    current: [StateId; 2],
}

#[derive(Default, Debug, Clone)]
struct Graph(fxhash::FxHashMap<StateId, Adj>, usize);

impl Graph {
    fn add_line(&mut self, line: &str) {
        let mut parts = line.split([' ', ';', ',', '=']);
        let id = parse_id(parts.nth(1).unwrap());
        let rate = parts.nth(3).unwrap().parse().unwrap();
        let mut dests: Vec<_> = parts
            .skip(5)
            .filter(|id| !id.is_empty())
            .map(parse_id)
            .map(|id| (id, 1))
            .collect();
        if rate > 0 {
            let open_id = id.open();
            self.0.insert(
                open_id,
                Adj {
                    rate,
                    dests: dests.clone(),
                },
            );
            dests.push((open_id, 1));
            self.1 += 1;
        }
        self.0.insert(id, Adj { rate: 0, dests });
    }

    fn shortest_path(&self, s1: StateId, s2: StateId) -> u8 {
        let mut dist = FxHashMap::default();
        let mut q = Vec::new();
        for &id in self.0.keys() {
            dist.insert(id, u8::MAX);
            q.push(id);
        }
        dist.insert(s1, 0);
        while !q.is_empty() {
            let (ndx, u) = q
                .iter()
                .copied()
                .enumerate()
                .min_by_key(|(_, id)| dist[id])
                .unwrap();
            if u == s2 {
                return dist[&u];
            }
            q.remove(ndx);
            for &(v, d) in self.0[&u].dests.iter().filter(|&(id, _)| q.contains(id)) {
                let alt = dist[&u] + d;
                if alt < dist[&v] {
                    dist.insert(v, alt);
                }
            }
        }
        panic!("no path found")
    }

    fn reduce(&mut self) {
        let ids = self
            .0
            .iter()
            .filter(|(id, node)| node.rate > 0 || id.0 == 0)
            .map(|(k, _)| *k)
            .collect::<Vec<_>>();
        for &id1 in ids.iter() {
            for &id2 in ids.iter() {
                // .skip(n + 1) {
                if id1 == id2 {
                    continue;
                }
                let dist = self.shortest_path(id1, id2);
                let adj = self.0.get_mut(&id1).unwrap();
                if !adj.dests.iter().any(|&(id, _)| id == id2) {
                    adj.dests.push((id2, dist));
                }
                // self.0.get_mut(&id2).unwrap().dests.push((id1, dist));
            }
        }
        self.0.retain(|k, _| ids.contains(k));
        self.0
            .values_mut()
            .for_each(|adj| adj.dests.retain(|(id, _)| ids.contains(id)));
    }

    fn bfs<'a, 'b: 'a>(
        &self,
        step: u8,
        flow: u64,
        state: StateId,
        visited: Cow<'b, Vec<StateId>>,
        memo: &'a mut FxHashMap<SearchState<'b>, (u64, Vec<(u8, StateId)>)>,
    ) -> (u64, Vec<(u8, StateId)>) {
        if visited.len() == self.1 {
            return (flow * (STEPS_PT1 + 1 - step) as u64, vec![(step, state)]);
        }
        let opened = visited.clone();
        let search_state = SearchState {
            current: state,
            step,
            opened,
        };
        if let Some((flow, path)) = memo.get(&search_state) {
            return (*flow, path.clone());
        }
        let s = self.0.get(&state).unwrap();
        if step >= STEPS_PT1 {
            memo.insert(search_state, (flow, vec![(step, state)]));
            return (flow, vec![(step, state)]);
        }
        let mut visited = visited.clone();
        if state.is_open() {
            visited.to_mut().push(state);
        }

        let new_flow = flow + s.rate as u64;
        let best_flow_dist = (STEPS_PT1 - step + 1) as u64;
        let mut best_flow = flow + new_flow * (best_flow_dist - 1) as u64;
        let mut best_path = Vec::new();
        for (dest, dist) in &s.dests {
            if visited.contains(dest) || step + dist > STEPS_PT1 {
                continue;
            }
            let (next_flow, next_path) =
                self.bfs(step + dist, new_flow, *dest, visited.clone(), memo);
            let next_flow = flow + new_flow * (dist - 1) as u64 + next_flow;
            if next_flow >= best_flow {
                best_flow = next_flow;
                best_path = next_path;
            }
        }
        best_path.push((step, state));
        memo.insert(search_state, (best_flow, best_path.clone()));
        (best_flow, best_path)
    }

    fn bfs2<'a, 'b: 'a>(
        &self,
        step: u8,
        flow: u64,
        states: [StateId; 2],
        visited: Cow<'b, Vec<StateId>>,
        memo: &'a mut FxHashMap<SearchState2<'b>, u64>,
    ) -> u64 {
        // dbg!(step);
        // if step == 15 {
        //     eprintln!("memo size: {}", memo.len());
        // }
        if visited.len() == self.1 {
            // eprintln!("Skipping {} steps", 27 - step);
            return flow * (27 - step as u64);
        }
        let opened = visited.clone();
        let search_state = SearchState2 {
            current: states,
            step,
            opened,
        };
        if let Some(flow) = memo.get(&search_state) {
            return *flow;
        }
        if step >= 26 {
            memo.insert(search_state, flow);
            return flow;
        }
        let s1 = self.0.get(&states[0]).unwrap();
        let s2 = self.0.get(&states[1]).unwrap();
        let mut visited = visited.clone();
        for s in states {
            if s.is_open() {
                visited.to_mut().push(s);
            }
        }

        let mut new_flow = flow + s1.rate as u64;
        if states[0] != states[1] {
            new_flow += s2.rate as u64;
        }
        let mut best_flow = flow;
        for &(dest1, _) in &s1.dests {
            if visited.contains(&dest1) {
                continue;
            }
            for &(dest2, _) in &s2.dests {
                if visited.contains(&dest2) {
                    continue;
                }
                let mut dest = [dest1, dest2];
                dest.sort();
                let next_flow = self.bfs2(step + 1, new_flow, dest, visited.clone(), memo);
                if next_flow > best_flow {
                    best_flow = next_flow;
                }
            }
        }
        memo.insert(search_state, best_flow + flow);
        best_flow + flow
    }
}

pub struct Part1;

impl Part for Part1 {
    type D = Day16;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(1651));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let mut g = Graph::default();
        input.for_each(|line| g.add_line(&line));
        let mut g2 = g.clone();
        g2.reduce();
        dbg!(&g2);
        // dbg!(&g);
        let mut memo = FxHashMap::default();
        let (total, mut path) = g2.bfs(0, 0, StateId(0), Cow::Owned(Vec::new()), &mut memo);
        path.reverse();
        eprintln!("path: {path:?}");
        Ok(Num(total))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day16;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(1707));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let mut g = Graph::default();
        input.for_each(|line| g.add_line(&line));
        // dbg!(&g);
        let mut memo = FxHashMap::default();
        let total = g.bfs2(
            0,
            0,
            [StateId(0), StateId(0)],
            Cow::Owned(Vec::new()),
            &mut memo,
        );
        Ok(Num(total))
    }
}
