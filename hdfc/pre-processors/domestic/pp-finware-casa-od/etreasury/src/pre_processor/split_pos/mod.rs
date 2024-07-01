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
    split_values.push(new_split_values(0, 15));
    split_values.push(new_split_values(15, 5));
    split_values.push(new_split_values(20, 3));
    split_values.push(new_split_values(23, 3));
    split_values.push(new_split_values(26, 18));
    split_values.push(new_split_values(44, 17));
    split_values.push(new_split_values(61, 16));
    split_values.push(new_split_values(77, 16));
    split_values.push(new_split_values(93, 9));
    split_values.push(new_split_values(102, 40));
    split_values.push(new_split_values(142, 11));
    split_values.push(new_split_values(153, 1));
    split_values.push(new_split_values(154, 1));
    split_values.push(new_split_values(155, 1));
    split_values.push(new_split_values(156, 1));
    split_values.push(new_split_values(157, 10));
    split_values.push(new_split_values(167, 10));
    split_values.push(new_split_values(177, 7));
    split_values.push(new_split_values(184, 7));
    split_values.push(new_split_values(191, 7));
    split_values.push(new_split_values(198, 11));
    split_values.push(new_split_values(209, 16));
    split_values.push(new_split_values(225, 16));
    split_values.push(new_split_values(241, 16));
    split_values.push(new_split_values(257, 42));
    split_values.push(new_split_values(299, 5));
    split_values.push(new_split_values(304, 7));
    split_values.push(new_split_values(311, 7));
    split_values.push(new_split_values(318, 40));
    split_values.push(new_split_values(358, 40));
    split_values.push(new_split_values(398, 3));
    split_values
}
