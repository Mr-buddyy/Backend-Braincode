
#[macro_use]
extern crate diesel;

use actix_web::{ http, delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use diesel::mysql::MysqlConnection; // Import MysqlConnection
use actix_cors::Cors;

mod schema;
mod models;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[delete("/edukasi")]
async fn delete_edukasi(pool: web::Data<DbPool>, id: web::Path<String>) -> impl Responder {
    // Acquire a connection from the pool
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    // Query the database to delete edukasi data
    let deleted_rows = diesel::delete(schema::edukasi::table.filter(schema::edukasi::id.eq(id.into_inner())))
        .execute(&mut conn)
        .expect("Error deleting edukasi data from database");

    HttpResponse::Ok().json(deleted_rows)
}

#[get("/edukasi")]
async fn get_edukasi(pool: web::Data<DbPool>) -> impl Responder {
    // Acquire a connection from the pool
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    // Query the database to retrieve edukasi data
    let edukasi_data = schema::edukasi::table
        .load::<models::Edukasi>(&mut conn)
        .expect("Error loading edukasi data from database");

    HttpResponse::Ok().json(edukasi_data)
}

#[post("/edukasi")]
async fn post_edukasi(pool: web::Data<DbPool>, edukasi:web::Json<models::CreateEdukasi>) -> impl Responder {
    // Acquire a connection from the pool
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    // Query the database to retrieve edukasi data
    let post_edukasi = models::NewEdukasi {
        id: Some(uuid::Uuid::new_v4().to_string()),
        title: edukasi.title.clone(),
        major: edukasi.major.clone(),
        periode: edukasi.periode.clone(),
        description: edukasi.description.clone(),
        logo: edukasi.logo.clone(),
    };

    diesel::insert_into(schema::edukasi::table)
        .values(&post_edukasi)
        .execute(&mut conn)
        .expect("Error saving new edukasi");

    HttpResponse::Ok().body("New edukasi has been saved")
}

#[get("/hero")]
async fn get_hero(pool: web::Data<DbPool>) -> impl Responder {
    // Acquire a connection from the pool
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    // Query the database to retrieve edukasi data
    let hero_data = schema::hero::table
        .load::<models::Hero>(&mut conn)
        .expect("Error loading edukasi data from database");

    HttpResponse::Ok().json(hero_data)

}

#[post("/hero")]
async fn post_hero(pool: web::Data<DbPool>, hero:web::Json<models::CreateHero>) -> impl Responder {
    // Acquire a connection from the pool
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    // Query the database to retrieve edukasi data
    let post_hero = models::NewHero {
        id: Some(uuid::Uuid::new_v4().to_string()),
        title: hero.title.clone(),
        description: hero.description.clone(),
        image: hero.image.clone(),
    };

    diesel::insert_into(schema::hero::table)
        .values(&post_hero)
        .execute(&mut conn)
        .expect("Error saving new edukasi");

    HttpResponse::Ok().body("New edukasi has been saved")
}

#[get("/experience")]
async fn get_experience(pool: web::Data<DbPool>) -> impl Responder {
    // Acquire a connection from the pool
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    // Query the database to retrieve experience data
    let experience_data = schema::experience::table
        .load::<models::Experience>(&mut conn)
        .expect("Error loading experience data from database");

    // Return the experience data as JSON response
    HttpResponse::Ok().json(experience_data)
}

#[post("/experience")]
async fn post_experience(pool: web::Data<DbPool>, experience:web::Json<models::CreateExperience>) -> impl Responder {
    // Acquire a connection from the pool
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    // Query the database to retrieve edukasi data
    let post_experience = models::NewExperience {
        id: Some(uuid::Uuid::new_v4().to_string()),
        periode: experience.periode.clone(),
        title: experience.title.clone(),
        division: experience.division.clone(),
        description: experience.description.clone(),
    };

    diesel::insert_into(schema::experience::table)
        .values(&post_experience)
        .execute(&mut conn)
        .expect("Error saving new edukasi");

    HttpResponse::Ok().body("New edukasi has been saved")
}

#[get("/contact")]
async fn get_contact(pool: web::Data<DbPool>) -> impl Responder {
    // Acquire a connection from the pool
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let contact_data = schema::contact::table
        .load::<models::Contact>(&mut conn)
        .expect("Error loading contact data from database");
    // Return the experiences as JSON response
    HttpResponse::Ok().json(contact_data)
}

#[post("/contact")]
async fn post_contact(pool: web::Data<DbPool>, contact:web::Json<models::CreateContact>) -> impl Responder {
    // Acquire a connection from the pool
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    // Query the database to retrieve edukasi data
    let post_contact = models::NewContact {
        id: Some(uuid::Uuid::new_v4().to_string()),
        location: contact.location.clone(),
        phone_number: contact.phone_number.clone(),
        email: contact.email.clone(),
    };

    diesel::insert_into(schema::contact::table)
        .values(&post_contact)
        .execute(&mut conn)
        .expect("Error saving new edukasi");

    HttpResponse::Ok().body("New edukasi has been saved")
}

#[get("/portofolio")]
async fn get_portofolio(pool: web::Data<DbPool>) -> impl Responder {
    // Acquire a connection from the pool
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let portofolio_data = schema::portofolio::table
        .load::<models::Portofolio>(&mut conn)
        .expect("Error loading contact data from database");
    // Return the experiences as JSON response
    HttpResponse::Ok().json(portofolio_data)
}

#[post("/portofolio")]
async fn post_portofolio(pool: web::Data<DbPool>, portofolio: web::Json<models::CreatePortofolio>) -> impl Responder {
    // Acquire a connection from the pool
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    // Query the database to retrieve edukasi data
    let post_portofolio = models::NewPortofolio {
        id: Some(uuid::Uuid::new_v4().to_string()),
        title: portofolio.title.clone(),
        description: portofolio.description.clone(),
        skills: portofolio.skills.clone(), 
        link: portofolio.link.clone(), 
        image:portofolio.image.clone(),
    };

    diesel::insert_into(schema::portofolio::table)
        .values(&post_portofolio)
        .execute(&mut conn)
        .expect("Error saving new edukasi");

    HttpResponse::Ok().body("New portofolio has been saved")
}

#[get("/skill")]
async fn get_skill(pool: web::Data<DbPool>) -> impl Responder {
    // Acquire a connection from the pool
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let skill_data = schema::skill::table
        .load::<models::Skill>(&mut conn)
        .expect("Error loading contact data from database");
    // Return the experiences as JSON response
    HttpResponse::Ok().json(skill_data)
}

#[post("/skill")]
async fn post_skill(pool: web::Data<DbPool>, skill: web::Json<models::CreateSkill>) -> impl Responder {
    // Acquire a connection from the pool
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    // Query the database to retrieve edukasi data
    let post_skill = models::NewSkill {
        id: Some(uuid::Uuid::new_v4().to_string()),
        skills: skill.skills.clone(),
    };

    diesel::insert_into(schema::skill::table)
        .values(&post_skill)
        .execute(&mut conn)
        .expect("Error saving new edukasi");

    HttpResponse::Ok().body("New portofolio has been saved")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Read the value of the environment variable DATABASE_URL
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        panic!("DATABASE_URL must be set");
    });

    // Create a connection manager
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    // Create a connection pool
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .data(pool.clone()) // Share the connection pool across all routes
            .service(post_skill)
            .service(get_skill)
            .service(post_portofolio)
            .service(get_contact)
            .service(post_hero)
            .service(get_hero)
            .service(get_portofolio)
            .service(post_experience)
            .service(post_edukasi)
            .service(get_experience)
            .service(get_edukasi)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
