use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use std::str;
use std::thread;
use std::sync::{Arc};

use chrono::{DateTime, Duration, Utc};
use curl::easy::{Easy, List};
use serde::{Deserialize, Serialize};


pub fn curl_api(api_path: &str)->String{
    let url = format!("http://location-management-system.sd.dev.outfra.xyz{}", api_path);
    let jwt = get_jwt();

    let mut buf = Vec::new();

    let mut handle = Easy::new();
    handle.url(&url).unwrap();

    let mut list = List::new();
    list.append(&format!("Authorization: Bearer {}", jwt)).unwrap();

    handle.http_headers(list).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            buf.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let resp = str::from_utf8(&buf).unwrap();

    resp.to_owned()
}

#[derive(Serialize, Deserialize)]
struct JWTFile {
    created_at_utc: DateTime<Utc>,
    jwt: String,
}

pub fn get_jwt() -> String {
    let file_path = Arc::new("/tmp/jwt".to_owned());
    match try_read_file(Arc::clone(&file_path)) {
        None => {
            let new_jwt = generate_jwt();
            let new_jwt_copy = new_jwt.clone();
            let fpc = Arc::clone(&file_path);
            thread::spawn( move || {
                let j = JWTFile { created_at_utc: Utc::now(), jwt: new_jwt_copy };
                let output_file = File::create(&*fpc).unwrap();
                serde_json::to_writer(output_file, &j).unwrap();
            });
            new_jwt
        }
        Some(file_data) => {
            let jwt_file: JWTFile = serde_json::from_str(&file_data).unwrap();
            let now = Utc::now();

            if now.signed_duration_since(jwt_file.created_at_utc) < Duration::seconds(148) {
                jwt_file.jwt
            } else {
                let fpc = Arc::clone(&file_path);
                thread::spawn(move || {
                    fs::remove_file(&*fpc).unwrap();
                });
                generate_jwt()
            }
        }
    }
}

fn generate_jwt() -> String {
    let command = "/usr/local/bin/npm";
    let xapi_repo_directory = "/Users/johnson/johnsonsync/Seek/code/aips-partner-portal-xapi";
    let npm_args = vec!["run", "--silent", "generate-token", "--", "location-management-system", "jyuen@seek.com.au", "Johnson Yuen"];

    let output = Command::new(command)
        .args(&npm_args)
        .current_dir(xapi_repo_directory)
        .output()
        .expect("Failed to execute command");
    let mut jwt = String::new();
    output.stdout.as_slice().read_to_string(&mut jwt).unwrap();
    jwt
}


fn try_read_file(file_path: Arc<String>) -> Option<String> {

    if Path::new(&*file_path).exists() {
        let mut buf = String::new();
        let mut input_file = File::open(&*file_path).unwrap();
        match input_file.read_to_string(&mut buf){
            Ok(_) => Some(buf),
            Err(_) => {
                let fpc = Arc::clone(&file_path);
                thread::spawn( move || {
                    fs::remove_file(&*fpc).unwrap();
                });
                None
            }
        }
    } else {
        None
    }
}
