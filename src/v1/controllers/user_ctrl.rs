use actix_web::{HttpResponse, Responder};
use crate::v1::models::users::User;

// Dummy database (to simulate data storage)
struct Database {
    users: Vec<User>,
}

impl Database {
    fn new() -> Self {
        Database { users: Vec::new() }
    }
}

pub trait UserControllerTrait {
    fn new() -> Self;
    // async fn get_users(&self);
}

pub struct UserController {
    db: Database,
}

impl UserControllerTrait for UserController {
    fn new() -> Self {
        UserController { db: Database::new() }
    }

    // async fn get_users(&self){
    //     String::from("value");
    // }
}
    // // POST request to create a new user
    // pub async fn create_user(&mut self, user: web::Json<User>) -> impl Responder {
    //     self.db.users.push(user.into_inner());
    //     HttpResponse::Created().body("User created successfully")
    // }

