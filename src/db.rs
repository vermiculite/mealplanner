use mongodb::{Client, options::ClientOptions, Database};
use futures::executor::block_on;

pub fn get_db() -> Database {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut client_options = block_on(ClientOptions::parse(&db_url)).expect("The options werent cool.");
    let client = Client::with_options(client_options).expect("It will have client options");
    client.database("mealplanner")
}
