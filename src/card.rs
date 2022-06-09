use std::fmt::Error;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::{json, Value};
use crate::Mangopay;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRegistrationBody {
    #[serde(rename = "Tag")]
    pub tag: String,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "CardType")]
    pub card_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRegistrationResponse {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Tag")]
    pub tag: Value,
    #[serde(rename = "CreationDate")]
    pub creation_date: i64,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "AccessKey")]
    pub access_key: String,
    #[serde(rename = "PreregistrationData")]
    pub preregistration_data: String,
    #[serde(rename = "RegistrationData")]
    pub registration_data: Value,
    #[serde(rename = "CardId")]
    pub card_id: Value,
    #[serde(rename = "CardType")]
    pub card_type: String,
    #[serde(rename = "CardRegistrationURL")]
    pub card_registration_url: String,
    #[serde(rename = "ResultCode")]
    pub result_code: Value,
    #[serde(rename = "ResultMessage")]
    pub result_message: Value,
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "Status")]
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCardRegistrationBody {
    #[serde(rename = "Tag")]
    pub tag: String,
    #[serde(rename = "RegistrationData")]
    pub registration_data: String,
}

impl Mangopay {
    pub async fn create_card_registration(self: &Mangopay, body: &CardRegistrationBody) -> Result<CardRegistrationResponse, reqwest::Error> {
        let user_response = match self.create_post_api_call("cardregistrations".parse().unwrap()).json(body).send().await {
            Ok(resp) => resp,
            Err(e) => return Err(e)
        };
        let json_response =  user_response.json().await?;
        return Ok(json_response);
    }

    pub async fn update_card_registration(self: &Mangopay, card_registration_id: String, body: &UpdateCardRegistrationBody) -> Result<CardRegistrationResponse, reqwest::Error> {
        let user_response = match self.create_put_api_call(format!("cardregistrations/{}", card_registration_id)).json(body).send().await {
            Ok(resp) => resp,
            Err(e) => return Err(e)
        };
        let json_response =  user_response.json().await?;
        return Ok(json_response);
    }

    pub async fn get_card_registration(self: &Mangopay, card_registration_id: String) -> Result<CardRegistrationResponse, reqwest::Error> {
        let user_response = match self.make_get_api_call(format!("cardregistrations/{}", card_registration_id)).await {
            Ok(resp) => resp,
            Err(e) => return Err(e)
        };
        let json_response =  user_response.json().await?;
        return Ok(json_response);
    }
}