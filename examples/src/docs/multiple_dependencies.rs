use depends::{derives::Operation, error::EarlyExit, DepRef2, UpdateDerived};

use crate::docs::simple_value::SomeNumber;

#[derive(Operation)]
pub struct Multiply;

impl UpdateDerived<DepRef2<'_, SomeNumber, SomeNumber>, Multiply> for SomeNumber {
    fn update(&mut self, deps: DepRef2<'_, SomeNumber, SomeNumber>) -> Result<(), EarlyExit> {
        self.value = deps.0.value * deps.1.value;
        Ok(())
    }
}
