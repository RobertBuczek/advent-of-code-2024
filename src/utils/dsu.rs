pub struct DSU {
    n: usize,
    parent_or_size: Vec<isize>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        DSU {
            n,
            parent_or_size: vec![-1; n],
        }
    }

    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        let mut x = self.leader(a);
        let mut y = self.leader(b);

        if x == y {
            return x;
        }

        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }

        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as isize;
        x
    }

    fn leader(&mut self, a: usize) -> usize {
        if self.parent_or_size[a] < 0 {
            a
        } else {
            let leader = self.leader(self.parent_or_size[a] as usize);
            self.parent_or_size[a] = leader as isize;
            leader
        }
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leader_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];

        for i in 0..self.n {
            leader_buf[i] = self.leader(i);
            group_size[leader_buf[i]] += 1;
        }

        let mut result = vec![vec![]; self.n];
        for i in 0..self.n {
            result[leader_buf[i]].push(i);
        }

        result.into_iter().filter(|x| !x.is_empty()).collect()
    }
}
