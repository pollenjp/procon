use proconio::input;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

fn main() {
    input! {
        in_abcd: [(i32, i32); 4],
    }

    for i in 0..in_abcd.len() {
        let p1 = in_abcd[i % in_abcd.len()];
        let p1 = Point {
            x: p1.0 as f64,
            y: p1.1 as f64,
        };
        let p2 = in_abcd[(i + 1) % in_abcd.len()];
        let p2 = Point {
            x: p2.0 as f64,
            y: p2.1 as f64,
        };
        let p3 = in_abcd[(i + 2) % in_abcd.len()];
        let p3 = Point {
            x: p3.0 as f64,
            y: p3.1 as f64,
        };

        let v1 = p2 - p1.clone();
        let v2 = p3 - p1.clone();

        let v1cos = v1.x / v1.norm();
        let v1sin = v1.y / v1.norm();

        // 負の方向に回転
        // let x = v1cos * v2.x + v1sin * v2.y;
        let y = -v1sin * v2.x + v1cos * v2.y;

        if y <= 0.0 {
            println!("No");
            return;
        }
    }

    println!("Yes")
}
