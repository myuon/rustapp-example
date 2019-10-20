use debil::*;

type SQLType = Vec<u8>;

#[derive(Record)]
#[sql(table_name = "ex_1", sql_type = "SQLType")]
struct Ex1 {
    #[sql(size = 50)]
    field1: String,
    aaaa: i32,
}

#[test]
fn it_derives_sql_table() {
    let ex1 = Ex1 {
        field1: "aaa".to_string(),
        aaaa: 10,
    };

    assert_eq!(ex1.table_name(), "ex_1");
}
