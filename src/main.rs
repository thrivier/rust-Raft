extern crate leveldb;

mod db;
mod state;

use state::State;
use db::Db;

fn main() {
    let mut test_state = State::new();
    println!("create state : test commit 1 = {}", test_state.commit(1));
    let path = "/tmp/test";
    {
        let db = Db::new(&path, true);
        match db {
            Ok(_) => {
                println!("Database succefully created");
            }, Err(e) => {
                println!("Err creating Database err: {}", e);
            }
        }
    }
    Db::destroy(::std::path::Path::new("/tmp/test")).unwrap();
}
