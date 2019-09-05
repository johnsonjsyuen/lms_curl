extern crate clap;

use std::io::{stdout, Write};
use std::str;

use std::process::Command;
use std::io::Read;
use std::collections::HashMap;

use curl::easy::{Easy, List};
use clap::{Arg, App, SubCommand};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Curl LMS Dev")
        .arg(Arg::with_name("PATH")
            .help("Sets the api path to call")
            .required(true)
            .index(1))
        .get_matches();

    curl_api(matches.value_of("PATH").unwrap())
}

fn curl_api(api_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://location-management-system.sd.dev.outfra.xyz{}", api_path);
    let jwt = jwt();
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
    Ok(())
}

fn jwt() -> String {
    let command = "/usr/local/bin/npm";
    let cwd = "/Users/jyuen/code/aips-partner-portal-xapi";
    let npm_args = vec!["run", "--silent", "generate-token", "--", "location-management-system", "jyuen@seek.com.au", "Johnson Yuen"];

    let output = Command::new(command)
        .args(&npm_args)
        .current_dir(cwd)
        .output()
        .expect("Failed to execute command");
    let mut jwt = String::new();
    output.stdout.as_slice().read_to_string(&mut jwt).unwrap();
    jwt
}
