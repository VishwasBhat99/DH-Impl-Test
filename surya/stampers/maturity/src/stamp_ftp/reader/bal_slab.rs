use super::*;

#[derive(Debug, Default)]
pub struct BalanceSlab {
    pub left: f64,
    pub right: f64,
    pub rate: f64,
}

#[derive(Debug, Default)]
pub struct BalanceSlabs {
    pub slabs: Vec<BalanceSlab>,
}

impl BalanceSlabs {
    pub fn new(file_path: &str) -> Self {
        let mut bal_slabs: Self = Default::default();
        let reader = read_file(file_path);
        for (line_num, lines) in reader.lines().enumerate() {
            let line = extract_lines(line_num, lines, file_path);
            let fields: Vec<&str> = line.split('|').collect();
            if fields.len() == 3 {
                bal_slabs.slabs.push(BalanceSlab {
                    left: parse_f64(fields[0].trim()),
                    right: parse_f64(fields[1].trim()),
                    rate: parse_f64(fields[2].trim()),
                })
            }
        }
        bal_slabs
    }

    pub fn get_adj_rate(&self, bal: f64) -> f64 {
        for bal_slab in self.slabs.iter() {
            if bal_slab.left <= bal && bal < bal_slab.right {
                return bal_slab.rate;
            }
        }
        0.0
    }
}
