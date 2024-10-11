pub struct DbHelper;

impl DbHelper {
    pub fn as_db_string(value: &str) -> String {
        format!("\"{}\"", value)
    }

    pub fn id_to_thing(table_name: &str, id: &str) -> String {
        format!(
            "type::thing({},{})",
            DbHelper::as_db_string(table_name),
            DbHelper::as_db_string(id)
        )
    }
}
