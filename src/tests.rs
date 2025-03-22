//cargo tarpaulin --ignore-tests = 79.23% coverage, 290/366 lines covered
//2 tests fail due to the new way that cosmwasm deals with bech32 addresses (cosmwasmaddr1...). Uncomment test address and comment out legit addresses in contract.rs for testing.
use crate::contract::{execute, instantiate, migrate, query};
use crate::msg::{
    AllListingsResponse, ArbitrationListingsResponse, ExecuteMsg, InstantiateMsg,
    ListingCountResponse, ListingResponse, MigrateMsg, QueryMsg, SearchListingsResponse,
    ProfileResponse,
};
use cosmwasm_std::testing::{message_info, mock_dependencies, mock_env};
use cosmwasm_std::{attr, coin, from_json, Response};

const JUNO: &str = "ujuno";
const IPFS_LINK: &str =
    "https://gateway.pinata.cloud/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT";

//Test that the contract is instantiated correctly
#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let instantiator = deps.api.addr_make("instantiator");
    let info = message_info(&instantiator, &[]);

    let msg = InstantiateMsg {};
    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![
            attr("action", "instantiate"),
            attr("admin", instantiator.to_string())
        ]
    );
}

//Test that the contract can be migrated
#[test]
fn migrate_works() {
    //instantiate
    let mut deps = mock_dependencies();
    let env = mock_env();
    let instantiator = deps.api.addr_make("instantiator");
    let info = message_info(&instantiator, &[]);

    let msg = InstantiateMsg {};
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    //migrate
    let msg = MigrateMsg {};
    let _res: Response = migrate(deps.as_mut(), mock_env(), msg).unwrap();
}

//Test that a listing can be created and then queried
#[test]

