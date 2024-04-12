
use serde::{Serialize, Deserialize};
use super::schema::edukasi;
use super::schema::experience;
use super::schema::contact;
use super::schema::portofolio;
use super::schema::hero;
use super::schema::skill;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Edukasi {
    pub id: String, // Menggunakan String untuk UUID
    pub title: String,
    pub major: String,
    pub periode: String,
    pub description: String,
    pub logo: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Experience {
    pub id: String, // Menggunakan String untuk UUID
    pub periode: String,
    pub title: String,
    pub division: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Contact {
    pub id: String, // Menggunakan String untuk UUID
    pub location: String,
    pub phone_number: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Portofolio {
    pub id: String, // Menggunakan String untuk UUID
    pub title: String,
    pub description: String,
    pub skills: String,
    pub link: String,
    pub image: String,
}   

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Hero {
    pub id: String, // Menggunakan String untuk UUID
    pub title: String,
    pub description: String,
    pub image: String,
}   

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Skill {
    pub id: String, // Menggunakan String untuk UUID
    pub skills: String,
}   

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "edukasi"]
pub struct NewEdukasi {
    pub id: Option<String>,
    pub title: String,
    pub major: String,
    pub periode: String,
    pub description: String,
    pub logo: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "experience"]
pub struct NewExperience {
    pub id: Option<String>,
    pub periode: String,
    pub title: String,
    pub division: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "contact"]
pub struct NewContact {
    pub id: Option<String>,
    pub location: String,
    pub phone_number: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "portofolio"]
pub struct NewPortofolio {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub skills: String, 
    pub link: String,
    pub image: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "hero"]
pub struct NewHero {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub image: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "skill"]
pub struct NewSkill {
    pub id: Option<String>,
    pub skills: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEdukasi {
    pub title: String,
    pub major: String,
    pub periode: String,
    pub description: String,
    pub logo: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateExperience {
    pub periode: String,
    pub title: String,
    pub division: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContact {
    pub location: String,
    pub phone_number: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePortofolio {
    pub title: String,
    pub description: String,
    pub skills: String, 
    pub link: String,
    pub image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHero {
    pub title: String,
    pub description: String,
    pub image: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSkill {
    pub skills: String,
}
