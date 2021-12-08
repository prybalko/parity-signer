use db_handling::{db_transactions::TrDbColdStub, helpers::{genesis_hash_in_specs, get_general_verifier, open_db, try_get_valid_current_verifier}};
use definitions::{error::{ErrorSigner, ErrorSource, IncomingMetadataSourceSigner, InputSigner, GeneralVerifierForContent, MetadataSource, Signer, TransferContent}, keyring::VerifierKey, metadata::MetaValues, network_specs::{ValidCurrentVerifier, Verifier}, history::{Event, MetaValuesDisplay}, qr_transfers::ContentLoadMeta};

use crate::cards::{Action, Card, Warning};
use crate::check_signature::pass_crypto;
use crate::helpers::accept_meta_values;

enum FirstCard {
    WarningCard(String),
    VerifierCard(String),
}

pub fn load_metadata(data_hex: &str, database_name: &str) -> Result<String, ErrorSigner> {
    let checked_info = pass_crypto(&data_hex, TransferContent::LoadMeta)?;
    let (meta, genesis_hash) = ContentLoadMeta::from_vec(&checked_info.message).meta_genhash::<Signer>()?;
    let meta_values = match MetaValues::from_vec_metadata(&meta) {
        Ok(a) => a,
        Err(e) => return Err(<Signer>::faulty_metadata(e, MetadataSource::Incoming(IncomingMetadataSourceSigner::ReceivedData))),
    };
    let general_verifier = get_general_verifier(&database_name)?;
    let verifier_key = VerifierKey::from_parts(&genesis_hash.to_vec());
    let valid_current_verifier = match try_get_valid_current_verifier (&verifier_key, &database_name)? {
        Some(a) => {
            if let Some(_) = genesis_hash_in_specs(&verifier_key, &open_db::<Signer>(&database_name)?)? {a}
            else {return Err(ErrorSigner::Input(InputSigner::LoadMetaNoSpecs{name: meta_values.name, valid_current_verifier: a, general_verifier}))}
        },
        None => return Err(ErrorSigner::Input(InputSigner::LoadMetaUnknownNetwork{name: meta_values.name})),
    };
    let mut stub = TrDbColdStub::new();
    let mut index = 0;
    
    let first_card = match checked_info.verifier {
        Verifier(None) => {
            stub = stub.new_history_entry(Event::Warning(Warning::NotVerified.show()));
            match valid_current_verifier {
                ValidCurrentVerifier::Custom(Verifier(None)) => (),
                ValidCurrentVerifier::Custom(Verifier(Some(verifier_value))) => return Err(ErrorSigner::Input(InputSigner::NeedVerifier{name: meta_values.name, verifier_value})),
                ValidCurrentVerifier::General => match general_verifier {
                   Verifier(None) => (),
                   Verifier(Some(verifier_value)) => return Err(ErrorSigner::Input(InputSigner::NeedGeneralVerifier{content: GeneralVerifierForContent::Network{name: meta_values.name}, verifier_value})),
                },
            }
            FirstCard::WarningCard(Card::Warning(Warning::NotVerified).card(&mut index,0))
        },
        Verifier(Some(ref new_verifier_value)) => {
            match valid_current_verifier {
                ValidCurrentVerifier::Custom(a) => {
                    if checked_info.verifier != a {
                        match a {
                            Verifier(None) => return Err(ErrorSigner::Input(InputSigner::LoadMetaSetVerifier{name: meta_values.name, new_verifier_value: new_verifier_value.to_owned()})),
                            Verifier(Some(old_verifier_value)) => return Err(ErrorSigner::Input(InputSigner::LoadMetaVerifierChanged{name: meta_values.name, old_verifier_value, new_verifier_value: new_verifier_value.to_owned()})),
                        }
                    }
                },
                ValidCurrentVerifier::General => {
                    if checked_info.verifier != general_verifier {
                        match general_verifier {
                            Verifier(None) => return Err(ErrorSigner::Input(InputSigner::LoadMetaSetGeneralVerifier{name: meta_values.name, new_general_verifier_value: new_verifier_value.to_owned()})),
                            Verifier(Some(old_general_verifier_value)) => return Err(ErrorSigner::Input(InputSigner::LoadMetaGeneralVerifierChanged{name: meta_values.name, old_general_verifier_value, new_general_verifier_value: new_verifier_value.to_owned()})),
                        }
                    }
                },
            }
            FirstCard::VerifierCard(Card::Verifier(new_verifier_value).card(&mut index,0))
        },
    };
    if accept_meta_values(&meta_values, &database_name)? {
        stub = stub.add_metadata(&meta_values);
        let checksum = stub.store_and_get_checksum(&database_name)?;
        let meta_display = MetaValuesDisplay::get(&meta_values);
        let meta_card = Card::Meta(meta_display).card(&mut index, 0);
        let action_card = Action::Stub(checksum).card();
        match first_card {
            FirstCard::WarningCard(warning_card) => Ok(format!("{{\"warning\":[{}],\"meta\":[{}],{}}}", warning_card, meta_card, action_card)),
            FirstCard::VerifierCard(verifier_card) => Ok(format!("{{\"verifier\":[{}],\"meta\":[{}],{}}}", verifier_card, meta_card, action_card)),
        }
    }
    else {return Err(ErrorSigner::Input(InputSigner::MetadataKnown{name: meta_values.name, version: meta_values.version}))}
}
