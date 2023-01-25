use serde::{Deserialize, de::DeserializeOwned};
use crate::request::auth;


const BASE_ELITEPROSPECTS_URL : &str = "https://api.eliteprospects.com/v1";
const MAX_REQ_LIMIT : usize = 1000; // EP-API hard cap on responses sent per GET


// Carrier struct with a single 'data'  field to match GET array response structure from EP-API
#[derive(Deserialize)]
pub struct EliteProspectsRequestVector<T> {
    pub data : Vec<Option<T>>
}

// GET method on EP-API 'endpoint'
// - returns deserialized JSON response as instance of type T (T must impl DeserializeOwned)
pub async fn get<T>(endpoint : &str) -> Result<Vec<T>, reqwest::Error> where T: DeserializeOwned {
    get_with_fields::<T>(endpoint, Vec::<&str>::new()).await
}

// GET method on EP-API 'endpoint'
// 'fields' is a list of fields to send alongside endpoint (eg: "id=503030")
// - returns Vec of deserialized JSON responses as instances of type T (T must impl DeserializeOwned)
pub async fn get_with_fields<T>(endpoint : &str, fields : Vec<&str>) -> Result<Vec<T>, reqwest::Error>  where T : DeserializeOwned {
    let mut req_url = auth::with_auth_code_field(format!("{BASE_ELITEPROSPECTS_URL}/{endpoint}?"));
    for field in fields.iter() {
        req_url.push('&');
        req_url.push_str(field);
    }

    let res = reqwest::get(req_url).await?.error_for_status()?
    .json::<EliteProspectsRequestVector<T>>().await?;

    Ok(
        res.data.into_iter().filter_map(|t| t).collect()
    )
}

// GET method on EP-API 'endpoint', returning all possible results.
// Repeatedly calls GET endpoint until all available data is received.
// - returns Vec of deserialized JSON responses as instances of type T (T must impl DeserializeOwned)
pub async fn get_all<T>(endpoint : &str) -> Result<Vec<T>, reqwest::Error> where T: DeserializeOwned {
    get_all_with_fields::<T>(endpoint, Vec::<&str>::new()).await
}

// GET method on EP-API 'endpoint', returning all possible results.
// Repeatedly calls GET endpoint until all available data is received.
// 'fields' is a list of fields to send alongside endpoint (eg: "id=503030")
// - returns Vec of deserialized JSON responses as instances of type T (T must impl DeserializeOwned)
pub async fn get_all_with_fields<T>(endpoint : &str, fields : Vec<&str>) -> Result<Vec<T>, reqwest::Error> where T: DeserializeOwned {

    let mut finished = false;
    let mut offset = 0;

    let mut all_records = Vec::new();

    while !finished {
        let mut req_url = auth::with_auth_code_field(format!("{BASE_ELITEPROSPECTS_URL}/{endpoint}?limit={MAX_REQ_LIMIT}&sort=-updatedAt&offset={offset}"));
        for field in fields.iter() {
            req_url.push('&');
            req_url.push_str(field);
        }

        dbg!(&req_url);
        let res = reqwest::get(req_url).await?.error_for_status()?
        .json::<EliteProspectsRequestVector<T>>().await?.data;
        
        if res.len() < MAX_REQ_LIMIT {
            finished = true;
        }
        offset += MAX_REQ_LIMIT;
        all_records.extend(res);
    }
    Ok(all_records.into_iter().filter_map(|t| t).collect())
}
