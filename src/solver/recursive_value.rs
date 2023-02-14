mod game_result;
mod game_result_with_rmt;

pub use game_result::GameResult;
pub use game_result_with_rmt::GameResultWithRmt;
pub use game_result_with_rmt::RmtU32;

pub trait RecursiveValue: Clone {
    fn recursion_step(children: &[Self]) -> Self;
}

pub trait ToRecursiveValue<RV>
where
    RV: RecursiveValue,
{
    fn to_recursive_value(&self) -> Option<RV>;
}
