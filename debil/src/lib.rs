pub use debil_derive::*;

pub trait SQLTable {
    fn table_name(&self) -> &'static str;
}
