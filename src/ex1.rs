use bip39::{MnemonicType, Mnemonic, Language};
use sp_core::hexdisplay::HexDisplay;
use sp_core::crypto::Ss58AddressFormat;
use sp_core::crypto::Ss58Codec;
use sp_runtime::{MultiSigner, traits::IdentifyAccount};
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

pub fn new_address() -> JsonValue {
    generate::<sp_core::sr25519::Pair>()
}

fn generate<Pair>() -> JsonValue 
    where Pair: sp_core::Pair, Pair::Public: Into<MultiSigner> {

    // 生成12个助记词   
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);

    let phrase = mnemonic.phrase();

    // 生成地址及助记词
    match Pair::from_phrase(phrase, None) {
        Ok((pair, seed)) => {
            let account = pair.public().into().into_account();
            json!({
                "data": {
                    "phrase": phrase,
                    "seed": format!("0x{}", HexDisplay::from(&seed.as_ref())),
                    "public": format!("0x{}", HexDisplay::from(&pair.public().as_ref())),
                    "ss58Address": account.to_ss58check(),
                    "addresses": 
                        {
                            // Acala
                            "acaAddress": account.to_ss58check_with_version(Ss58AddressFormat::AcalaAccount),
                            // Bifrost
                            "bncAddress": account.to_ss58check_with_version(Ss58AddressFormat::BifrostAccount),
                            // Crust
                            //"cruAddress": account.to_ss58check_with_version(Ss58AddressFormat::Custom(45)),
                            // ChainX, chainx用的是ed25519
                            //"pcxAddress": account.to_ss58check_with_version(Ss58AddressFormat::ChainXAccount),
                            // Polkadot
                            "dotAddress": account.to_ss58check_with_version(Ss58AddressFormat::PolkadotAccount),
                            // Kusama
                            "ksmAddress": account.to_ss58check_with_version(Ss58AddressFormat::KusamaAccount),
                            // Edgeware
                            "edgAddress": account.to_ss58check_with_version(Ss58AddressFormat::EdgewareAccount),
                            // Darwinia
                            "cringAddress": account.to_ss58check_with_version(Ss58AddressFormat::DarwiniaAccount),
                            // Plasm
                            "plmAddress": account.to_ss58check_with_version(Ss58AddressFormat::PlasmAccount),
                            // Kulupu
                            "klpAddress": account.to_ss58check_with_version(Ss58AddressFormat::KulupuAccount),
                            // Stafi
                            "fisAddress": account.to_ss58check_with_version(Ss58AddressFormat::StafiAccount),
                        },
                },
                "msg": "success"
            })
            
        }
        Err(err) => {
            json!({
                "data": {},
                "msg": format!("{:?}", err),
            })
        }
    }
}