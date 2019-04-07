

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


use class_database::{ClassDatabase,time_since_epoch};
#[get("/register/<id>/<pk>/<base64_repo_address>")]
fn hello(id: String, pk: String, base64_repo_addr: String) -> String {
    format!("registration recived for {} with public key {}\n", id, pk)
}

fn main(){

	ClassDatabase::turn_off_host_checks();
	let db = ClassDatabase::new("git@github.com:replicatedu/test_database.git");
        
    db.pull_class_database();

    rocket::ignite().mount("/", routes![hello]).launch();

    // let mut filename:String = "test".to_owned();
    // filename+= &time_since_epoch();
    // db.add_file(&filename);
}