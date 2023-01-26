use serde::{Deserialize, de::DeserializeOwned};
use crate::request::auth;

pub const BASE_ELITEPROSPECTS_URL : &str = "https://api.eliteprospects.com/v1";
pub const MAX_REQ_LIMIT : usize = 1000; // EP-API hard cap on responses sent per GET

// Carrier struct with a single 'data'  field to match GET array response structure from EP-API
#[derive(Deserialize)]
pub struct EliteProspectsRequestVector<T> {
    pub data : Vec<Option<T>>
}

// GET method on EP-API 'endpoint'
// - returns deserialized JSON response as instance of type T (T must impl DeserializeOwned)
pub async fn get<T>(endpoint : &str) -> Result<Vec<T>, reqwest::Error> where T: DeserializeOwned {
    let empty_fields = Vec::new();
    get_with_fields::<T>(endpoint, empty_fields).await
}

// GET method on EP-API 'endpoint'
// 'fields' is a list of fields to send alongside endpoint (eg: "id=503030")
// - returns Vec of deserialized JSON responses as instances of type T (T must impl DeserializeOwned)
pub async fn get_with_fields<T>(endpoint : &str, fields : Vec<String>) -> Result<Vec<T>, reqwest::Error>  where T : DeserializeOwned {
    let mut req_url = auth::with_auth_code_field(format!("{BASE_ELITEPROSPECTS_URL}/{endpoint}?"));
    for field in fields.iter() {
        req_url.push('&');
        req_url.push_str(field);
    }

    dbg!(&req_url);
    let res = reqwest::get(req_url).await?.error_for_status()?
    .json::<EliteProspectsRequestVector<T>>().await?;

    Ok(
        res.data.into_iter().filter_map(|t| t).collect()
    )
}