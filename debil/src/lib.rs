pub use debil_derive::*;

pub struct FieldAttribute {
    pub size: Option<i32>,
    pub unique: bool,
    pub not_null: bool,
}

pub trait SQLTable {
    type Type;
    fn table_name(&self) -> &'static str;
    fn schema_of(&self) -> Vec<(String, String, FieldAttribute)>;

    fn map_to_sql<V: SQLValue<Self::Type>>(self) -> Vec<(String, V)>;
    fn map_from_sql<V: SQLValue<Self::Type>>(_: Vec<V>) -> Self;
}

pub trait SQLValue<Type> {
    fn column_type(_: std::marker::PhantomData<Self>) -> &'static str;

    fn serialize(self) -> Type;
    fn deserialize(_: Type) -> Self;
}
