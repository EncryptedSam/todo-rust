use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

// Sample user structure
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: u64,
    name: String,
    email: String,
}

// Dummy database (to simulate data storage)
struct Database {
    users: Vec<User>,
}

impl Database {
    fn new() -> Self {
        Database { users: Vec::new() }
    }
}

pub struct UserController {
    db: Database,
}

impl UserController {
    pub fn new() -> Self {
        UserController { db: Database::new() }
    }

    // GET request to retrieve all users
    pub async fn get_users(&self) -> impl Responder {
        HttpResponse::Ok().json(&self.db.users)
    }

    // GET request to retrieve a single user by ID
    pub async fn get_user_by_id(&self, user_id: web::Path<u64>) -> impl Responder {
        let user = self
            .db
            .users
            .iter()
            .find(|&u| u.id == *user_id)
            .ok_or_else(|| HttpResponse::NotFound().body("User not found"));

        match user {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(response) => response,
        }
    }

    // POST request to create a new user
    pub async fn create_user(&mut self, user: web::Json<User>) -> impl Responder {
        self.db.users.push(user.into_inner());
        HttpResponse::Created().body("User created successfully")
    }

    // PUT request to update a user by ID
    pub async fn update_user(&mut self, user_id: web::Path<u64>, updated_user: web::Json<User>) -> impl Responder {
        let index = self
            .db
            .users
            .iter()
            .position(|u| u.id == *user_id)
            .ok_or_else(|| HttpResponse::NotFound().body("User not found"));

        match index {
            Ok(idx) => {
                self.db.users[idx] = updated_user.into_inner();
                HttpResponse::Ok().body("User updated successfully")
            }
            Err(response) => response,
        }
    }

    // DELETE request to delete a user by ID
    pub async fn delete_user(&mut self, user_id: web::Path<u64>) -> impl Responder {
        if let Some(pos) = self.db.users.iter().position(|u| u.id == *user_id) {
            self.db.users.remove(pos);
            HttpResponse::Ok().body("User deleted successfully")
        } else {
            HttpResponse::NotFound().body("User not found")
        }
    }
}
