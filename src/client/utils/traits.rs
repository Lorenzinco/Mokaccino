pub trait Clone {
    fn clone(&self) -> Self;
}

pub trait Copy: Clone {}

