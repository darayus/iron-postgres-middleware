# iron-postgres-middleware

An attempt to create postgres middleware for [Iron](https://github.com/iron/iron/) web framework

## Usage

### Cargo.toml

    [dependencies.iron-postgres-middleware]
    git = "https://github.com/martinsp/iron-postgres-middleware.git"

### Import

    extern crate "iron-postgres-middleware" as pg_middleware;
    use pg_middleware::{PostgresMiddleware, PostgresReqExt};

### Using middleware

    fn main() {
      let mut router = Router::new();
      router.get("/", handler);

      let mut c = ChainBuilder::new(router);

      let pg_middleware = PostgresMiddleware::new("postgres://user@localhost/db_name");
      c.link_before(pg_middleware);

      Iron::new(c).listen(SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 3000 }).unwrap();
    }

    fn handler(req: &mut Request) -> IronResult<Response> {
      let con = req.db_conn();
      con.execute("INSERT INTO foo (bar) VALUES ($1)", &[&1i32]).unwrap();
      Ok(Response::new().set(status::Ok).set("Success"))
    }
