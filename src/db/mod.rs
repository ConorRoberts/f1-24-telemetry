use libsql::{Builder, Connection, Result};

pub async fn connect_db() -> Result<Connection> {
    // dotenv().ok();

    let db = Builder::new_local("local.db").build().await?;
    let conn = db.connect();

    return conn;
}
