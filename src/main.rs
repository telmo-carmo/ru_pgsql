
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;


mod models;
mod schema;

use models::{Bonus, Dept};
use schema::{bonus, dept};

// Function to establish a database connection
pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    dotenv().ok(); // Load environment variables from .env file
    env_logger::init();

    let mut conn = establish_connection();

    let results = bonus::table
        .load::<Bonus>(&mut conn)
        .expect("Error loading bonuses");
    println!("bonus get all #={}", results.len());
    for bn in results {
        println!("{}, {}, {}, {}", bn.ename, bn.job, bn.sal, bn.comm);
    }
    /*
        let mut pargs = pico_args::Arguments::from_env();
    let port = pargs.opt_value_from_str("-port").unwrap().unwrap_or(5432);
    let user= pargs.opt_value_from_str("-usr").unwrap().unwrap_or_else(|| "scott".to_string());
    let pwd= pargs.opt_value_from_str("-pwd").unwrap().unwrap_or_else(|| "tiger".to_string());
    let db= pargs.opt_value_from_str("-db").unwrap().unwrap_or_else(|| "scottdb".to_string());
    let host= pargs.opt_value_from_str("-host").unwrap().unwrap_or_else(|| "localhost".to_string());
    let remaining_args = pargs.finish();
    println!("args: {:?}",remaining_args);
    */
}
