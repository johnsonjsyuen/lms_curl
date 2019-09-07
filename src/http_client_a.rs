use actix_web::Error;
use actix_web::client::Client;
use actix::System;
use std::{str, error};
use futures::future::lazy;
use futures::future::{join_all, ok as fut_ok, Future};
use tokio::executor::spawn;

use crate::lib_lms_curl::get_jwt;


pub fn get_lms_path(api_path: &str) -> Result<String, Error> {
    let url = format!("http://location-management-system.sd.dev.outfra.xyz{}", api_path);
    let jwt = get_jwt();
    let jwt_header = format!("Bearer {}",jwt);

    let mut resp = String::new();

    let a = spawn(lazy(move || {
        Client::new()
            .get(url) // <- Create request builder
            .header("Authorization", jwt_header)
            .send() // <- Send http request
            .from_err()
            .and_then(|mut response| {
                // read response body
                response
                    .body()
                    .from_err()
                    .and_then(|body|{
                        fut_ok(resp.push_str( str::from_utf8(&body).unwrap()))
                    })
            })
    }));
    Ok(resp)
}
