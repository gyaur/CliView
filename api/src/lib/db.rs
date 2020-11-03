use dotenv::dotenv;
use rustorm::*;
use std::{collections::VecDeque, env};

use crate::{RetriveUrl, Url};

pub fn establish_connection() -> EntityManager {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut pool = Pool::new();
    pool.em(&database_url).unwrap()
}

pub fn establish_test_connection() -> EntityManager {
    dotenv().ok();

    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let mut pool = Pool::new();
    let mut em = pool.em(&database_url).unwrap();
    clear_table("url", &mut em);
    em
}

pub fn init_db(em: &mut EntityManager) {
    let create_sql = "CREATE TABLE url(
        id integer PRIMARY KEY AUTOINCREMENT,
        url text)";
    em.db().execute_sql_with_return(create_sql, &[]);
}

fn clear_table(table_name: &str, em: &mut EntityManager) {
    let sql = format!("DELETE FROM {}", table_name);
    em.db().execute_sql_with_return(&sql, &[]);
}

fn insert_values(urls: &VecDeque<Url>, em: &mut EntityManager) {
    for url in urls.iter() {
        em.insert::<Url, RetriveUrl>(&[url]);
    }
}

pub fn select_values(em: &mut EntityManager) -> Vec<Url> {
    let select_sql = "SELECT * FROM url";
    em.execute_sql_with_return(select_sql, &[]).unwrap()
}

pub fn update_db(urls: &VecDeque<Url>, em: &mut EntityManager) {
    clear_table("url", em);
    insert_values(urls, em);
}

#[cfg(test)]
mod test {
    use rustorm::EntityManager;

    use crate::{establish_test_connection, init_db, select_values, update_db, Url};

    use super::{clear_table, insert_values};

    fn init_test() -> EntityManager {
        let mut em = establish_test_connection();
        init_db(&mut em);
        clear_table("url", &mut em);
        em
    }
    #[test]
    fn test_init_db() {
        let mut em = establish_test_connection();
        init_db(&mut em);
        let tables = em.db().get_all_tables().unwrap();
        assert!(tables.len() == 2);
        let expacted_names = vec!["url", "sqlite_sequence"];
        let table_names = tables
            .iter()
            .all(|x| expacted_names.contains(&x.name.name.as_str()));
        assert!(table_names);
    }

    #[test]
    fn test_clear_table() {
        let mut em = init_test();
        init_db(&mut em);
        let values = select_values(&mut em);
        assert!(values.len() == 0);
        let values_to_insert = vec![
            Url::from(String::from("test1")),
            Url::from(String::from("test2")),
        ]
        .into_iter()
        .collect();
        insert_values(&values_to_insert, &mut em);
        let values = select_values(&mut em);
        assert!(values.len() == 2);
        clear_table("url", &mut em);
        let values = select_values(&mut em);
        assert!(values.len() == 0);
    }

    #[test]
    fn test_insert_values() {
        let mut em = init_test();
        init_db(&mut em);
        let values = select_values(&mut em);
        assert!(values.len() == 0);
        let values_to_insert = vec![
            Url::from(String::from("test1")),
            Url::from(String::from("test2")),
        ]
        .into_iter()
        .collect();
        insert_values(&values_to_insert, &mut em);
        let values = select_values(&mut em);
        assert!(values.len() == 2);
        assert!(values.iter().zip(values_to_insert).all(|(x, y)| x == &y));
    }

    #[test]
    fn test_update_db() {
        let mut em = init_test();
        init_db(&mut em);
        let values = select_values(&mut em);
        assert!(values.len() == 0);
        let values_to_insert = vec![
            Url::from(String::from("test1")),
            Url::from(String::from("test2")),
        ]
        .into_iter()
        .collect();
        insert_values(&values_to_insert, &mut em);
        let values = select_values(&mut em);
        assert!(values.len() == 2);
        let values_update = vec![
            Url::from(String::from("test3")),
            Url::from(String::from("test4")),
            Url::from(String::from("test5")),
        ]
        .into_iter()
        .collect();
        update_db(&values_update, &mut em);
        let values = select_values(&mut em);
        print!("{:?}", values.len());
        assert!(values.len() == 3);
        assert!(values.iter().zip(values_update).all(|(x, y)| x == &y));
    }
}
