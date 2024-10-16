use mysql::*;
#[allow(warnings)]
pub fn get_conn() -> PooledConn {
    let url = "mysql://root:123456@localhost:3306/todo_list";
    let pool = Pool::new(url).unwrap();
    pool.get_conn().unwrap()
}
