#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::http::RawStr;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[options("/")]
fn index_options() -> &'static str {
    "No otpion defined."
}

#[get("/hello/<name>")]
fn hello(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, index_options,hello])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn hello_name(){
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/hello/dora").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, dora!".into()));
    }
    #[test]
    fn get_root() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
    #[test]
    fn post_root() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.post("/").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
    #[test]
    fn put_root() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.put("/").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
     #[test]
    fn del_root() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.delete("/").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
    #[test]
    fn head_root() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.head("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
     #[test]
    fn option_root() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.options("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
       #[test]
    fn hello_nonexist_path() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/register").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}

