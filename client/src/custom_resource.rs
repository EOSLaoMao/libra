use types::{
    access_path::{AccessPath, Accesses},
    account_address::AccountAddress,
    account_state_blob::AccountStateBlob,
    language_storage::StructTag,
};
use abi_deserializer::AbiDeserializer;
use canonical_serialization::SimpleDeserializer;
use failure::prelude::*;
use std::{collections::BTreeMap, convert::TryInto, fs::File, io::prelude::*};


pub fn get_account_resource(
    address: AccountAddress,
    account_state: &Option<AccountStateBlob>,
    params: &[&str],
) -> Result<String> {
    ensure!(
        params.len() == 6,
        "Invalid number of arguments to get account resource"
    );

    let mut file = File::open(params[5])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    get_account_resource_string(account_state, address, params[3], params[4], contents.as_str())
}

pub fn get_account_resource_string(
    account_state: &Option<AccountStateBlob>,
    address: AccountAddress,
    module_name: &str,
    resource_name: &str,
    abi_str: &str,
) -> Result<String> {
    match account_state {
        Some(blob) => {
            let account_btree: BTreeMap<Vec<u8>, Vec<u8>> = blob.try_into()?;
            let ap = account_resource_path(address, module_name, resource_name);
            match account_btree.get(&ap) {
                Some(bytes) => {
                    let de = AbiDeserializer::from_str(abi_str)?;
                    let mut deserializer = SimpleDeserializer::new(&bytes);
                    de.deserialize_to_string_pretty(&mut deserializer)
                }
                None => bail!("No data for {:?}", ap),
            }
        }
        None => bail!("None account state"),
    }
}

pub fn account_resource_path(address: AccountAddress, module_name: &str, resource_name: &str) -> Vec<u8> {
    AccessPath::resource_access_vec(
        &StructTag {
            address: address,
            module: module_name.to_string(),
            name: resource_name.to_string(),
            type_params: vec![],
        },
        &Accesses::empty(),
    )
}
