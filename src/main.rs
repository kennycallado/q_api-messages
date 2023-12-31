#![allow(unused_imports)]

mod app;
mod database;

#[cfg(test)]
mod test;

#[cfg(feature = "db_diesel")]
extern crate openssl;
#[cfg(feature = "db_diesel")]
#[macro_use]
extern crate diesel;
#[cfg(feature = "db_diesel")]
#[macro_use]
extern crate diesel_migrations;
#[cfg(feature = "db_diesel")]
extern crate rocket_sync_db_pools;

#[macro_use]
extern crate rocket;

fn main() {
    app::server::main();
}
