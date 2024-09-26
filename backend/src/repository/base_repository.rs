use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    database::{Connection, DbHelper, QueryBuilder},
    domain::Entity,
};

pub trait BaseRepository<T>
where
    T: Clone + DeserializeOwned + Serialize + Entity + Debug + 'static,
{
    async fn query_search(&self, query: String) -> Vec<T> {
        log::debug!("query_search for <{}>", T::get_table_name());

        let db = self.get_connection().db();

        match db
            .query(query)
            .await
            .and_then(|mut res| res.take::<Vec<T>>(0))
        {
            Ok(res) => {
                log::debug!("{} items found", res.len());
                res
            }
            Err(err) => {
                log::debug!("Error querying database: {}", err);
                vec![]
            }
        }
    }

    async fn get_all(&self) -> Vec<T> {
        let table_name = T::get_table_name();
        log::debug!("Getting all entities <{}>", table_name);
        let db = self.get_connection().db();

        let entities: Vec<T> = match db.select(table_name).await {
            Ok(res) => {
                log::debug!("result -> {:?}", res);
                res
            }
            Err(err) => {
                log::debug!("Error querying database: {}", err);
                vec![]
            }
        };

        entities.into_iter().collect()
    }

    async fn find_by_ids(&self, ids: Vec<String>) -> Vec<T> {
        let table_name = T::get_table_name();
        log::debug!("Getting entities by ids <{}>", table_name);
        log::debug!("ids -> {:?}", ids.len());
        let parsed_ids = DbHelper::ids_to_things(table_name, ids);

        let query = QueryBuilder::new(table_name)
            .q_select()
            .q_where_in("id", parsed_ids)
            .get_query();

        self.query_search(query).await
    }

    async fn find_by_id(&self, id: &str) -> Option<T> {
        let table_name = T::get_table_name();
        log::debug!("Getting entity by id <{}>", table_name);
        log::debug!("id: {:?}", id);
        let parsed_id = DbHelper::id_to_thing(table_name, id);

        let query = QueryBuilder::new(table_name)
            .q_select()
            .q_where_eq("id", &parsed_id)
            .get_query();

        self.query_search(query).await.pop()
    }

    async fn create(&self, entity: T) -> Option<T> {
        let table_name = T::get_table_name();
        log::debug!("Creating entity <{}>", table_name);
        log::debug!("entity: {:#?}", entity);
        let db = self.get_connection().db();

        let mut entity = entity.clone();
        entity.generate_id();

        match db.create(table_name).content(entity).await {
            Ok(res) => {
                log::debug!("result -> {:#?}", res);
                res
            }
            Err(err) => {
                log::debug!("Error creating entity: {}", err);
                None
            }
        }
    }
    async fn update(&self, entity: T) -> Option<T> {
        let table_name = T::get_table_name();
        log::debug!("Updating entity <{}>", table_name);
        log::debug!("entity: {:#?}", entity);
        let db = self.get_connection().db();

        let id = entity.get_id().to_string();

        match db.update((table_name, id)).merge(entity).await {
            Ok(res) => {
                log::debug!("result -> {:#?}", res);
                res
            }
            Err(err) => {
                log::debug!("Error updating entity: {}", err);
                None
            }
        }
    }
    async fn delete(&self, id: &str) -> Option<T> {
        let table_name = T::get_table_name();
        log::debug!("Deleting entity <{}>", table_name);
        log::debug!("id: {:?}", id);
        let db = self.get_connection().db();

        match db.delete((table_name, id)).await {
            Ok(res) => {
                log::debug!("result -> {:#?}", res);
                res
            }
            Err(err) => {
                log::debug!("{}", err.to_string());
                None
            }
        }
    }

    fn get_connection(&self) -> &Connection;
}
