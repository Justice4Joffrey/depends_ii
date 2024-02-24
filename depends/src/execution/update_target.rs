use crate::{
    error::EarlyExit,
    execution::{Clean, HashValue, Named},
};

pub trait UpdateTarget<T>: Named + HashValue + Clean {
    fn update_mut(&mut self, value: T) -> Result<(), EarlyExit>;
}
