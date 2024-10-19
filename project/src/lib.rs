use connection::get_conn;
use mysql::prelude::*;
use mysql::*;
use salvo::prelude::*;

#[handler]
pub async fn add_project(req: &mut Request) -> String {
    let project_id = req.query::<String>("案號").unwrap_or("NULL!!".to_string());
    if project_id == "NULL!!" {
        return "Err 案號 not provided".to_string();
    }

    let project_name = req.query::<String>("案名").unwrap_or("NULL!!".to_string());
    if project_name == "NULL!!" {
        return "Err 案名 not provided".to_string();
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
    let mission_field = vec![
        "案號",
        "案名",
        "說明",
        "Delivery pecentage",
        "客戶",
        "通路",
        "開案日期",
        "需求回覆日期",
        "負責PM",
        "開案人員",
        "Agent",
        "Kernel",
        "Algorithm",
        "software",
        "module",
        "loading",
        "Project_status",
        "Project note",
        "hardware-computer",
        "hardware-TX type",
        "hardware-RX type",
        "SI partner",
        "Phase0-開案開始",
        "Phase0-確認規格",
        "RFP time(day)",
        "Phase1-需求評估開始",
        "Phase1-評估結束時間",
        "PoC time(day)",
        "Phase2-方案評估開始",
        "Phase2-方案評估結束",
        "SOW time(day)",
        "Phase3-商務報價評估(Quotation)",
        "Phase3-商務報價提供(Quotation)",
        "Business time(day)",
        "Phase4-工程開始(Get PO)",
        "Phase4-工程結束(Acceptance)",
        "Engineering time(day)",
        "Phase0 document (RFQ simple)",
        "Phase1 document (PoC report)",
        "Phase2 document (SoW/RFQ complete)",
        "Phase3 document (Quotation)",
        "Phase4 document (PO)",
        "Total amount(未稅)",
        "Deposit status",
        "Deposit pecentage",
        "Deposit amount",
        "time1",
        "Delivery status",
        "Delivery amount",
        "time2",
        "Acceptance status",
        "Acceptance pecentage",
        "Acceptance amount",
        "time3",
        "交付類型",
    ];

    let project_id = req.query::<String>("案號").unwrap_or("NULL!!".to_string());
    if project_id == "NULL!!" {
        return "Err 案號 not provided".to_string();
    }
    if !get_all_project_id().contains(&project_id) {
        return "Err 案號 not exist".to_string();
    }

    let field = req.query::<String>("field").unwrap_or("NULL!!".to_string());
    if field == "NULL!!" {
        return "Err field not provided".to_string();
    }
    if !mission_field.contains(&field.as_str()) {
        return "Err field not exist".to_string();
    }

    let value = req.query::<String>("value").unwrap_or("NULL!!".to_string());
    if value == "NULL!!" {
        return "Err value not provided".to_string();
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

fn get_all_project_id() -> Vec<String> {
    let mut conn = get_conn();
    let query = "SELECT `案號` FROM project_table";
    conn.query_iter(query)
        .unwrap()
        .map(|row| {
            let row = row.unwrap();
            row.get("案號").unwrap_or("".to_string())
        })
        .collect()
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
        return "Err 案號 not provided".to_string();
    }
    if !get_all_project_id().contains(&project_id) {
        return "Err 案號 not exist".to_string();
    }

    let mut conn = get_conn();
    let query = "DELETE FROM project_table WHERE `案號` = :project_id";
    match conn.exec_drop(query, params! { "project_id" => project_id }) {
        Ok(_) => "OK".to_string(),
        Err(_) => "Err".to_string(),
    }
}
