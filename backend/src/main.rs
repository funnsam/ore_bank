use rocket::*;

fn _verify(user: &str, password: &str) -> bool {
    let pw_hash = sha256::digest(password);
    pw_hash == "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad".to_string()
}

macro_rules! verify_or_return {
    ($user: expr, $password: expr) => {
        if !_verify($user, $password) {
            return "fail login";
        }
    };
}

#[get("/verify/<user>/<password>")]
fn verify(user: &str, password: &str) -> &'static str {
    verify_or_return!(user, password);
    "ok"
}

#[get("/transfer/<user>/<password>/<to_user>/<amount>")]
fn transfer(user: &str, password: &str, to_user: &str, amount: usize) -> &'static str {
    verify_or_return!(user, password);
    "ok"
}

#[launch]
fn rocket() -> _ {
    build()
        .mount("/", routes![verify, transfer])
}
