use crate::db;
use actix_web::{web, HttpResponse};
use crate::models::{CreateItem, Item};
use mongodb::{
    bson::doc, 
    Collection
};
use futures::stream::StreamExt;

pub async fn create_item(item: web::Json<CreateItem>) -> HttpResponse {
    let db = db::connect().await.unwrap();

    let new_item = doc! {
        "name": &item.name,
        "description": &item.description,
    };

    let collection = db.collection("items");
    match collection.insert_one(new_item, None).await {
        Ok(result) => {
            let response = Item {
                id: result.inserted_id.as_object_id().unwrap().to_hex(),
                name: item.name.clone(),
                description: item.description.clone(),
            };

            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_all_items() -> HttpResponse {
    let db = db::connect().await.unwrap();
    let collection: Collection<Item> = db.collection("items");

    println!("Fetching items from database...");
    let mut cursor = match collection.find(doc! {}, None).await {
        Ok(cursor) => cursor,
        Err(e) => {
            println!("Error fetching data: {}", e);
            return HttpResponse::InternalServerError().body(e.to_string());
        },
    };

    let mut items: Vec<Item> = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(item) => items.push(item),
            Err(e) => println!("Error reading item from cursor: {}", e),
        }
    }

    HttpResponse::Ok().json(items)
}