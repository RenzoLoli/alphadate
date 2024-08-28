use serde::{de::DeserializeOwned, Serialize};

use crate::{
    database::{Connection, DbHelper, QueryBuilder},
    domain::Entity,
};

pub trait BaseRepository<T>
where
    T: Clone + DeserializeOwned + Serialize + Entity,
{
    async fn query_search(&self, query: String) -> Vec<T> {
        let db = self.get_connection().db();

        match db
            .query(query)
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

    async fn get_all(&self) -> Vec<T> {
        let table_name = T::get_table_name();
        let db = self.get_connection().db();

        let entities: Vec<T> = db.select(table_name).await.ok().unwrap_or(vec![]);

        entities.into_iter().collect()
    }

    async fn find_by_ids(&self, ids: Vec<String>) -> Vec<T> {
        let table_name = T::get_table_name();
        let parsed_ids = DbHelper::ids_to_things(table_name, ids);

        let query = QueryBuilder::new(table_name)
            .q_select()
            .q_where_in("id", parsed_ids)
            .get_query();

        self.query_search(query).await
    }

    async fn find_by_id(&self, id: &str) -> Option<T> {
        let table_name = T::get_table_name();
        let parsed_id = DbHelper::id_to_thing(table_name, id);

        let query = QueryBuilder::new(table_name)
            .q_select()
            .q_where_eq("id", &parsed_id)
            .get_query();

        self.query_search(query).await.pop()
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
