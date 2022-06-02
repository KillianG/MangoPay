# WORK IN PROGRESS
MangoPay library for rust

### How to use (by examples)

```rust
let mango: Mangopay = Mangopay::init(
    env!("MANGO_CLIENT_ID").parse().unwrap(),
    env!("MANGO_API_KEY").parse().unwrap(),
    "https://api.sandbox.mangopay.com/v2.01/".to_string()
);

let user_infos = CreateUserBody {
    first_name: "firstName".to_string(),
    last_name: "lastNme".to_string(),
    email: "john@doe.com".to_string(),
    user_category: "Payer".to_string(),
    tag: "Tagged".to_string(),
    terms_and_conditions_accepted: true,
};

let user = mango.create_user(&user_infos).unwarp();

let wallet: Wallet = mango.create_wallet(CreateWallet{
    owners: vec![user.id],
    description: "Description".to_string(),
    currency: "EUR".to_string(),
    tag: "Tagged".to_string()
})
```