mod args;
mod db;

use args::TodoArgs;
use clap::Parser;
fn main() {
    let conn = db::connect();

    let args = TodoArgs::parse();
    args.handle(&conn);
}
