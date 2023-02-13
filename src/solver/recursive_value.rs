mod game_result;
mod game_result_with_remoteness;

pub use game_result::GameResult;

pub trait RecursiveValue: Clone {
    fn recursion_step(children: &[Self]) -> Self;
}

pub trait ToRecursiveValue<RV>
where
    RV: RecursiveValue,
{
    fn to_recursive_value(&self) -> Option<RV>;
}
