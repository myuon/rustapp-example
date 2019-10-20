pub use debil_derive::*;

pub trait SQLTable {
    type Type;
    fn table_name(&self) -> &'static str;

    fn map_to_sql<V: SQLValue<Self::Type>>(self) -> Vec<(String, V)>;
    fn map_from_sql<V: SQLValue<Self::Type>>(_: Vec<V>) -> Self;
}

pub trait SQLValue<Type> {
    fn serialize(self) -> Type;
    fn deserialize(_: Type) -> Self;
}
