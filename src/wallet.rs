use crate::Mangopay;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWallet {
    #[serde(rename = "Owners")]
    pub owners: Vec<String>,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "Tag")]
    pub tag: String,
}

pub type ListWallets = Vec<Wallet>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Owners")]
    pub owners: Vec<String>,
    #[serde(rename = "Balance")]
    pub balance: Balance,
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "FundsType")]
    pub funds_type: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Tag")]
    pub tag: Value,
    #[serde(rename = "CreationDate")]
    pub creation_date: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "Amount")]
    pub amount: i64,
}

impl Mangopay {

    pub fn create_wallet(self: &Mangopay, user_id: String) -> Option<Wallet> {
        let create_wallet = CreateWallet {
            owners: vec![user_id],
            description: "Mecen wallet".to_string(),
            currency: "EUR".to_string(),
            tag: "Created by Mecen backend".to_string()
        };

        let wallet_response = match self.create_post_api_call("wallets/".to_string()).json(&create_wallet).send() {
            Ok(val) => val,
            Err(_) => return None
        };

        match wallet_response.json() {
            Ok(val) => Some(val),
            Err(_) => None
        }
    }

    pub fn list_wallets(self: &Mangopay, user_id: String) -> Option<ListWallets> {

        let wallet_response = match self.make_get_api_call(format!("users/{}/wallets", user_id)){
            Ok(val) => val,
            Err(_) => return None
        };

        match wallet_response.json() {
            Ok(val) => Some(val),
            Err(_) => None
        }
    }
}

mod test {
    use crate::Mangopay;
    use crate::user::CreateUserBody;
    use crate::wallet::ListWallets;

    #[test]
    fn create_wallet() {
        let client_id: &str = env!("MANGO_CLIENT_ID");
        let api_key: &str = env!("MANGO_API_KEY");
        let mangop: Mangopay = Mangopay::init(client_id.to_owned(), api_key.to_owned());

        let user_id = mangop.create_user(&CreateUserBody {
            first_name: "Killian".parse().unwrap(),
            last_name: "G".parse().unwrap(),
            email: "killian.g@gmail.com".parse().unwrap(),
            user_category: "Payer".parse().unwrap(),
            tag: "TestUser".to_string(),
            terms_and_conditions_accepted: true,
        }).unwrap().id;
        let wallet: Wallet = mangop.create_wallet(user_id.to_string()).unwrap();
        assert_eq!(wallet.balance.amount, 0);
        assert_eq!(wallet.balance.currency, "EUR");
        assert_eq!(wallet.owners.get(0).unwrap(), &user_id);

        let list_wallets: ListWallets = mangop.list_wallets(user_id.to_string()).unwrap();
        assert_eq!(list_wallets[0].balance.amount, 0);
        assert_eq!(list_wallets[0].balance.currency, "EUR");
        assert_eq!(list_wallets[0].owners.get(0).unwrap(), &user_id);
    }
}