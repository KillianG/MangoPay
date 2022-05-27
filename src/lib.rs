pub mod user;
pub mod wallet;

use reqwest::blocking::{Client, RequestBuilder, Response};
use serde_json::Value;

#[macro_use]
extern crate serde_derive;

pub struct Mangopay {
    client_id: String,
    api_key: String,
    authorization_token: String
}

pub type GetCardsResponse = Vec<CardResponse>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardResponse {
    #[serde(rename = "ExpirationDate")]
    pub expiration_date: String,
    #[serde(rename = "Alias")]
    pub alias: String,
    #[serde(rename = "CardType")]
    pub card_type: String,
    #[serde(rename = "CardProvider")]
    pub card_provider: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Product")]
    pub product: String,
    #[serde(rename = "BankCode")]
    pub bank_code: String,
    #[serde(rename = "Active")]
    pub active: bool,
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "Validity")]
    pub validity: String,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Tag")]
    pub tag: Value,
    #[serde(rename = "CreationDate")]
    pub creation_date: i64,
    #[serde(rename = "Fingerprint")]
    pub fingerprint: String,
}

impl Mangopay {

    pub fn init(client_id: String, api_key: String) -> Self {
        let formatted_token = format!("{}:{}", client_id, api_key);
        let authorization_token = base64::encode(formatted_token);
        let mango_infos: Mangopay = Mangopay {client_id, api_key, authorization_token};
        return mango_infos;
    }

    fn make_get_api_call(self: &Mangopay, api_url: String) -> reqwest::Result<Response> {
        let client: Client = reqwest::blocking::Client::new();
        let mango_api_base_url: String = format!("https://api.sandbox.mangopay.com/v2.01/{}", self.client_id);
        client.get(format!("{}/{}", mango_api_base_url, api_url))
            .header("Authorization", format!("Basic {}", self.authorization_token))
            .send()
    }

    fn create_post_api_call(self: &Mangopay, api_url: String) -> RequestBuilder {
        let client: Client = reqwest::blocking::Client::new();
        let mango_api_base_url: String = format!("https://api.sandbox.mangopay.com/v2.01/{}", self.client_id);
        client.post(format!("{}/{}", mango_api_base_url, api_url))
            .header("Authorization", format!("Basic {}", self.authorization_token))
    }
}

#[cfg(test)]
mod tests {
    use crate::Mangopay;

    #[test]
    fn init() {
        let client_id: String = "client_id".to_string();
        let api_key: String = "api_key".to_string();
        let mangop: Mangopay = Mangopay::init(client_id.to_owned(), api_key.to_owned());
        assert_eq!(mangop.api_key, "api_key");
        assert_eq!(mangop.client_id, "client_id");
        let formatted_token = format!("{}:{}", &client_id, &api_key);
        let authorization_token = base64::encode(formatted_token);
        assert_eq!(mangop.authorization_token, authorization_token);
    }
}
