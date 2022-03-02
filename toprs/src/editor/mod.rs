use crate::component::Component;

pub mod container;
pub mod generic;
pub mod primitive;

pub trait Editor: Default {
    type Read;
    type Write;
    type Error;

    fn ui(&self) -> Component;

    fn read_value(&self) -> Self::Read;

    fn write_value(&mut self, value: Self::Write) -> Result<(), Self::Error>;
}
