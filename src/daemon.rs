extern crate iron_login;


use std::path::Path;
use std::error::Error;

use staticfile::Static;

use iron::prelude::*;
use iron::status;
use iron_login::User;

use urlencoded::UrlEncodedBody;

use iron;
use mount::Mount;
use router::{Router};
use api;

use rustc_serialize::json;
// use frank_jwt::Header;
// use frank_jwt::Payload;
// use frank_jwt::encode;
// use frank_jwt::decode;
// use frank_jwt::Algorithm;

#[derive(RustcEncodable)]
struct LogFileList {
  files: Vec<String>
}

#[derive(RustcEncodable)]
struct ErrorMessage {
  message: String
}

fn send_json(pair: (iron::status::Status,String)) -> IronResult<Response> {
  let mut response = Response::with(pair);

  let jsony_ctype = iron::headers::ContentType(iron::mime::Mime(
    iron::mime::TopLevel::Application,
    iron::mime::SubLevel::Json,
    vec![(iron::mime::Attr::Charset, iron::mime::Value::Utf8)]));

  response.headers.set::<iron::headers::ContentType>(jsony_ctype);
  Ok(response)
}

fn list_files() -> (iron::status::Status, String) {
  match api::find_files("./logs") {
    Ok(logs) => (status::Ok, json::encode(&LogFileList{ files: logs.iter().map(|&ref file| file[1..].to_string()).collect() }).unwrap()),
    Err(e) => (status::InternalServerError, json::encode(&ErrorMessage{message: e.description().to_string()}).unwrap())
  }
}


#[derive(Debug)]
/// Representation of an authenticated user
struct MyUser(String);
impl MyUser {
    fn new(user_id: &str) -> MyUser {
        MyUser(user_id.to_owned())
    }
}
impl User for MyUser {
    fn from_user_id(_: &mut Request, user_id: &str) -> Option<MyUser> {
        Some(MyUser(user_id.to_owned()))
    }
    fn get_user_id(&self) -> String {
        self.0.to_owned()
    }
}

/// A basic iron request handler
fn login_handler(req: &mut Request) -> IronResult<Response> {
    let login = MyUser::get_login(req);
    // If a query (`?username`) is passed, set the username to that string
    if let Some(ref uid) = req.url.query {
        // If no username is passed, log out
        if uid == "" {
            Ok(Response::new()
                   .set(::iron::status::Ok)
                   .set(format!("Logged out"))
                   .set(login.log_out()))
        } else {
            Ok(Response::new()
                   .set(::iron::status::Ok)
                   .set(format!("User set to '{}'", uid))
                   .set(login.log_in(MyUser::new(uid))))
        }
    } else {
        let user = login.get_user();
        Ok(Response::new()
               .set(::iron::status::Ok)
               .set(format!("user = {:?}", user)))
    }
}


// fn login_handler(req: &mut Request) -> IronResult<Response> {
//   match req.get_ref::<UrlEncodedBody>() {
//     Ok(ref hashmap) => {
//       if hashmap.contains_key("login") && hashmap.contains_key("password") {
//         let ref login = hashmap["login"][0];
//         let ref password = hashmap["password"][0];
//         println!("{:?} {:?}", login, password);
//       }
//     },
//     Err(ref e) => println!("{:?}", e)
//   };
//   Ok(Response::with((status::Ok, String::from("coucou"))))
// }

fn get_handler(req: &mut Request) -> IronResult<Response> {
  let query = req.extensions.get::<Router>().unwrap().find("method").unwrap_or("/");

  if query == "list" {
    send_json(list_files())
  } else {
    Ok(Response::with(status::NotFound))
  }
}

pub fn startup() {
  println!("Starting HTTP Daemon...");

  // FIXME compute this at startup
  let cookie_signing_key = b"LoginSecretKey"[..].to_owned();

  match api::find_files("./logs") {
    Ok(logs) => println!("Files: {:?}", logs),
    Err(e) => println!("Error: {:?}", e)
  }

  let mut mount = Mount::new();

  mount.mount("/api/1/files/logs", Static::new(Path::new("logs")));
  let mut router = Router::new();
  router.get("/:method", get_handler);
  router.post("/login", login_handler);

  mount.mount("/api/1/", router);
  mount.mount("/viewer", Static::new(Path::new("viewer")));

  let mut chain = Chain::new(mount);
  chain.around(iron_login::LoginManager::new(cookie_signing_key));

  println!("Open http://localhost:5001/viewer/");
  Iron::new(chain).http("0.0.0.0:5001").unwrap();
}
