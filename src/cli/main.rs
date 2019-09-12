extern crate clap;
extern crate lib_lms_curl;

use clap::{App, Arg};
use lib_lms_curl::curl_api;
use lib_lms_curl::StringUtils;

fn main(){
    let matches = App::new("Curl LMS Dev")
        .arg(Arg::with_name("PATH")
            .help("Sets the api path to call. This may be quoted or unquoted")
            .required(true)
            .index(1))
        .about("Do a HTTP with the LMS Dev server, automatically handling JWT, so you don't have to")
        .get_matches();

    let path = matches.value_of("PATH").unwrap();

    let resp = match path.substring(    0,1){
        "\"" => curl_api(path.substring(1,path.len()-1)),
        _ => curl_api(path)
    };
    println!("{}",resp)

}

