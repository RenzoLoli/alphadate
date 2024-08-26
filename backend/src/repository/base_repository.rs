use serde::{de::DeserializeOwned, Serialize};

use crate::{database::Connection, domain::Entity};

fn encapsulate(value: String) -> String {
    format!("\"{}\"", value)
}

fn encapsulate_ids_vec(table_name: String, ids: Vec<String>) -> Vec<String> {
    let encapsulated_table_name = encapsulate(table_name);
    ids.into_iter()
        .map(|id| {
            format!(
                "type::record({},{})",
                encapsulated_table_name.clone(),
                encapsulate(id)
            )
        })
        .collect()
}

pub trait BaseRepository<T>
where
    T: Clone + DeserializeOwned + Serialize + Entity,
{
    async fn get_all(&self) -> Vec<T> {
        let table_name = T::get_table_name();
        let db = self.get_connection().db();

        let entities: Vec<T> = db.select(table_name).await.ok().unwrap_or(vec![]);

        entities.into_iter().collect()
    }

    async fn find_by_ids(&self, ids: Vec<String>) -> Vec<T> {
        self.find_by_in("id", ids).await
    }

    async fn find_by_in(&self, key: &str, ids: Vec<String>) -> Vec<T> {
        let table_name = T::get_table_name();
        let db = self.get_connection().db();

        match db
            .query("SELECT * from $table WHERE $key IN [$ids]")
            .bind(("table", table_name))
            .bind(("key", key))
            .bind(("ids", encapsulate_ids_vec(table_name.to_owned(), ids)))
            .await
            .and_then(|mut res| res.take::<Vec<T>>(0))
        {
            Ok(res) => res,
            Err(err) => {
                log::error!("{}", err.to_string());
                vec![]
            }
        }
    }

    async fn find_by_id(&self, id: &str) -> Option<T> {
        self.find_by_where("id", id).await.pop()
    }

    async fn find_by_where(&self, field: &str, value: &str) -> Vec<T> {
        let table_name = T::get_table_name();
        let db = self.get_connection().db();

        let query = format!(
            "SELECT * FROM {} WHERE {} = {}",
            table_name,
            field,
            encapsulate(value.to_owned())
        );

        match db
            .query(query)
            .bind(("table", table_name))
            .bind(("field", field))
            .bind(("value", encapsulate(value.to_owned())))
            .await
            .and_then(|mut res| res.take::<Vec<T>>(0))
        {
            Ok(res) => res,
            Err(err) => {
                log::error!("{}", err.to_string());
                vec![]
            }
        }
    }

    async fn create(&self, entity: T) -> Option<T> {
        let table_name = T::get_table_name();
        let db = self.get_connection().db();

        let mut entity = entity.clone();
        entity.generate_id();

        db.create(table_name)
            .content(entity)
            .await
            .inspect_err(|err| log::error!("{}", err.to_string()))
            .map(|mut res| res.pop())
            .ok()?
    }
    async fn update(&self, entity: T) -> Option<T> {
        let table_name = T::get_table_name();
        let db = self.get_connection().db();

        let id = entity.get_id().to_string();

        db.update((table_name, id))
            .merge(entity)
            .await
            .inspect_err(|err| log::error!("{}", err.to_string()))
            .ok()?
    }
    async fn delete(&self, id: &str) -> Option<T> {
        let table_name = T::get_table_name();
        let db = self.get_connection().db();

        db.delete((table_name, id))
            .await
            .inspect_err(|err| log::error!("{}", err.to_string()))
            .ok()?
    }

    fn get_connection(&self) -> &Connection;
}
