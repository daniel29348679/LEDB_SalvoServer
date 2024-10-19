use connection::get_conn;
use mysql::prelude::*;
use mysql::*;
use salvo::prelude::*;
use worker::get_all_worker_names;

#[handler]
pub async fn add_mission(req: &mut Request) -> String {
    let mission_name = req
        .query::<String>("mission_name")
        .unwrap_or("NULL!!".to_string());
    if mission_name == "NULL!!" {
        return "Err mission_name not provided".to_string();
    }

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
    let query = "INSERT INTO mission_table (`mission_name`, `worker_name`, `state`) VALUES (:mission_name,:worker_name,:state)";
    match conn.exec_drop(
        query,
        params! { "mission_name" => mission_name,
        "worker_name" => worker_name,
        "state" => "nonstart" },
    ) {
        Ok(_) => "Ok".to_string(),
        Err(_) => "Err".to_string(),
    }
}

#[handler]
pub async fn remove_mission(req: &mut Request) -> String {
    let mission_id = req
        .query::<String>("mission_id")
        .unwrap_or("NULL!!".to_string());
    if mission_id == "NULL!!" {
        return "Err mission_id not provided".to_string();
    }
    if !get_all_mission_id().contains(&mission_id) {
        return "Err mission_id not found".to_string();
    }

    let mut conn = get_conn();
    let query = "DELETE FROM mission_table WHERE  (`id` =:mission_id)";
    match conn.exec_drop(query, params! { "mission_id" => mission_id }) {
        Ok(_) => "Ok".to_string(),
        Err(_) => "Err".to_string(),
    }
}

#[handler]
pub async fn list_all_mission(req: &mut Request) -> String {
    format!(
        "[{}]",
        get_all_missions(req)
            .iter()
            .map(|mission| format!("[\"{}\"],", mission.join("\",\"")))
            .fold("".to_string(), |acc, mission| format!("{}{}", acc, mission))
    )
}

fn get_all_missions(req: &mut Request) -> Vec<Vec<String>> {
    let mut conn = get_conn();
    let query = "SELECT * FROM mission_table";

    let missions = conn
        .query_map(query, |n: Row| {
            vec![
                n.get("id").unwrap(),
                n.get("mission_name").unwrap(),
                n.get("worker_name").unwrap(),
                n.get("state").unwrap(),
            ]
        })
        .unwrap();

    if let Some(worker_name) = req.query::<String>("worker_name") {
        missions
            .iter()
            .filter(|mission| mission[2] == worker_name)
            .map(|mission| mission.clone())
            .collect()
    } else if let Some(mission_name) = req.query::<String>("mission_name") {
        missions
            .iter()
            .filter(|mission| mission[1] == mission_name)
            .map(|mission| mission.clone())
            .collect()
    } else if let Some(mission_state) = req.query::<String>("mission_state") {
        missions
            .iter()
            .filter(|mission| mission[3] == mission_state)
            .map(|mission| mission.clone())
            .collect()
    } else if let Some(mission_id) = req.query::<String>("mission_id") {
        missions
            .iter()
            .filter(|mission| mission[0] == mission_id.to_string())
            .map(|mission| mission.clone())
            .collect()
    } else {
        missions
    }
}

pub fn get_all_mission_id() -> Vec<String> {
    let mut conn = get_conn();
    let query = "SELECT `id` FROM mission_table";
    conn.query_iter(query)
        .unwrap()
        .map(|row| {
            let row = row.unwrap();
            row.get("id").unwrap_or("".to_string())
        })
        .collect()
}

#[handler]
pub async fn update_mission_state(req: &mut Request) -> String {
    let allow_state = vec!["nonstart", "ongoing", "done"];

    let mission_id = req
        .query::<String>("mission_id")
        .unwrap_or("NULL!!".to_string());
    if mission_id == "NULL!!" {
        return "Err mission_id not provided".to_string();
    }

    let mission_state = req
        .query::<String>("mission_state")
        .unwrap_or("NULL!!".to_string());
    if mission_state == "NULL!!" {
        return "Err mission_state not provided".to_string();
    }
    if !allow_state.contains(&mission_state.as_str()) {
        return "Err mission_state not allowed".to_string();
    }

    let mut conn = get_conn();
    let query = "UPDATE mission_table SET state = :state WHERE id = :id";
    match conn.exec_drop(
        query,
        params! {
            "state" => mission_state,
            "id" => mission_id,
        },
    ) {
        Ok(_) => "Ok".to_string(),
        Err(_) => "Err".to_string(),
    }
}
