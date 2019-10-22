use crate::SQLValue;

impl SQLValue<mysql_async::Value> for i32 {
    fn column_type(_: std::marker::PhantomData<Self>, _: i32) -> String {
        "int".to_string()
    }

    fn serialize(self) -> mysql_async::Value {
        From::from(self)
    }
    fn deserialize(val: mysql_async::Value) -> Self {
        mysql_async::from_value(val)
    }
}

impl SQLValue<mysql_async::Value> for String {
    fn column_type(_: std::marker::PhantomData<Self>, size: i32) -> String {
        format!("varchar({})", size)
    }

    fn serialize(self) -> mysql_async::Value {
        From::from(self)
    }
    fn deserialize(val: mysql_async::Value) -> Self {
        mysql_async::from_value(val)
    }
}
