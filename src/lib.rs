use r2d2::{PooledConnection, Pool};
use r2d2_postgres::{postgres::{NoTls, Row}, PostgresConnectionManager};
use tokio_postgres::error::Error;
use std::fmt;

pub struct ConnectionInfo {
    host: String,
    password: String,
    database: String,
    user: String,
}

impl fmt::Display for ConnectionInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "host={} user={} password={} dbname={}", self.host, self.user, self.password, self.database)
    }
}

pub type ConnectionManager = PostgresConnectionManager<NoTls>;
pub type ConnectionPool = Pool<ConnectionManager>;
pub type ConnectionPooled = PooledConnection<ConnectionManager>;

pub fn execute(mut db: ConnectionPooled, query: String) -> Result<Vec<Row>, Error> {
    db.query(query.as_str(), &[])
}

pub fn process_rows<'a, T: From<&'a Row>>(res: &'a Vec<Row>) -> Vec<T> {
    res.iter()
        .map(<T as From<&Row>>::from)
        .collect::<Vec<T>>()
}

pub fn create_manager(conInfo: ConnectionInfo) -> ConnectionManager {
    PostgresConnectionManager::new(
        conInfo.to_string().parse().unwrap(),
        NoTls,
    )
}

pub fn create_pool(conInfo: ConnectionInfo) -> ConnectionPool {
    r2d2::Pool::new(create_manager(conInfo)).unwrap()
}
