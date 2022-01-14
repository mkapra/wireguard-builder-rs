use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

/// Represents a connection pool for the database
pub type DatabasePool = Pool<ConnectionManager<PgConnection>>;
/// Represents a connection to the database
pub type DatabaseConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Wrapper for a database pool
pub struct Database(DatabasePool);

impl Database {
    /// Creates a new `Database` with a connection pool inside
    ///
    /// # Panics
    /// Panics if the connection to the database was not successful
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::new(database_url);
        Database(Pool::new(manager).expect("Could not connect to database"))
    }

    /// Returns a new connection to the database from the pool
    ///
    /// # Panics
    /// Panics if no connection could be retrieved from the pool
    pub fn get(&self) -> DatabaseConnection {
        self.0
            .get()
            .expect("Could not get database connection from pool")
    }
}
