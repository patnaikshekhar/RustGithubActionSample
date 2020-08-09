use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    for i in 1..4 {
        let resp = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
        .await?
        .json::<Todo>()
        .await?;
        println!("{:#?}", resp);
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: u16,
    id: u16,
    title: String,
    completed: bool,
}
