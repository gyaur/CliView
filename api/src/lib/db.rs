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
