#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
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
    fn hello_root() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
    #[test]
    fn hello_nonexist_path() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/register").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}

