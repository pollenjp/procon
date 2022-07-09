use proconio::input;

use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("filed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

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

struct Circle {
    x: i64,
    y: i64,
    r: i64,
}

struct Point {
    x: i64,
    y: i64,
}

fn main() {
    input! {
        in_n: usize,
        in_s_x: i64,
        in_s_y: i64,
        in_t_x: i64,
        in_t_y: i64,
    }

    // dbg!(in_n, in_s_x, in_s_y, in_t_x, in_t_y);

    let in_s = Point {
        x: in_s_x,
        y: in_s_y,
    };
    let in_t = Point {
        x: in_t_x,
        y: in_t_y,
    };

    let mut circles = Vec::<Circle>::with_capacity(in_n);

    for i in 0..in_n {
        input! {
            in_x: i64,
            in_y: i64,
            in_r: i64,
        }
        circles.push(Circle {
            x: in_x,
            y: in_y,
            r: in_r,
        });
    }

    // s, t の存在する円を特定

    let mut s_circle_index = 0;
    let mut t_circle_index = 0;

    for it in circles.iter().enumerate() {
        let (i, circle) = it;
        if (in_s.x - circle.x).pow(2) + (in_s.y - circle.y).pow(2) == circle.r.pow(2) {
            s_circle_index = i;
        }
        if (in_t.x - circle.x).pow(2) + (in_t.y - circle.y).pow(2) == circle.r.pow(2) {
            {
                t_circle_index = i;
            }
        }
    }

    // dbg!(s_circle_index, t_circle_index);

    // 円の接続関係から edges を求める
    let mut edges = Vec::<(usize, usize)>::with_capacity(in_n);
    for i in 0..circles.len() - 1 {
        for j in i + 1..circles.len() {
            let c1 = &circles[i];
            let c2 = &circles[j];
            let center_distance_pow = (c1.x - c2.x).pow(2) + (c1.y - c2.y).pow(2);

            if (c1.r - c2.r).pow(2) <= center_distance_pow
                && center_distance_pow <= (c1.r + c2.r).pow(2)
            {
                edges.push((i, j));
            }
        }
    }
    // Union-Find で s, t を含む円が同一根であるかを判定
    let mut uf = UnionFind::new(circles.len());
    for edge in edges {
        uf.unite(edge.0, edge.1);
    }

    if uf.issame(s_circle_index, t_circle_index) {
        println!("Yes");
    } else {
        println!("No");
    }
}
