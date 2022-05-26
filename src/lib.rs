use std::borrow::Borrow;
use reqwest::blocking::{Client, RequestBuilder, Response};
use serde::de::Error;
use serde::Serialize;
use serde_json::{json, Value};

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserBody {
    #[serde(rename = "FirstName")]
    pub first_name: String,
    #[serde(rename = "LastName")]
    pub last_name: String,
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "UserCategory")]
    pub user_category: String,
    #[serde(rename = "Tag")]
    pub tag: String,
    #[serde(rename = "TermsAndConditionsAccepted")]
    pub terms_and_conditions_accepted: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "Address")]
    pub address: Address,
    #[serde(rename = "FirstName")]
    pub first_name: String,
    #[serde(rename = "LastName")]
    pub last_name: String,
    #[serde(rename = "Birthday")]
    pub birthday: Value,
    #[serde(rename = "Nationality")]
    pub nationality: Value,
    #[serde(rename = "CountryOfResidence")]
    pub country_of_residence: Value,
    #[serde(rename = "Occupation")]
    pub occupation: Value,
    #[serde(rename = "IncomeRange")]
    pub income_range: Value,
    #[serde(rename = "ProofOfIdentity")]
    pub proof_of_identity: Value,
    #[serde(rename = "ProofOfAddress")]
    pub proof_of_address: Value,
    #[serde(rename = "Capacity")]
    pub capacity: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Tag")]
    pub tag: String,
    #[serde(rename = "CreationDate")]
    pub creation_date: i64,
    #[serde(rename = "PersonType")]
    pub person_type: String,
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "KYCLevel")]
    pub kyclevel: String,
    #[serde(rename = "TermsAndConditionsAccepted")]
    pub terms_and_conditions_accepted: bool,
    #[serde(rename = "TermsAndConditionsAcceptedDate")]
    pub terms_and_conditions_accepted_date: i64,
    #[serde(rename = "UserCategory")]
    pub user_category: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[serde(rename = "AddressLine1")]
    pub address_line1: Value,
    #[serde(rename = "AddressLine2")]
    pub address_line2: Value,
    #[serde(rename = "City")]
    pub city: Value,
    #[serde(rename = "Region")]
    pub region: Value,
    #[serde(rename = "PostalCode")]
    pub postal_code: Value,
    #[serde(rename = "Country")]
    pub country: Value,
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

    pub fn create_user(self: &Mangopay, user_infos: &CreateUserBody) -> Option<User> {
        let user_response = match self.create_post_api_call("users/natural/".parse().unwrap()).json(user_infos).send() {
            Ok(resp) => resp,
            Err(_) => return None
        };
        match user_response.json() {
            Ok(val) => Some(val),
            Err(_) => None
        }
    }

    pub fn get_user(self: &Mangopay, user_id: String) -> Option<User> {
        let user_response = match self.make_get_api_call(format!("users/{}", user_id)) {
            Ok(resp) => resp,
            Err(_) => return None
        };
        match user_response.json() {
            Ok(val) => Some(val),
            Err(_) => None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{CreateUserBody, User, Mangopay};

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

    #[test]
    fn crud_on_user() {
        let client_id: &str = env!("MANGO_CLIENT_ID");
        let api_key: &str = env!("MANGO_API_KEY");
        let mangop: Mangopay = Mangopay::init(client_id.to_owned(), api_key.to_owned());

        let mut user = mangop.create_user(&CreateUserBody {
            first_name: "Killian".parse().unwrap(),
            last_name: "G".parse().unwrap(),
            email: "killian.g@gmail.com".parse().unwrap(),
            user_category: "Payer".parse().unwrap(),
            tag: "TestUser".to_string(),
            terms_and_conditions_accepted: true,
        }).unwrap();
        assert_eq!(user.first_name, "Killian");
        assert_eq!(user.email, "killian.g@gmail.com");

        let user = mangop.get_user(user.id).unwrap();
        assert_eq!(user.first_name, "Killian");
        assert_eq!(user.email, "killian.g@gmail.com");
    }
}
