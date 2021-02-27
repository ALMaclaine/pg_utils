use r2d2::{PooledConnection};
use r2d2_postgres::{postgres::{NoTls, Row}, PostgresConnectionManager};
use tokio_postgres::error::Error;

pub type ConnectionPool = PooledConnection<PostgresConnectionManager<NoTls>>;

pub fn execute(mut db: ConnectionPool, query: String) -> Result<Vec<Row>, Error> {
    db.query(query.as_str(), &[])
}

pub fn process_rows<'a, T: From<&'a Row>>(res: &'a Vec<Row>) -> Vec<T> {
    res.iter()
        .map(<T as From<&Row>>::from)
        .collect::<Vec<T>>()
}
