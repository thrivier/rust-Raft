extern crate leveldb;
use std::path::Path;
use std::vec::Vec;
use leveldb::database::Database;
use leveldb::options::{Options,WriteOptions,ReadOptions};
use leveldb::database::management::destroy;
use leveldb::database::kv::{KV};

//TODO add test with disk failure ... 

/**
*   Struct with a leveldb instance
*   and the Database path
*/
pub struct Db<'a> {
    db: Database<i32>, 
    pub path: &'a Path
}

/**
*   Wrapper for use a database for persitant data
*/
impl <'a> Db <'a> {
    /**
    *  Constructor for create Db
    *  @params {str} path to the database
    *  @params {bool} if_missing - Boolean use for know 
    *              if need it to create the Database 
    *              in case or the database doesn't exist
    */
    pub fn new(path: &'a str, if_missing: bool) -> Result<Db, &'static str> {
        let mut options = Options::new();
        options.create_if_missing = if_missing;
        let path_db = Path::new(path);
        let db: Result<Database<i32>,_> = Database::open(path_db, options);
        match db {
            Ok(tmp) => {
                Ok( Db{db: tmp, path: &path_db} )
            },
            Err(e) => { panic!("error : {}", e)}
        }
    }

    /**
    *  Remove all the database with all the datas.
    *  @params {Path} path - Path to the database
    */
    pub fn destroy(path: &'a Path) -> Result<(), &'static str> {
        let options = Options::new();
        match destroy(path, options) {
            Ok(tmp) => {
                Ok(tmp)
            }, Err(err) => {
                println!("error : {}", err);
                Err("Error deleting database")
            }
        }
    }

    /**
    *   Associate the key with the value in the database . 
    *   Use a write sync . 
    *   @params {i32} k - Key to save in the database
    *   @params {String} v - Value to associate for the key
    */
    pub fn put_as_string(&self, k: i32, v: String) -> Result<(), &'static str> {
        let array_value : &[u8] = v.as_bytes();
        let mut options = WriteOptions::new();
        options.sync = true;
        match self.db.put(options, k, array_value) {
            Ok(o) => {
                Ok(o)
            },
            Err(_) => {
                Err("Error puting object")
            }
        }
    }

    /**
    *   Associate the key with the value in the database . 
    *   Use a write sync . 
    *   @params {i32} k - Key to save in the database
    *   @params {&[u8]} v - Value to associate for the key
    */
    pub fn put_as_bytes(&self, k: i32, v: &[u8]) -> Result<(), &'static str> {
        let mut options = WriteOptions::new();
        options.sync = true;
        match self.db.put(options, k, v) {
            Ok(o) => {
                Ok(o)
            },
            Err(_) => {
                Err("Error puting object")
            }
        }
    }

    /**
    *   Find a key in the database and return the value associate.
    *   @params {i32} k - Key to find
    */
    pub fn get(&self, k: i32) -> Result<Vec<u8>, &'static str> {
        let options = ReadOptions::new();
        match self.db.get(options, k) {
            Ok(result) => {
                match result {
                    Some(ret) => {
                        Ok(ret)
                    }, None => {
                        Err("objectnotfound")
                    }
                }
            }, Err(_) => {
                Err("fail to read database")
            }
        }
    }

    /**
    *   Delete a key im the Database.
    *   Never send error exept in case of disk failure.
    *   @params {i32} k - Key to delete in database
    */
    pub fn delete(&self, k: i32) -> Result<(), &'static str> {
        let options = WriteOptions::new();
        match self.db.delete(options, k) {
            Ok(ok) => {
                Ok(ok) 
            }, Err(_) => {
                Err("fail")
            }
       }
    }
}

#[cfg(test)]
use std::fs;

#[test]
fn test_db_new_and_destroy_1() {
    let path = "/tmp/test2";
    {
        println!("Should create the database ");
        match Db::new(&path, true)  {
            Ok(db) => {
                let tmp_path = Path::new("/tmp/test2");
                assert_eq!(tmp_path, db.path);
              },
            Err(e) => {println!("test_db_new_2 err: {}", e); assert!(false)}
        };
        let exist = fs::metadata(path);
        assert!(exist.is_ok());
    }
    {
        println!("Should open an existing database ");
        match Db::new(&"/tmp/test2".to_string(), false)  {
            Ok(db) => {
                let tmp_path = Path::new("/tmp/test2");
                assert_eq!(tmp_path, db.path);
            },
            Err(e) => {println!("err : {}", e); assert!(false)}
        }
    }
    {
        println!("Should delete an existing database ");
        match Db::destroy(Path::new(path)) {
            Ok(_) => {}, 
            Err(e) => {
                println!("test_db_destroy err: {}", e);
                assert!(false);
            }

        }
        let exist = fs::metadata(path);
        assert!(exist.is_err());
    }
}

