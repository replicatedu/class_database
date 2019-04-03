use class_database::{ClassDatabase,time_since_epoch};

fn main(){

	ClassDatabase::turn_off_host_checks();
	let db = ClassDatabase::new("git@github.com:replicatedu/test_database.git");
        
    db.pull_class_database();
    let mut filename:String = "test".to_owned();
    filename+= &time_since_epoch();
    db.add_file(&filename);
}