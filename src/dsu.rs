use std::usize;

type Id = usize;

struct DSU {
    par:  Vec::<Id>,
    rank: Vec::<u64>,
}

impl DSU {
    fn new(size: usize) -> DSU {
        return DSU { 
            par:  (0..size).collect(), 
            rank: (0..size).map(|_| 0).collect()
        };
    }

    fn get(&mut self, x: Id) -> Id {
        let mut it = x;
        
        while self.par[it] != it { it = self.par[it] }
        
        let root = it;
        it = x;

        while self.par[it] != it { (it, self.par[it]) = (self.par[it], root) }

        return root;
    }

    fn union(&mut self, mut x: Id, mut y:Id) {
        (x, y) = (self.get(x), self.get(y));

        if x== y {return;}
        if self.rank[x] == self.rank[y] { self.rank[x] += 1 }

        if self.rank[x] < self.rank[y] {
            self.par[x] = y
        } else {
            self.par[y] = x
        }
    }
}
