mod args;
mod db;

use args::TodoArgs;
use clap::Parser;
fn main() {
    let conn = db::connect();
    let db = conn.get_db();

    println!("{:?}", db);
    // let args = TodoArgs::parse();
    // args.handle(conn.db);
}
