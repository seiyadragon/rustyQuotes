#[macro_use] extern crate rocket;

use rand::{Rng, rngs::StdRng, SeedableRng};
use chrono::{Datelike, Timelike, Utc};

use rocket::http::Method;
use rocket::{get, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

const SUPABASE_URL: &str = "https://nuitnvbkhtnzqbcedbkl.supabase.co";
const SUPABASE_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Im51aXRudmJraHRuenFiY2VkYmtsIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTY3MDczMzY5OCwiZXhwIjoxOTg2MzA5Njk4fQ.xek_eDtlbEWczVog9dprPzjnEyE32bfPEBbgby_CAG8";

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
    let allowed_origins = AllowedOrigins::some_exact(&["https://seiyadragon.vercel.app"]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors().unwrap();

    rocket::build()
        .mount("/", routes![quotes])
        .mount("/quotes", routes![quotes])
        .mount("/quotes", routes![quote])
        .mount("/quotes", routes![random])
        .mount("/quotes", routes![by_author])
        .mount("/quotes", routes![daily])
        .mount("/quotes", routes![hourly])
        .attach(cors)
}