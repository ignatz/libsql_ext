use libsql::{Builder, Connection};

#[tokio::main]
async fn main() {
    let db = Builder::new_local("test.db").build().await.unwrap();
    let conn: Connection = db.connect().unwrap();

    conn.load_extension_enable().unwrap();
    conn.load_extension("./uuid", None).unwrap();
    conn.load_extension_disable().unwrap();

    println!("done");
}
