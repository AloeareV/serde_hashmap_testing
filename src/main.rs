#[derive(Debug, serde :: Deserialize, serde :: Serialize)]
pub struct Consensus {
    pub chaintip: String,
    pub nextblock: String,
}
#[derive(Debug, serde :: Deserialize, serde :: Serialize)]
pub struct Enforce {
    pub found: rust_decimal::Decimal,
    pub required: rust_decimal::Decimal,
    pub status: bool,
    pub window: rust_decimal::Decimal,
}
#[derive(Debug, serde :: Deserialize, serde :: Serialize)]
pub struct Reject {
    pub found: rust_decimal::Decimal,
    pub required: rust_decimal::Decimal,
    pub status: bool,
    pub window: rust_decimal::Decimal,
}
#[derive(Debug, serde :: Deserialize, serde :: Serialize)]
pub struct Softforks {
    pub enforce: Enforce,
    pub id: String,
    pub reject: Reject,
    pub version: rust_decimal::Decimal,
}
#[derive(Debug, serde :: Deserialize, serde :: Serialize)]
pub struct Upgrade {
    pub activationheight: rust_decimal::Decimal,
    pub info: String,
    pub name: String,
    pub status: String,
}
#[derive(Debug, serde :: Deserialize, serde :: Serialize)]
pub struct Getblockchaininfo {
    pub bestblockhash: String,
    pub blocks: rust_decimal::Decimal,
    pub chain: String,
    pub chainwork: String,
    pub commitments: rust_decimal::Decimal,
    pub consensus: Consensus,
    pub difficulty: rust_decimal::Decimal,
    pub estimatedheight: rust_decimal::Decimal,
    pub headers: rust_decimal::Decimal,
    pub initial_block_download_complete: Option<bool>,
    pub size_on_disk: rust_decimal::Decimal,
    pub softforks: Vec<Softforks>,
    pub upgrades: std::collections::HashMap<String, Upgrade>,
    pub verificationprogress: rust_decimal::Decimal,
}

fn main() {
    let file = std::fs::read("exampledata.json").unwrap();
    let test: Getblockchaininfo =
        serde_json::from_str(&String::from_utf8_lossy(&file)).unwrap();
    println!("{:#?}", test);
    dbg!("{:#?}", type_name_of_val(test.upgrades));
}

fn type_name_of_val<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}
