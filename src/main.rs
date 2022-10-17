use std::{path::PathBuf, fs};

use serde_derive::{Serialize, Deserialize};
use uuid::Uuid;
use warp::Filter;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo{
    id: String,
    text: String,
    checked: bool,
}

#[tokio::main]
async fn main() {
    
    let create_route = warp::path!("create" / String).map(|text:String| {
        let id = Uuid::new_v4();

        let todo = Todo { 
            id: "".to_string(),
            text, 
            checked: false };

        let todo_path:PathBuf = ["./todos".to_string(), id.to_string()].iter().collect();

        fs::write(todo_path,serde_json::to_string(&todo).unwrap()).unwrap(); //write returns the result

        return id.to_string();
    });


    warp::serve(create_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
