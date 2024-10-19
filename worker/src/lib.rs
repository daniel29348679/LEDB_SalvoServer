use connection::get_conn;
use mysql::prelude::*;
use mysql::*;
use salvo::prelude::*;

#[handler]
pub async fn list_all_worker_name() -> String {
    format!(
        "[{}]",
        get_all_worker_names()
            .iter()
            .map(|name| format!("\"{}\",", name.clone()))
            .fold("".to_string(), |acc, name| format!("{}{}", acc, name))
    )
}

pub fn get_all_worker_names() -> Vec<String> {
    let mut conn = get_conn();
    let query = "SELECT name FROM name_table";
    let names: Vec<String> = conn.query_map(query, |name| name).unwrap();
    names
}

#[handler]
pub async fn add_worker_name(req: &mut Request) -> String {
    let worker_name = req
        .query::<String>("worker_name")
        .unwrap_or("NULL!!".to_string());

    if worker_name == "NULL!!" {
        return "Err worker_name not provided".to_string();
    }

    let mut conn = get_conn();

    let query = "INSERT INTO name_table (name) VALUES (:name)";
    match conn.exec_drop(query, params! { "name" => worker_name }) {
        Ok(_) => "Ok".to_string(),
        Err(_) => "Err".to_string(),
    }
}
#[handler]
pub async fn remove_worker_name(req: &mut Request) -> String {
    let worker_name = req
        .query::<String>("worker_name")
        .unwrap_or("NULL!!".to_string());

    if worker_name == "NULL!!" {
        return "Err worker_name not provided".to_string();
    }

    if !get_all_worker_names().contains(&worker_name) {
        return "Err worker_name not exist".to_string();
    }

    let mut conn = get_conn();
    //DELETE FROM `todo_list`.`name_table` WHERE (`name` = 'Sandy');
    let query = "DELETE FROM name_table WHERE  (`name` =:name)";
    match conn.exec_drop(query, params! { "name" => worker_name }) {
        Ok(_) => "Ok".to_string(),
        Err(_) => "Err".to_string(),
    }
}
