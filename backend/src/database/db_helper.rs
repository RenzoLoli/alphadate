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

    pub fn as_db_string_array(values: Vec<String>) -> String {
        if values.is_empty() {
            return String::from("[]");
        }

        let mut result = String::from("[");

        for value in values {
            result.push_str(DbHelper::as_db_string(&value).as_str());
            result.push(',');
        }

        result.pop();

        result.push(']');

        result
    }
}
