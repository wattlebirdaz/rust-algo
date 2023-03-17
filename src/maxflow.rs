#[derive(Clone)]

struct Edge {
    to: usize,
    cap: usize,
    rev: usize,
}

pub struct MaxFlow {
    size: usize,
    g: Vec<Vec<Edge>>,
    used: Vec<bool>,
}

impl MaxFlow {
    pub fn new(n: usize) -> Self {
        let g = vec![vec![]; n];
        Self {
            size: n,
            g,
            used: vec![false; n],
        }
    }

    fn reset(&mut self) {
        self.used = vec![false; self.size];
    }

    pub fn add_edge(&mut self, a: usize, b: usize, c: usize) {
        let current_ga = self.g[a].len();
        let current_gb = self.g[b].len();
        self.g[a].push(Edge {
            to: b,
            cap: c,
            rev: current_gb,
        });
        self.g[b].push(Edge {
            to: a,
            cap: 0,
            rev: current_ga,
        });
    }

    fn dfs(&mut self, pos: usize, goal: usize, min_cap: usize) -> usize {
        if pos == goal {
            return min_cap;
        }

        self.used[pos] = true;

        for i in 0..self.g[pos].len() {
            if self.g[pos][i].cap == 0 {
                continue;
            }

            if self.used[self.g[pos][i].to] == true {
                continue;
            }

            let flow = self.dfs(self.g[pos][i].to, goal, min_cap.min(self.g[pos][i].cap));

            if flow >= 1 {
                self.g[pos][i].cap -= flow;
                let to = self.g[pos][i].to;
                let rev = self.g[pos][i].rev;
                self.g[to][rev].cap += flow;
                return flow;
            }
        }

        0
    }

    pub fn max_flow(&mut self, s: usize, t: usize) -> usize {
        let mut total_flow = 0;
        loop {
            self.reset();
            let f = self.dfs(s, t, std::usize::MAX / 2);
            if f == 0 {
                break;
            }

            total_flow += f;
        }
        return total_flow;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fn;
    use crate::input;

    fn test_maxflow(input: String) -> String {
        input! {
            source = input,
            n: usize,
            m: usize,
            abc: [(usize, usize, usize); m],
        };

        let mut mf = MaxFlow::new(n);

        for (a, b, c) in abc {
            mf.add_edge(a, b, c);
        }

        let ans = mf.max_flow(0, n - 1);

        format!("{}", ans)
    }

    fn test_one(id: usize) {
        let input_file = format!("./assets/GRL_6_A/{}/in", id);
        let output_file = format!("./assets/GRL_6_A/{}/out", id);
        test_fn(&input_file, &output_file, test_maxflow);
    }

    #[test]
    fn test_all() {
        for i in 1..=39 {
            test_one(i);
        }
    }
}
