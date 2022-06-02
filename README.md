# [WIP] MangoPay library for rust

Feel free to help me build this [KillianG/MangoPay](https://github.com/KillianG/MangoPay) :)

## How to use (by examples)

### Initialization
```rust
let mango: Mangopay = Mangopay::init(
    env!("MANGO_CLIENT_ID").parse().unwrap(),
    env!("MANGO_API_KEY").parse().unwrap(),
    "https://api.sandbox.mangopay.com/v2.01/".to_string()
);
```

### User
```Rust
let user_infos = CreateUserBody {
    first_name: "firstName".to_string(),
    last_name: "lastNme".to_string(),
    email: "john@doe.com".to_string(),
    user_category: "Payer".to_string(),
    tag: "Tagged".to_string(),
    terms_and_conditions_accepted: true,
};

let user = mango.create_user(&user_infos).unwrap();
let user_get = mangop.get_user(user.id).unwrap();

```

### Wallet
```Rust
let wallet: Wallet = mango.create_wallet(CreateWallet{
    owners: vec![user.id],
    description: "Description".to_string(),
    currency: "EUR".to_string(),
    tag: "Tagged".to_string()
})

let list_wallets: ListWallets = mangop.list_wallets(user_id.to_string()).unwrap();
```

### Card registration
```rust
let card_registration_result = mango.create_card_registration(&CardRegistrationBody{
    tag: "Tag".to_string(),
    user_id: user_id.to_owned(),
    currency: "EUR".to_string(),
    card_type: "CB_VISA_MASTERCARD".to_string()
}).unwrap();

let card_registration = mango.get_card_registration(card_registration_result.id).unwrap();

let modified_card_registration = mangop.update_card_registration(card_registration.id, &UpdateCardRegistrationBody {
    tag: "".to_string(),
    registration_data: "registrationdata".to_string()
}).unwrap();
```