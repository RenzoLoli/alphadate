pub struct QueryBuilder {
    table_name: String,
    body: String,
}

impl QueryBuilder {
    pub fn new(table_name: &str) -> QueryBuilder {
        QueryBuilder {
            table_name: table_name.to_owned(),
            body: String::new(),
        }
    }

    pub fn q_select(mut self) -> QueryBuilder {
        let query = format!("SELECT * FROM {}\n", self.table_name);
        self.body.push_str(query.as_str());
        self
    }

    pub fn q_where_eq(mut self, field: &str, value: &str) -> QueryBuilder {
        let query = format!("WHERE {} = {}\n", field, value);
        self.body.push_str(query.as_str());
        self
    }

    pub fn q_and_eq(mut self, field: &str, value: &str) -> QueryBuilder {
        let query = format!("AND {} = {}\n", field, value);
        self.body.push_str(query.as_str());
        self
    }

    #[allow(dead_code)]
    pub fn q_and_in(mut self, field: &str, values: Vec<String>) -> QueryBuilder {
        let query = format!("AND {} IN [{}]\n", field, values.join(","));
        self.body.push_str(query.as_str());
        self
    }

    pub fn get_query(self) -> String {
        self.body
    }
}
