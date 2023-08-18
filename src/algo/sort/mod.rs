pub mod bubble;
pub mod insertion;
pub mod quick;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Asc,
    Desc,
}
