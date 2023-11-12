use serde::{Deserialize, Serialize};
use bson::Bson;
use serde::de::{self, Deserializer};

#[derive(Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "_id", deserialize_with = "bson_object_id_to_string")]
    pub id: String,
    pub name: String,
    pub description: String,
}

// Fungsi custom untuk deserialisasi `ObjectId` menjadi `String`
fn bson_object_id_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let bson = Bson::deserialize(deserializer)?;
    match bson {
        Bson::ObjectId(oid) => Ok(oid.to_string()),
        _ => Err(de::Error::custom("expected ObjectId")),
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateItem {
    pub name: String,
    pub description: String,
}