#[test]
#[should_panic]
fn test_db_new_1() {
    println!("Should panic database create_if_missing false");
    match Db::new(&"/tmp/test".to_string(), false) {
        Ok(_) => {},
        Err(e) => {println!("test_db_new_1 err: {}", e); assert!(false)}
    }
}

#[test]
#[should_panic]
fn test_db_new_2() {
    println!("Should panic database bad path");
    match Db::new(&"/tekjfbmp/test".to_string(), true)  {
        Ok(_) => {return;},
        Err(e) => {println!("test_db_new_2 err : {}", e); assert!(false)}
    }
}

#[test]
fn test_db_destroy() {
    println!("Should delete the entire database");
    let path = Path::new("/tmp/teskejnft2");
    match Db::destroy(path) {
        Ok(_) => {
            match fs::metadata(path) {
                Ok(_) => {
                    println!("err: Database still existing");
                    assert!(false);
                }, Err(_) => {}
            }
        }, Err(_) => {
            assert!(false);
        } 
    }
}

#[test]
fn test_db_put_as_string() {
    println!("Should put database");
    let path = Path::new("/tmp/put_as_string");
    {
        let db_path = String::from("/tmp/put_as_string");
        let db = Db::new(&db_path, true).unwrap();
        db.put_as_string(2, "toto".to_string()).unwrap();
        let get = db.get(2).unwrap();
        let string = String::from_utf8(get).unwrap();
        assert_eq!(string, "toto");
    }
    {
        Db::destroy(path).unwrap();
    }
}

#[test]
fn test_db_put_as_bytes() {
    println!("Should put in database");
    let path = Path::new("/tmp/put_as_bytes");
    {
        let db_path = String::from("/tmp/put_as_bytes");
        let db = Db::new(&db_path, true).unwrap();
        db.put_as_bytes(2, "toto".as_bytes()).unwrap();
        let get = db.get(2).unwrap();
        let string = String::from_utf8(get).unwrap();
        assert_eq!(string, "toto");
    }
    {
        Db::destroy(path).unwrap();
    }
}

#[test]
fn test_db_get_1() {
    println!("Should not get an unexisting key");
    let path = Path::new("/tmp/get1");
    {
        let db_path = String::from("/tmp/get1");
        let db = Db::new(&db_path, true).unwrap();
        let get = db.get(2);
        assert!(get.is_err());
    }
    {
        Db::destroy(path).unwrap();
    }
}

#[test]
fn test_db_get_2() {
    println!("Should get an existing key");
    let path = Path::new("/tmp/get2");
    {
        let db_path = String::from("/tmp/get2");
        let db = Db::new(&db_path, true).unwrap();
        db.put_as_string(2, "toto".to_string()).unwrap();
        let get = db.get(2).unwrap();
        let string = String::from_utf8(get).unwrap();
        assert_eq!(string, "toto");
    }
    {
        Db::destroy(path).unwrap();
    }
}

#[test]
fn test_db_delete_1() {
    println!("Should delete an existing key");
    let path = Path::new("/tmp/delete");
    {
        let db_path = String::from("/tmp/delete");
        let db = Db::new(&db_path, true).unwrap();
        db.put_as_string(2, "toto".to_string()).unwrap();
        let delete = db.delete(2);
        assert!(delete.is_ok());
    }
    {
        Db::destroy(path).unwrap();
    }
    
}


#[test]
fn test_db_delete_2() {
    println!("Should delete an unexisting key");
    let path = Path::new("/tmp/delete2");
    {
        let db_path = String::from("/tmp/delete2");
        let db = Db::new(&db_path, true).unwrap();
        let delete = db.delete(2);
        assert!(delete.is_ok());
    }
    {
        Db::destroy(path).unwrap();
    }
    
}
