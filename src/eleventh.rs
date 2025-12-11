use std::collections::{HashMap, HashSet};

use crate::utils::Exercise;

pub struct Servers {
    data: HashMap<String, usize>,
    adj_list: Vec<Vec<usize>>,
}

impl Servers {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            adj_list: Vec::new(),
        }
    }

    pub fn fill(&mut self, data: &str) {
        // process
        let mut s_data = HashMap::new();
        for line in data.lines() {
            let (s, cons) = line.trim().split_once(": ").unwrap();
            let cons: Vec<&str> = cons.split_whitespace().collect();
            s_data.insert(s, cons);
        }

        // fill data with serv name -> ids
        for k in s_data.keys() {
            let id = self.data.len();
            self.data.insert(k.to_string(), id);
        }
        self.data.insert("out".to_string(), self.data.len());

        // build adjacency list
        self.adj_list.resize(self.data.len(), Vec::new());
        for (k, cons) in s_data {
            let from = self.data[k];
            for c in cons {
                self.adj_list[from].push(self.data[c]);
            }
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.adj_list.clear();
    }

    pub fn paths(
        &self,
        node: usize,
        mask: u64,
        bm_map: &HashMap<usize, u64>,
        mem: &mut HashMap<(usize, u64), u64>,
    ) -> u64 {
        // try to find in memory
        if let Some(&val) = mem.get(&(node, mask)) {
            return val;
        }

        // update mask if required
        let mut new_mask = mask;
        if let Some(&bit) = bm_map.get(&node) {
            new_mask |= 1 << bit;
        }

        // exit if 'out' id found
        if node == self.data["out"] {
            let needed = (1 << bm_map.len()) - 1;
            // 1 if mask contains required servers.
            return (new_mask == needed) as u64;
        }

        // recursive case
        let mut result = 0;
        for &n in &self.adj_list[node] {
            result += self.paths(n, new_mask, bm_map, mem);
        }

        mem.insert((node, mask), result);
        result
    }

    pub fn total_paths(&self, from: &str, must: &[&str]) -> u64 {
        // map must to bit mask
        let mut bm_map = HashMap::new();
        for (i, serv) in must.iter().enumerate() {
            bm_map.insert(self.data[*serv], i as u64);
        }

        let start = self.data[from];

        let mut mem = HashMap::new();
        self.paths(start, 0, &bm_map, &mut mem)
    }
}

impl Exercise for Servers {
    fn day(&self) -> u8 {
        11
    }

    fn part1(&mut self, data: &str) -> String {
        self.clear();
        self.fill(data);
        self.total_paths("you", &[]).to_string()
    }

    fn part2(&mut self, data: &str) -> String {
        self.clear();
        self.fill(data);
        self.total_paths("svr", &["dac", "fft"]).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_data;

    use super::*;

    #[test]
    fn tenth_test_one() {
        let data = read_data(11, "test1").unwrap();
        let mut servers = Servers::new();
        let result = servers.part1(&data);
        assert_eq!(result, "5");
    }

    #[test]
    fn tenth_test_two() {
        let data = read_data(11, "test2").unwrap();
        let mut servers = Servers::new();
        let result = servers.part2(&data);
        assert_eq!(result, "2");
    }
}
