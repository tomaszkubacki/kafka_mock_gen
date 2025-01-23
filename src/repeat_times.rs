#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RepeatTimes {
    Times(usize),
    Infinite,
}
