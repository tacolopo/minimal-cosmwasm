use cosmwasm_std::{
    coin, entry_point, to_json_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Order,
    Response, StdError, StdResult,
};
use cw2::{get_contract_version, set_contract_version};
use cw_storage_plus::Bound;
use is_false::is_false;
use std::env;

use crate::coin_helpers::assert_sent_exact_coin;
use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg,
};
use crate::state::{
    Config, Listing, CONFIG, LAST_LISTING_ID, LISTING, LISTING_COUNT, LISTING_TITLES,
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
//Admin & fee wallet
const ADMIN: &str = "juno107zhxnyyvrskwv8vnqhrmfzkm8xlzphksuvdpz";


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let config = Config {
        admin: info.sender.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    LAST_LISTING_ID.save(deps.storage, &0)?;
    LISTING_COUNT.save(deps.storage, &0)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", info.sender.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateProfile { profile_name } => {
            execute_create_profile(deps, env, info, profile_name)
        }
    }
}
pub fn execute_create_profile(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    profile_name: String,
) -> Result<Response, ContractError> {


    Ok(Response::new()
        .add_attribute("action", "create_profile")
        .add_attribute("profile_name", profile_name))
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Listing { listing_id } => query_listing(deps, env, listing_id),
    }
}


#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let ver = get_contract_version(deps.storage)?;
    if ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }
    //canonical way from official docs https://docs.cosmwasm.com/docs/1.0/smart-contracts/migration/#migrate-which-updates-the-version-only-if-newer
    #[allow(clippy::cmp_owned)]
    if ver.version > (*CONTRACT_VERSION).to_string() {
        return Err(StdError::generic_err("Must upgrade from a lower version").into());
    }
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default()
        .add_attribute("action", "migration")
        .add_attribute("version", CONTRACT_VERSION)
        .add_attribute("contract", CONTRACT_NAME))
}
