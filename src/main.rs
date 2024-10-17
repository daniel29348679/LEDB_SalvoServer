#![allow(warnings)]
use connection::get_conn;
use log::*;
use mission::*;
use mysql::prelude::*;
use mysql::*;
use project::*;
use salvo::prelude::*;
use serde::Deserialize;
use worker::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let router = Router::new()
        .push(Router::with_path("").get(root))
        .push(Router::with_path("list_all_worker_name").post(list_all_worker_name))
        .push(Router::with_path("add_worker_name").post(add_worker_name))
        .push(Router::with_path("remove_worker_name").post(remove_worker_name))
        .push(Router::with_path("add_mission").post(add_mission))
        .push(Router::with_path("remove_mission").post(remove_mission))
        .push(Router::with_path("list_all_mission").post(list_all_mission))
        .push(Router::with_path("update_mission_state").post(update_mission_state))
        .push(Router::with_path("add_log").post(add_log))
        .push(Router::with_path("list_all_logs").post(list_all_logs))
        .push(Router::with_path("add_project").post(add_project))
        .push(Router::with_path("update_project_state").post(update_project_state))
        .push(Router::with_path("list_all_project").post(list_all_project))
        .push(Router::with_path("remove_project").post(remove_project));

    let acceptor = TcpListener::new("0.0.0.0:3000").bind().await;
    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn root() -> String {
    "server running".to_string()
}
