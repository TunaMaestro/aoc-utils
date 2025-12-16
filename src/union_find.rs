type T = usize;

pub struct UnionFind {
    inner: Vec<Node>,
    distinct_count: usize,
}

#[derive(Clone, Copy)]
struct Node {
    // parent idx
    parent: usize,
    rank: usize,
}

impl UnionFind {
    pub fn new(items: usize) -> UnionFind {
        UnionFind {
            inner: (0..items).map(|i| Node { parent: i, rank: 0 }).collect(),
            distinct_count: items,
        }
    }

    pub fn union(&mut self, a: T, b: T) {
        let a_set = self.find(a);
        let b_set = self.find(b);
        if a_set == b_set {
            return;
        }
        self.link(a_set, b_set);
        self.distinct_count -= 1;
    }

    pub fn find(&mut self, x: T) -> T {
        if x != self.inner[x].parent {
            self.inner[x].parent = self.find(self.inner[x].parent);
        }
        return self.inner[x].parent;
    }

    fn link(&mut self, x_idx: T, y_idx: T) {
        let mut x = self.inner[x_idx];
        let mut y = self.inner[y_idx];
        if x.rank > y.rank {
            y.parent = x_idx;
        } else {
            if x.rank == y.rank {
                y.rank += 1;
            }
            x.parent = y_idx;
        }
        self.inner[x_idx] = x;
        self.inner[y_idx] = y;
    }

    pub fn distinct_count(&self) -> usize {
        self.distinct_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_operation() {
        let mut u = UnionFind::new(10);

        assert_eq!(u.find(0), 0);
        assert_eq!(u.find(1), 1);
        assert_eq!(u.find(2), 2);
        assert_eq!(u.find(3), 3);
        assert_eq!(u.find(4), 4);
        assert_eq!(u.find(5), 5);
        assert_eq!(u.find(6), 6);

        u.union(0, 1);
        u.union(1, 2);
        u.union(3, 4);
        u.union(5, 6);

        assert_eq!(u.find(1), u.find(0));
        assert_eq!(u.find(2), u.find(0));

        assert_eq!(u.find(4), u.find(3));
        assert_eq!(u.find(6), u.find(5));

        u.union(0, 5);

        assert_eq!(u.find(0), u.find(0));
        assert_eq!(u.find(1), u.find(0));
        assert_eq!(u.find(2), u.find(0));
        assert_eq!(u.find(5), u.find(0));
        assert_eq!(u.find(6), u.find(0));

        assert_ne!(u.find(3), u.find(0));
        assert_ne!(u.find(4), u.find(0));
        assert_ne!(u.find(7), u.find(0));
        assert_ne!(u.find(8), u.find(0));

        u.union(5, 4);

        assert_eq!(u.find(0), u.find(0));
        assert_eq!(u.find(1), u.find(0));
        assert_eq!(u.find(2), u.find(0));
        assert_eq!(u.find(3), u.find(0));
        assert_eq!(u.find(4), u.find(0));
        assert_eq!(u.find(5), u.find(0));
        assert_eq!(u.find(6), u.find(0));

        assert_ne!(u.find(7), u.find(0));
        assert_ne!(u.find(8), u.find(0));
    }
}
