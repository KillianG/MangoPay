use std::fmt::Error;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
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
    pub fn create_card_registration(self: &Mangopay, body: &CardRegistrationBody) -> Result<CardRegistrationResponse, reqwest::Error> {
        let user_response = match self.create_post_api_call("cardregistrations".parse().unwrap()).json(body).send() {
            Ok(resp) => resp,
            Err(e) => return Err(e)
        };
        match user_response.json() {
            Ok(val) => Ok(val),
            Err(e) => Err(e)
        }
    }

    pub fn update_card_registration(self: &Mangopay, card_registration_id: String, body: &UpdateCardRegistrationBody) -> Result<CardRegistrationResponse, reqwest::Error> {
        let user_response = match self.create_put_api_call(format!("cardregistrations/{}", card_registration_id)).json(body).send() {
            Ok(resp) => resp,
            Err(e) => return Err(e)
        };
        match user_response.json() {
            Ok(val) => Ok(val),
            Err(e) => Err(e)
        }
    }

    pub fn get_card_registration(self: &Mangopay, card_registration_id: String) -> Result<CardRegistrationResponse, reqwest::Error> {
        let user_response = match self.make_get_api_call(format!("cardregistrations/{}", card_registration_id)) {
            Ok(resp) => resp,
            Err(e) => return Err(e)
        };
        match user_response.json() {
            Ok(val) => Ok(val),
            Err(e) => Err(e)
        }
    }
}

mod test {
    use crate::card::{CardRegistrationBody, UpdateCardRegistrationBody};
    use crate::Mangopay;
    use crate::user::CreateUserBody;
    use crate::wallet::{CreateWallet, ListWallets, Wallet};

    #[test]
    fn create_card_registration() {
        let client_id: &str = env!("MANGO_CLIENT_ID");
        let api_key: &str = env!("MANGO_API_KEY");
        let mangop: Mangopay = Mangopay::init(client_id.to_owned(), api_key.to_owned(), "https://api.sandbox.mangopay.com/v2.01/".to_string());

        let user_id = mangop.create_user(&CreateUserBody {
            first_name: "Killian".parse().unwrap(),
            last_name: "G".parse().unwrap(),
            email: "killian.g@gmail.com".parse().unwrap(),
            user_category: "Payer".parse().unwrap(),
            tag: "TestUser".to_string(),
            terms_and_conditions_accepted: true,
        }).unwrap().id;

        let mut card_registration_result = mangop.create_card_registration(&CardRegistrationBody{
            tag: "Tag".to_string(),
            user_id: user_id.to_owned(),
            currency: "EUR".to_string(),
            card_type: "CB_VISA_MASTERCARD".to_string()
        }).unwrap();
        assert_eq!(card_registration_result.card_type, "CB_VISA_MASTERCARD");
        assert_eq!(card_registration_result.currency, "EUR");
        assert_eq!(card_registration_result.user_id, user_id.to_owned());

        card_registration_result = mangop.get_card_registration(card_registration_result.id).unwrap();
        assert_eq!(card_registration_result.card_type, "CB_VISA_MASTERCARD");
        assert_eq!(card_registration_result.currency, "EUR");
        assert_eq!(card_registration_result.user_id, user_id.to_owned());
        card_registration_result = mangop.update_card_registration(card_registration_result.id, &UpdateCardRegistrationBody {
            tag: "".to_string(),
            registration_data: "registrationdata".to_string()
        }).unwrap();
        assert_eq!(card_registration_result.card_type, "CB_VISA_MASTERCARD");
        assert_eq!(card_registration_result.currency, "EUR");
        assert_eq!(card_registration_result.user_id, user_id.to_owned());
        assert_eq!(card_registration_result.registration_data, "registrationdata".to_string());
    }
}