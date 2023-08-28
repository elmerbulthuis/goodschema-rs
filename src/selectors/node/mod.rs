mod intermediate_a;

pub use intermediate_a::*;

pub trait NodeSelectors {
    fn select_is_empty(&self) -> bool;
}
