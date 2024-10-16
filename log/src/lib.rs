use connection::get_conn;
use mysql::prelude::*;
use mysql::*;
use salvo::prelude::*;

#[handler]
pub async fn add_log(req: &mut Request) -> String {
    let mission_id = req
        .query::<String>("mission_id")
        .unwrap_or("NULL!!".to_string());
    if mission_id == "NULL!!" {
        return "mission_id not provided".to_string();
    }

    let log_date = req
        .query::<String>("log_date")
        .unwrap_or("NULL!!".to_string());
    if log_date == "NULL!!" {
        return "log_date not provided".to_string();
    }

    let log_messege = req
        .query::<String>("log_messege")
        .unwrap_or("NULL!!".to_string());
    if log_messege == "NULL!!" {
        return "log_messege not provided".to_string();
    }

    let mut conn = get_conn();
    let query = "INSERT INTO log_table (`mission_id`, `log_messege`, `log_date`) VALUES (:mission_id,:log_messege,:log_date)";
    match conn.exec_drop(
        query,
        params! {
            "mission_id" => mission_id,
            "log_messege" => log_messege,
            "log_date" => log_date,
        },
    ) {
        Ok(_) => "log insert successful".to_string(),
        Err(_) => "log insert failed".to_string(),
    }
}

#[handler]
pub async fn list_all_logs(req: &mut Request) -> String {
    format!(
        "[{}]",
        get_all_logs(req)
            .iter()
            .map(|log| format!("[\"{}\"],", log.join("\",\"")))
            .fold("".to_string(), |acc, log| format!("{}{}", acc, log))
    )
}

fn get_all_logs(req: &mut Request) -> Vec<Vec<String>> {
    let mut conn = get_conn();
    let query = "SELECT * FROM log_table";
    let logs = conn
        .query_map(query, |n: Row| {
            vec![
                n.get("log_id").unwrap(),
                n.get("mission_id").unwrap(),
                n.get("log_messege").unwrap(),
                n.get("log_date").unwrap(),
            ]
        })
        .unwrap();
    if let Some(mission_id) = req.query::<String>("mission_id") {
        logs.iter()
            .filter(|log| log[1] == mission_id.to_string())
            .map(|log| log.clone())
            .collect()
    } else {
        logs
    }
}
