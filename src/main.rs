extern crate clap;

use std::io::{stdout, Write};
use std::str;

use std::process::Command;
use std::io::Read;
use std::collections::HashMap;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use curl::easy::{Easy, List};
use clap::{Arg, App, SubCommand};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use chrono::{DateTime, Utc, Duration};


fn main(){
    let matches = App::new("Curl LMS Dev")
        .arg(Arg::with_name("PATH")
            .help("Sets the api path to call")
            .required(true)
            .index(1))
        .get_matches();

    curl_api(matches.value_of("PATH").unwrap());
}

fn curl_api(api_path: &str){
    let url = format!("http://location-management-system.sd.dev.outfra.xyz{}", api_path);
    let jwt = get_jwt();
    let auth_header_value = format!("Bearer {}", jwt);

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

    print!("{}", resp);
}

#[derive(Serialize, Deserialize)]
struct JWTFile {
    created_at_utc: DateTime<Utc>,
    jwt: String,
}

fn get_jwt() -> String {
    let file_path = "/tmp/jwt";
    match try_read_file(file_path) {
        None => {
            let new_jwt = generate_jwt();
            let j = JWTFile { created_at_utc: Utc::now(), jwt: new_jwt.clone() };
            let mut output_file = File::create(file_path).unwrap();
            serde_json::to_writer(output_file, &j);
            new_jwt
        }
        Some(file_data) => {
            let jwt_file: JWTFile = serde_json::from_str(&file_data).unwrap();
            let now = Utc::now();

            if now.signed_duration_since(jwt_file.created_at_utc) < Duration::seconds(148) {
                jwt_file.jwt
            } else {
                generate_jwt()
            }
        }
    }
}

fn generate_jwt() -> String {
    let command = "/usr/local/bin/npm";
    let xapi_repo_directory = "/Users/jyuen/code/aips-partner-portal-xapi";
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


fn try_read_file(file_path: &str) -> Option<String> {
    if Path::new(file_path).exists() {
        let mut buf = String::new();
        let mut input_file = File::open(file_path).unwrap();
        input_file.read_to_string(&mut buf);
        Some(buf)
    } else {
        None
    }
}
