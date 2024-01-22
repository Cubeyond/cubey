use crate::Error;

use serde::Deserialize;


#[derive(Deserialize)]
struct UUID {
    id: String,
}

pub async fn uuid(name: &String) -> Result<String, Error> {
    let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", name);
    let uuid: UUID = reqwest::get(url).await?.json().await?;

    Ok(uuid.id)
}