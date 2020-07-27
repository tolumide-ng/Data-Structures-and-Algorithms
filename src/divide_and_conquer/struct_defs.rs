#[derive(Debug, Clone, Copy)]
pub struct Pair {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub enum SortBy {
    X,
    Y,
}

pub type T = Vec<Pair>;
