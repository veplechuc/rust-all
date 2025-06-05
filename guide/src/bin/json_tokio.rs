use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Receive type-checked JSON

    let todos: Vec<Todo> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .json()
        .await?;
    println!("TODOs for USER ID 1");
    println!("--------------------");
    for ele in todos.iter() {
        println!("{:#?}", ele);
    }

    // Send and receive type-checked JSON

    let new_todo = Todo {
        user_id: 1,
        id: None,
        title: "Some text added to TODOS".to_owned(),
        completed: false,
    };

    let new_todo: Todo = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&new_todo)
        .send()
        .await?
        .json()
        .await?;

    println!("NEW TODO ");
    println!("--------------------");

    println!("{:#?}", new_todo);

    // // Send and receive arbitrary JSON

    let new_todo: serde_json::Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&serde_json::json!({
            "userId": 1,
            "title": "Some text added to TODOS-Arbitrary Json".to_owned(),
            "completed": false
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("NEW TODO - arbitrary not using Struct ");
    println!("--------------------");
    println!("{:#?}", new_todo);

    Ok(())
}
