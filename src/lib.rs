use std::fs;
use std::fs::OpenOptions;
use std::io;

use std::process::{Command};

#[macro_use]
extern crate serde_derive;
extern crate toml;

use hex;
use class_crypto::{ClassCrypto};
use std::time::{SystemTime, UNIX_EPOCH};

//use hubcaps::repositories::{RepoOptions};


//returns a command setup ready to run the tests
fn command_wrapper(test_command: &str, command_directory: &str) -> Command {
    let mut command = if cfg!(target_os = "windows") {
        {
            let mut c = Command::new("cmd");
            c.args(&["/C", test_command]);
            c
        }
    } else {
        {
            let mut c = Command::new("sh");
            c.arg("-c");
            c.arg(test_command);
            c
        }
    };
    command.current_dir(command_directory);
    command
}


//rsa key generation
//ssh-keygen -f /etc/ssh/ssh_host_rsa_key -N '' -t rsa

pub fn gen_rsa_keys(){
    let command = "rm -rf ./deploy_key* && \
                   ssh-keygen -f ./deploy_key -N '' -t rsa && \
                   echo \"paste the following into deploy keys\" && \
                   cat deploy_key.pub &&
                   ssh-add -y -K ./deploy_key";
    let mut c = command_wrapper(&command, ".");
    println!("{}",String::from_utf8_lossy(&c.output().unwrap().stdout));
    let command = "rm -rf ./deploy_key* && \
                   ssh-keygen -f ./deploy_key -N '' -t rsa && \
                   echo \"paste the following into deploy keys\" && \
                   cat deploy_key.pub &&
                   ssh-add -y -K ./deploy_key";
    let mut c = command_wrapper(&command, ".");
    println!("{}",String::from_utf8_lossy(&c.output().unwrap().stdout));

}


pub fn prompt() -> ClassDatabase {
    println!("Please enter ssh repo address [ie git@github.com:replicatedu/test_database.git]");
    let mut repo_address = String::new();
    io::stdin().read_line(&mut repo_address)
        .expect("Failed to read repo address");
    ClassDatabase::new(&repo_address)
}


//holds data for instructor and students
pub struct ClassDatabase {
	repo_address: String,
}

impl ClassDatabase {
    //constructs a new database
    pub fn new(github_repo: &str) -> ClassDatabase {
        ClassDatabase{
            repo_address: github_repo.to_string()
        }
    }
 
    pub fn pull_class_database(&self) {
        let owned_string: String = "git clone ".to_owned();
        let command = owned_string + &self.repo_address;
        println!("running: {}",command);
        let mut c = command_wrapper(&command, "/tmp");
        c.output();
    }

    pub fn turn_off_host_checks(){
        let mut command: String = "mkdir -p ~/.ssh && ".to_owned();     
        command+="echo \"Host *\" > ~/.ssh/config && ";
        command+="echo \" StrictHostKeyChecking no\" >> ~/.ssh/config";
        println!("running: {}",command);
        let mut c = command_wrapper(&command, "/");
        println!("{}",String::from_utf8_lossy(&c.output().unwrap().stdout));
    
    }

    pub fn turn_on_host_checks(){
        let mut command: String = "rm ~/.ssh/config && ".to_owned();     
        command+="echo \"Host *\" > ~/.ssh/config && ";
        command+="echo \" StrictHostKeyChecking no\" >> ~/.ssh/config";
        println!("running: {}",command);
        let mut c = command_wrapper(&command, "/tmp/test_database");
        println!("{}",String::from_utf8_lossy(&c.output().unwrap().stdout));
    
    }

    pub fn add_file(&self,filename: &str){
        let mut command: String = "touch ".to_owned();
        command += &filename;
        command += " && git pull ";
        command += " && git add ";
        command += &filename;
        command += " && git commit -a -m \"added a new file\" ";
        command += "&& git push origin master";
        println!("running: {}",command);
        let mut c = command_wrapper(&command, "/tmp/test_database");
        println!("{}",String::from_utf8_lossy(&c.output().unwrap().stdout));
    }

    pub fn prune_files(&self){
        let mut command: String = "find . -not -name .git -exec rm -vf {} \\;".to_owned();
        let mut c = command_wrapper(&command, "/tmp/test_database");
        println!("{}",String::from_utf8_lossy(&c.output().unwrap().stdout));
    }
    pub fn create_repo(&self){
        let mut command: String = "curl -u 'hortinstein' https://api.github.com/user/repos -d '{\"name\":\"REPOs\"}'".to_owned();
        
        let mut c = command_wrapper(&command, "/tmp/");
        println!("{}",String::from_utf8_lossy(&c.output().unwrap().stdout));
    }
}
pub fn time_since_epoch() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gen_test_keypair() {

        // ClassDatabase::turn_off_host_checks();
        // get_rsa_key();
        // let db = ClassDatabase::new("git@github.com:replicatedu/test_database.git");
        
        // db.pull_class_database();
        // let mut filename:String = "test".to_owned();
        // filename+= &time_since_epoch();
        // db.add_file(&filename);
        ClassDatabase::create_repo();
    }
}