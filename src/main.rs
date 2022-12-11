#[macro_use] extern crate rocket;

use rand::Rng;

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
    let mut vec_copy:Vec<String> = Vec::new();
    for quote in split_quotes {
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

    format!("{{{}}}", split_quotes[rng.gen_range(0..1000)])
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/quotes", routes![quotes])
        .mount("/quotes", routes![quote])
        .mount("/quotes", routes![random])
}