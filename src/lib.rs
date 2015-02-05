extern crate iron;

extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

use iron::prelude::*;
use iron::{typemap, BeforeMiddleware};

use std::sync::Arc;
use std::default::Default;
use postgres::{SslMode};
use r2d2_postgres::PostgresConnectionManager;

pub type PostgresPool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager>;
pub type SharedPostgresPool = Arc<PostgresPool>;

pub struct PostgresMiddleware {
  pub pool: SharedPostgresPool,
}

struct Value(SharedPostgresPool);

impl typemap::Key for PostgresMiddleware { type Value = Value; }

impl PostgresMiddleware {
  pub fn new(pg_connection_str: &str) -> PostgresMiddleware {
    let config = Default::default();
    let manager = PostgresConnectionManager::new(pg_connection_str, SslMode::None);
    let error_handler = r2d2::LoggingErrorHandler;
    let pool = Arc::new(r2d2::Pool::new(config, manager, Box::new(error_handler)).unwrap());
    PostgresMiddleware {
      pool: pool,
    }
  }

  pub fn from_pool(pool: SharedPostgresPool) -> PostgresMiddleware {
    PostgresMiddleware {pool: pool}
  }
}

impl BeforeMiddleware for PostgresMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<PostgresMiddleware>(Value(self.pool.clone()));
        Ok(())
    }
}

pub trait PostgresReqExt {
  fn db_conn(&self) -> r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>;
}

impl<'a> PostgresReqExt for Request<'a> {
  fn db_conn(&self) -> r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager> {
    let poll_value = self.extensions.get::<PostgresMiddleware>().unwrap();
    let &Value(ref poll) = poll_value;

    return poll.get().unwrap();
  }
}
