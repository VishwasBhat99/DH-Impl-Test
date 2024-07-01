#[derive(Debug)]
pub struct Split {
    pub st_pos: usize,
    pub length: usize,
}
fn new_split_values(pos: usize, l: usize) -> Split {
    Split {
        st_pos: pos,
        length: l,
    }
}

pub fn get_split_pos() -> Vec<Split> {
    let mut split_values: Vec<Split> = Vec::new();
    split_values.push(new_split_values(0, 20));
    split_values.push(new_split_values(20, 14));
    split_values.push(new_split_values(59, 4));
    split_values.push(new_split_values(79, 16));
    split_values.push(new_split_values(95, 11));
    split_values.push(new_split_values(129, 7));
    split_values.push(new_split_values(136, 7));
    split_values.push(new_split_values(143, 11));
    split_values.push(new_split_values(154, 11));
    split_values.push(new_split_values(165, 11));
    split_values.push(new_split_values(176, 3));
    split_values.push(new_split_values(179, 10));
    split_values.push(new_split_values(189, 16));
    split_values.push(new_split_values(243, 11));
    split_values.push(new_split_values(243, 11));
    split_values.push(new_split_values(287, 41));
    split_values.push(new_split_values(328, 9));
    split_values.push(new_split_values(339, 40));
    split_values.push(new_split_values(55, 4));
    split_values.push(new_split_values(128, 1));
    split_values.push(new_split_values(269, 16));
    split_values.push(new_split_values(386, 3));
    split_values.push(new_split_values(338, 1));
    split_values.push(new_split_values(402, 4));
    split_values
}
