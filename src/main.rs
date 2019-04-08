#![feature(proc_macro_hygiene, decl_macro)]

extern crate base64;
use base64::{encode, decode};

use std::str;
#[macro_use] 
extern crate rocket;

use class_database::{ClassDatabase,time_since_epoch,write_file};

#[get("/register/<id>/<pk>/<base64_repo_addr>")]
fn register(id: String, pk: String, base64_repo_addr: String) -> String {
	ClassDatabase::turn_off_host_checks();

    let mut filename:String = "/tmp/class_database/reg_".to_owned();
    
    filename.push_str(&format!("{}_{}", id,pk));//+=(id+"_"+&pk);
    let git_name_raw = &decode(&base64_repo_addr).expect("invalid base64")[..];
	let db = ClassDatabase::new("git@github.com:hortinstein/class_database.git");
      
    db.pull_class_database();
    let git_name = str::from_utf8(git_name_raw).unwrap();
    write_file(&filename,&git_name);
    db.add_file(&filename);
    format!("registration recived for {} with public key {}\n", id, pk)
}

fn main(){

	ClassDatabase::turn_off_host_checks();
    
    rocket::ignite().mount("/", routes![register]).launch();

    // let mut filename:String = "test".to_owned();
    // filename+= &time_since_epoch();
    // db.add_file(&filename);
}