#![warn(clippy::pedantic, clippy::nursery)]
use std::collections::BTreeSet;
use std::collections::HashMap;

struct Submission {
    id: usize,
    s: String,
    time: u32,
}

impl Submission {
    fn to_string(&self) -> String {
        return format!("id:{} / s:{} / time:{}", self.id, self.s, self.time);
    }

    fn Debug(&self) -> String {
        return format!("id:{} / s:{} / time:{}", self.id, self.s, self.time);
    }
}

impl std::fmt::Debug for Submission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.Debug())
    }
}

impl std::cmp::PartialEq for Submission {
    fn eq(&self, other: &Self) -> bool {
        return self.time == other.time;
    }
}

impl std::cmp::PartialOrd for Submission {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.time.partial_cmp(&other.time);
    }

    fn lt(&self, other: &Submission) -> bool {
        return self.time < other.time;
    }

    fn gt(&self, other: &Submission) -> bool {
        return self.time > other.time;
    }
}

impl std::cmp::Eq for Submission {}

impl std::cmp::Ord for Submission {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

struct STVal {
    s: String,
    t: u32,
}

fn main() {
    proconio::input! {
        in_n: usize,
    }
    let mut in_s_t_vec: Vec<STVal> = Vec::with_capacity(in_n);
    for _ in 0..in_n {
        proconio::input! {
            in_s: String,
            in_t: u32,
        }
        in_s_t_vec.push(STVal { s: in_s, t: in_t });
    }

    let mut s_to_t_set_map: HashMap<String, BTreeSet<Submission>> = HashMap::new();

    s_to_t_set_map.insert("Hello".to_string(), BTreeSet::new());
    if let Some(x) = s_to_t_set_map.get_mut(&"Hello".to_string()) {
        x.insert(Submission {
            id: 0,
            s: "abc".to_string(),
            time: 2,
        });
    }
    dbg!(&s_to_t_set_map[&"Hello".to_string()]);
}
