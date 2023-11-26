use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use validator::Validate;

use crate::db::connection::DbPool;
use crate::schema::users::dsl::{email, username, users};
use crate::utils::{ generate_jwt_token, hash_password, verify_password, decode_jwt_token };
use crate::v1::models::users::{NewUser, SignInData, User};

pub async fn sign_up(pool: web::Data<DbPool>, data: web::Form<NewUser>) -> HttpResponse {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // Validate the incoming user data
    match data.validate() {

        Ok(_) => {
            let user = data.into_inner();

            let new_user: NewUser = NewUser {
                username: user.username,
                email: user.email,
                password: hash_password(&user.password),
                role: user.role,
            };

            match diesel::insert_into(users)
                .values(&new_user)
                .execute(&mut conn)
            {
                Ok(_) => {
                    let token = generate_jwt_token(&new_user.username).unwrap();
                    return HttpResponse::Ok()
                        .body(format!("User stored successfully. Token: {}", token));
                }
                Err(e) => {
                    return HttpResponse::BadRequest().body(format!("{:?}", e));
                }
            }
        }

        Err(e) => {
            return HttpResponse::BadRequest().body(format!("{:?}", e));
        }
    }
}

pub async fn sign_in(pool: web::Data<DbPool>, data: web::Form<SignInData>) -> HttpResponse {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let creds = data.into_inner();

    // Check if the input is an email or a username
    let user = match creds.username_or_email.contains('@') {
        true => users
            .filter(email.eq(&creds.username_or_email))
            .first::<User>(&mut conn),
        false => users
            .filter(username.eq(&creds.username_or_email))
            .first::<User>(&mut conn),
    };

    match user {
        Ok(user) => {
            // Check if the password matches
            if verify_password(&user.password, &creds.password) {
                let token = generate_jwt_token(&user.username).unwrap();
                return HttpResponse::Ok()
                    .body(format!("User stored successfully. Token: {}", token));
            } else {
                return HttpResponse::Unauthorized().body("Invalid password");
            }
        }
        Err(_) => {
            return HttpResponse::NotFound().body("User not found");
        }
    }
}

pub async fn verify_token(_pool: web::Data<DbPool>, token: web::Path<String>) -> HttpResponse {

    match decode_jwt_token(&token)  {
        Ok(_)=>{
            return HttpResponse::Ok().body("Token is valid");
        }
        Err(_)=>{
            return HttpResponse::Unauthorized().body("Token verification failed");
        }
    }
}

// pub async fn get_users(pool: web::Data<DbPool>) -> HttpResponse {
//     let mut conn = pool.get().expect("couldn't get db connection from pool");
//     let users_data: Result<Vec<User>, diesel::result::Error> = users.load::<User>(&mut conn);

//     match users_data {
//         Ok(other_users) => {
//             let users_json: serde_json::Value = serde_json::to_value(&other_users).unwrap();
//             HttpResponse::Ok().json(users_json)
//         }
//         Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
//     }
// }

// pub async fn get_users(pool: web::Data<DbPool>) -> HttpResponse {
//     let mut conn = pool.get().expect("couldn't get db connection from pool");
//     let emp_data = users.select(Emp::as_select()).load::<Emp>(&mut conn);

//     match emp_data {
//         Ok(other_users) => {
//             let users_json = serde_json::to_value(&other_users).unwrap();
//             HttpResponse::Ok().json(users_json)
//         }
//         Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
//     }
// }

// pub async fn get_users(pool: web::Data<DbPool>) -> HttpResponse {
//     let mut conn = pool.get().expect("couldn't get db connection from pool");
//     let users_data: Result<Vec<User>, diesel::result::Error> = users.load::<User>(&mut conn);

//     match users_data {
//         Ok(other_users ) => {
//             let users_json = serde_json::to_string(&other_users).unwrap();
//             HttpResponse::Ok().json(users_json)
//         }
//         Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
//     }
// }

//-----------------------------------------------------------------------------------------
// pub async fn get_users(pool: web::Data<DbPool>) -> HttpResponse {
//     let db = Database::new();
//     let mut conn = pool.get().expect("couldn't get db connection from pool");

//     let usrs = users.load::<User>(&mut conn)?;
//     let users_json = serde_json::to_string(&usrs)?;

//     HttpResponse::Ok().json(users_json)

//     // // Create insertion model
//     // let new_user = NewUser {
//     //     email: String::from("hola@gmail.com"),
//     //     password: String::from("1231654"),
//     //     role: String::from("value"),
//     //     username: String::from("username"),
//     // };
//     // normal diesel operations
//     // diesel::insert_into(users)
//     //     .values(&new_user)
//     //     .execute(&mut conn)
//     //     .expect("Error inserting person");

//     // let count = posts::table.count().get_results(&conn).unwrap();
//     // let user = users
//     //     .first::<User>(&mut conn)
//     //     .expect("Error loading person that was just inserted");

//     // HttpResponse::Ok().body("body")
// }
// pub async fn get_users(pool: web::Data<DbPool>) -> HttpResponse {
//     let db = Database::new();

//     HttpResponse::Ok().body("body")
// }

// // POST /users
// pub async fn create_user(pool: web::Data<DbPool>, user: web::Json<NewUser>) -> impl Responder {
//     let conn = pool.get().expect("couldn't get db connection from pool");

//     let new_user = user.into_inner();

//     if let Err(errors) = new_user.validate() {
//         return HttpResponse::BadRequest().json(errors);
//     }

//     if let Err(errors) = new_user.validate() {
//         return HttpResponse::BadRequest().json(errors);
//     }

//     // let new_post = NewUser { title, body };

//     // diesel::insert_into(user::table)
//     //     .values(&new_post)
//     //     .returning(Post::as_returning())
//     //     .get_result(conn)
//     //     .expect("Error saving new post");

//     HttpResponse::Created().into()
// }
