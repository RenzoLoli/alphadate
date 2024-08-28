pub struct DbHelper;

impl DbHelper {
    pub fn as_db_string(value: &str) -> String {
        format!("\"{}\"", value)
    }

    pub fn ids_to_things(table_name: &str, ids: Vec<String>) -> Vec<String> {
        ids.into_iter()
            .map(|id| DbHelper::id_to_thing(table_name, id.as_str()))
            .collect::<Vec<String>>()
    }

    pub fn id_to_thing(table_name: &str, id: &str) -> String {
        format!(
            "type::thing({},{})",
            DbHelper::as_db_string(table_name),
            DbHelper::as_db_string(id)
        )
    }
}
