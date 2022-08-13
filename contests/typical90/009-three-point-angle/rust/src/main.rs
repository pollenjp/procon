use proconio::input;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    // from 0.0 to 2.0pi
    fn radian(&self) -> f64 {
        let rad = self.y.atan2(self.x);
        if rad < 0.0 {
            rad + 2.0 * std::f64::consts::PI
        } else {
            rad
        }
    }

    fn angle(&self) -> f64 {
        self.radian().to_degrees()
    }

    fn is_same(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn main() {
    input! {
        in_n: usize,
        in_x: [(usize, usize); in_n],
    }

    let points = in_x
        .iter()
        .map(|&(x, y)| Point {
            x: x as f64,
            y: y as f64,
        })
        .collect::<Vec<_>>();

    let mut ans: f64 = 0.0;

    // center point index
    for cp_idx in 0..in_n {
        // center point
        let cp = &points[cp_idx];
        // points
        let ps = points
            .iter()
            .map(|p| Point {
                x: p.x - cp.x,
                y: p.y - cp.y,
            })
            .collect::<Vec<_>>();

        let mut angles = ps
            .iter()
            .enumerate()
            .map(|(i, p)| (i, p.angle()))
            .collect::<Vec<(usize, f64)>>();
        angles.sort_by(|&a, &b| a.1.partial_cmp(&b.1).unwrap());

        // しゃくとり法
        let mut p1_idx = 0;
        let mut p2_idx = 1;
        while p2_idx < in_n {
            if p1_idx == p2_idx {
                p2_idx += 1;
                continue;
            }
            let x1 = angles[p1_idx];
            let x2 = angles[p2_idx];
            if x1.0 == cp_idx {
                p1_idx += 1;
                continue;
            }
            if x2.0 == cp_idx {
                p2_idx += 1;
                continue;
            }

            let mut angle_diff = x2.1 - x1.1;
            if angle_diff > 180.0 {
                angle_diff = 2.0 * std::f64::consts::PI.to_degrees() - angle_diff;
                p1_idx += 1;
                p2_idx -= 1;
            }

            ans = ans.max(angle_diff);
            p2_idx += 1;
        }
    }

    println!("{}", ans);
}
