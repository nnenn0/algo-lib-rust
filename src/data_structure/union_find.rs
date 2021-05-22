pub struct UnionFind {
    parents: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parents: (0..n).collect::<Vec<usize>>(),
            size: vec![1; n],
        }
    }
    pub fn unite(&mut self, mut x: usize, mut y: usize) {
        x = self.get_root(x);
        y = self.get_root(y);
        if x == y {
            return;
        }
        if self.size[x] > self.size[y] {
            self.parents[y] = x;
            self.size[x] += self.size[y];
        } else {
            self.parents[x] = y;
            self.size[y] += self.size[x];
        }
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.get_root(x) == self.get_root(y)
    }
    pub fn get_root(&mut self, x: usize) -> usize {
        if x != self.parents[x] {
            self.parents[x] = self.get_root(self.parents[x]);
        }
        self.parents[x]
    }
    pub fn get_size(&mut self, x: usize) -> usize {
        let root = self.get_root(x);
        self.size[root]
    }
}

#[test] 
fn union_find_test() {
    let n = 10;
    let mut uf = UnionFind::new(n);
    uf.unite(0, 1);
    uf.unite(0, 2);
    uf.unite(3, 4);
    uf.unite(5, 7);
    uf.unite(5, 6);
    uf.unite(6, 7);
    uf.unite(6, 8);
    uf.unite(7, 8);
    uf.unite(8, 9);
    assert_eq!(uf.is_same(0, 1), true);
    assert_eq!(uf.is_same(5, 9), true);
    assert_eq!(uf.is_same(1, 3), false);
    assert_eq!(uf.get_size(0), 3);
}