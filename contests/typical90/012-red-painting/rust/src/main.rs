use proconio::input;
use std::collections::HashSet;

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return false;
        }

        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}

fn main() {
    input! {
        in_h: usize,
        in_w: usize,
        in_q: usize,
    }

    let mut uf = UnionFind::new(in_h * in_w);

    // index is `r * in_w + c`
    let mut edges = HashSet::<(usize, usize)>::new();

    let mut grid_color = vec![vec![0; in_w]; in_h];

    for _ in 0..in_q {
        input! {
            in_q_label: usize,
        }

        match in_q_label {
            1 => {
                input! {
                    q_r: usize,
                    q_c: usize,
                }
                let (q_r, q_c) = (q_r - 1, q_c - 1);

                grid_color[q_r][q_c] = 1;
                // up, left, down, right
                let r_offsets = [-1, 0, 1, 0];
                let c_offsets = [0, -1, 0, 1];

                for i in 0..r_offsets.len() {
                    let r = q_r as i32 + r_offsets[i];
                    let c = q_c as i32 + c_offsets[i];
                    if r < 0 || c < 0 || r >= in_h as i32 || c >= in_w as i32 {
                        continue;
                    }
                    let r = r as usize;
                    let c = c as usize;

                    if grid_color[r][c] != 1 {
                        continue;
                    }

                    edges.insert((q_r * in_w + q_c, r * in_w + c));
                }
            }
            2 => {
                input! {
                    q_ra: usize,
                    q_ca: usize,
                    q_rb: usize,
                    q_cb: usize,
                }
                let (q_ra, q_ca, q_rb, q_cb) = (q_ra - 1, q_ca - 1, q_rb - 1, q_cb - 1);

                // check if both of the cells are painted
                if grid_color[q_ra][q_ca] != 1 || grid_color[q_rb][q_cb] != 1 {
                    println!("No");
                    continue;
                }

                // update Union-Find
                for edge in edges.iter() {
                    uf.unite(edge.0, edge.1);
                }
                // edges.clear();
                edges = HashSet::<(usize, usize)>::new();

                // get an answer
                if uf.issame(q_ra * in_w + q_ca, q_rb * in_w + q_cb) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => {
                panic!("error");
            }
        }
    }
}
