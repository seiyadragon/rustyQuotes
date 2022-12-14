#[macro_use] extern crate rocket;

use rand::{Rng, rngs::StdRng, SeedableRng};
use chrono::{Datelike, Timelike, Utc};

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

const SUPABASE_URL: &str = "https://nuitnvbkhtnzqbcedbkl.supabase.co";
const SUPABASE_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Im51aXRudmJraHRuenFiY2VkYmtsIiwicm9sZSI6ImFub24iLCJpYXQiOjE2NzA3MzM2OTgsImV4cCI6MTk4NjMwOTY5OH0.pMl9EPd5kWiyLbwgSDtsmZZFXYZh3NdTp2s9_fBcj74";
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
       }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

async fn get_data() -> String {
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/rest/v1/Quotes", SUPABASE_URL))
        .header("apikey", SUPABASE_KEY)
        .header("Authorization", format!("Bearer {}", SUPABASE_KEY))
        .send()
        .await
        .unwrap()
    ;

    let body = res
        .text()
        .await
        .unwrap()
    ;

    format!("{}", body)
}

async fn split_data() -> Vec<String> {
    let data_string: &str = &format!("{}", &get_data().await);

    let mut clean_string = data_string.replace("[{", "");
    clean_string = clean_string.replace("}]", "");

    let split_quotes: Vec<&str> = clean_string.split("}, \n {").collect();
    
    to_string_vec(split_quotes)
}

fn to_string_vec(vector: Vec<&str>) -> Vec<String> {
    let mut vec_copy:Vec<String> = Vec::new();
    for quote in vector {
        vec_copy.push(quote.to_string());
    }

    vec_copy
}

#[get("/")]
async fn quotes() -> String {
    get_data().await
}

#[get("/<id>")]
async fn quote(id: usize) -> String {
    let split_quotes = split_data().await;

    format!("{{{}}}", split_quotes[id - 1])
}

#[get("/random")]
async fn random() -> String {
    let split_quotes = split_data().await;
    let mut rng = rand::thread_rng();

    format!("{{{}}}", split_quotes[rng.gen_range(0..split_quotes.len())])
}

#[get("/daily")]
async fn daily() -> String {
    let split_quotes = split_data().await;
    let now = Utc::now();
    let mut rng = StdRng::seed_from_u64(now.year() as u64 + now.month() as u64 + now.day() as u64);

    format!("{{{}}}", split_quotes[rng.gen_range(0..split_quotes.len())])
}

#[get("/hourly")]
async fn hourly() -> String {
    let split_quotes = split_data().await;
    let now = Utc::now();
    let mut rng = StdRng::seed_from_u64(now.year() as u64 + now.month() as u64 + now.day() as u64 + now.hour() as u64);

    format!("{{{}}}", split_quotes[rng.gen_range(0..split_quotes.len())])
}

#[get("/byauthor/<author>")]
async fn by_author(author: String) -> String {
    let split_quotes = split_data().await;
    let mut return_quotes: Vec<String> = vec![];
    let mut final_string: String = "".to_string();

    for i in 0..split_quotes.len() {
        let split_quote: Vec<&str> = split_quotes[i].split(",").collect();

        let author_vec: Vec<&str> = split_quote[0].split(":").collect();
        let author_value = author_vec[1];

        if author_value.to_lowercase().contains(&author.to_lowercase()) {
            return_quotes.push(format!("{{{}}}", split_quotes[i].to_string()));
        }
    }

    for i in 0..return_quotes.len() {
        final_string.push_str(&return_quotes[i]);
    }

    format!("{}", final_string)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![quotes])
        .mount("/quotes", routes![quotes])
        .mount("/quotes", routes![quote])
        .mount("/quotes", routes![random])
        .mount("/quotes", routes![by_author])
        .mount("/quotes", routes![daily])
        .mount("/quotes", routes![hourly])
}