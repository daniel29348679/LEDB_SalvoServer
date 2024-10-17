use connection::get_conn;
use mysql::prelude::*;
use mysql::*;
use salvo::prelude::*;

#[handler]
pub async fn add_project(req: &mut Request) -> String {
    let project_id = req.query::<String>("案號").unwrap_or("NULL!!".to_string());
    if project_id == "NULL!!" {
        return "案號 not provided".to_string();
    }

    let project_name = req.query::<String>("案名").unwrap_or("NULL!!".to_string());
    if project_name == "NULL!!" {
        return "案名 not provided".to_string();
    }

    let mut conn = get_conn();
    let query = "INSERT INTO project_table (`案號`, `案名`) VALUES (:project_id,:project_name)";
    match conn.exec_drop(
        query,
        params! { "project_id" => project_id,
        "project_name" => project_name },
    ) {
        Ok(_) => "OK".to_string(),
        Err(_) => "Err".to_string(),
    }
}

#[handler]
pub async fn update_project_state(req: &mut Request) -> String {
    let project_id = req.query::<String>("案號").unwrap_or("NULL!!".to_string());
    if project_id == "NULL!!" {
        return "案號 not provided".to_string();
    }

    let field = req.query::<String>("field").unwrap_or("NULL!!".to_string());
    if field == "NULL!!" {
        return "field not provided".to_string();
    }

    let value = req.query::<String>("value").unwrap_or("NULL!!".to_string());
    if value == "NULL!!" {
        return "value not provided".to_string();
    }

    let mut conn = get_conn();
    let query = format!(
        "UPDATE project_table SET `{}` = :value WHERE `案號` = :project_id",
        field
    );
    match conn.exec_drop(
        query,
        params! { "project_id" => project_id,
        "value" => value },
    ) {
        Ok(_) => "OK".to_string(),
        Err(_) => "Err".to_string(),
    }
}

fn get_all_project(req: &mut Request) -> Vec<Vec<String>> {
    let mut conn = get_conn();
    let query = "SELECT * FROM project_table";
    conn.query_iter(query)
        .unwrap()
        .map(|row| {
            //裡面會有超多行資料
            let row = row.unwrap();
            row.columns_ref()
                .iter()
                .map(|col| {
                    //這裡會有每一行的資料
                    row.get(col.name_str().as_ref()).unwrap_or("".to_string())
                })
                .collect()
        })
        .collect()
}

#[handler]
pub async fn list_all_project(req: &mut Request) -> String {
    format!(
        "[{}]",
        get_all_project(req)
            .iter()
            .map(|project| format!("[\"{}\"],", project.join("\",\"")))
            .fold("".to_string(), |acc, project| format!("{}{}", acc, project))
    )
}

#[handler]
pub async fn remove_project(req: &mut Request) -> String {
    let project_id = req.query::<String>("案號").unwrap_or("NULL!!".to_string());
    if project_id == "NULL!!" {
        return "案號 not provided".to_string();
    }

    let mut conn = get_conn();
    let query = "DELETE FROM project_table WHERE `案號` = :project_id";
    match conn.exec_drop(query, params! { "project_id" => project_id }) {
        Ok(_) => "OK".to_string(),
        Err(_) => "Err".to_string(),
    }
}
