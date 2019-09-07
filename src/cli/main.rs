extern crate clap;
extern crate lib_lms_curl;

use clap::{App, Arg};
use lib_lms_curl::curl_api;

fn main(){
    let matches = App::new("Curl LMS Dev")
        .arg(Arg::with_name("PATH")
            .help("Sets the api path to call")
            .required(true)
            .index(1))
        .about("Do a HTTP with the LMS Dev server, automatically handling JWT, so you don't have to")
        .get_matches();

    let resp = curl_api(matches.value_of("PATH").unwrap());
    print!("{}",resp);
}

