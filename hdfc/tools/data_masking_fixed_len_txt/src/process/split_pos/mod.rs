#[derive(Debug)]
pub struct Split {
    pub st_pos: usize,
    pub end_pos: usize,
    pub length: usize,
}
fn new_split_values(st_pos: usize, end_pos: usize, length: usize) -> Split {
    Split {
        st_pos,
        end_pos,
        length,
    }
}

pub fn get_split_pos(position_changed: &[Vec<usize>]) -> Vec<Split> {
    let mut split_values: Vec<Split> = Vec::new();
    for values in position_changed {
        let start_pos = values[0] - 1;
        let end_pos = start_pos + values[1];
        split_values.push(new_split_values(start_pos, end_pos, values[1]));
    }
    split_values
}
