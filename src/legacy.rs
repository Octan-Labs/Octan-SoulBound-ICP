use crate::*;

// ======================
//      QUERY  CALLS
// ======================

#[query(manual_reply = true)]
#[candid_method(query)]
fn name() -> ManualReply<Option<String>> {
    sbt_name()
}

#[query(manual_reply = true)]
#[candid_method(query)]
fn logo() -> ManualReply<Option<String>> {
    sbt_logo()
}

#[query(manual_reply = true)]
#[candid_method(query)]
fn symbol() -> ManualReply<Option<String>> {
    sbt_symbol()
}

#[query(manual_reply = true)]
#[candid_method(query)]
fn custodians() -> ManualReply<HashSet<Principal>> {
    sbt_custodians()
}

#[query(manual_reply = true)]
#[candid_method(query)]
fn metadata() -> ManualReply<Metadata> {
    sbt_metadata()
}

#[update(name = "setName", guard = "is_canister_custodian")]
#[candid_method(update, rename = "setName")]
fn set_name(name: String) {
    sbt_set_name(name)
}

#[update(name = "setLogo", guard = "is_canister_custodian")]
#[candid_method(update, rename = "setLogo")]
fn set_logo(logo: String) {
    sbt_set_logo(logo)
}

#[update(name = "setSymbol", guard = "is_canister_custodian")]
#[candid_method(update, rename = "setSymbol")]
fn set_symbol(symbol: String) {
    sbt_set_symbol(symbol)
}
#[update(name = "setCustodians", guard = "is_canister_custodian")]
#[candid_method(update, rename = "setCustodians")]
fn set_custodians(custodians: HashSet<Principal>) {
    sbt_set_custodians(custodians)
}

#[query(name = "totalSupply")]
#[candid_method(query, rename = "totalSupply")]
fn total_supply() -> Nat {
    sbt_total_supply()
}

#[query(name = "totalTransactions")]
#[candid_method(query, rename = "totalTransactions")]
fn total_transactions() -> Nat {
    sbt_total_transactions()
}

#[query()]
#[candid_method(query)]
fn cycles() -> Nat {
    sbt_cycles()
}

#[query(name = "totalUniqueHolders")]
#[candid_method(query, rename = "totalUniqueHolders")]
fn total_unique_holders() -> Nat {
    sbt_total_unique_holders()
}

#[query()]
#[candid_method(query)]
fn stats() -> Stats {
    sbt_stats()
}

#[query(name = "supportedInterfaces")]
#[candid_method(query, rename = "supportedInterfaces")]
fn supported_interfaces() -> Vec<SupportedInterface> {
    sbt_supported_interfaces()
}

#[query(name = "balanceOf")]
#[candid_method(query, rename = "balanceOf")]
fn balance_of(owner: Principal) -> Result<Nat, NftError> {
    sbt_balance_of(owner)
}

#[query(name = "ownerOf")]
#[candid_method(query, rename = "ownerOf")]
fn owner_of(token_identifier: TokenIdentifier) -> Result<Option<Principal>, NftError> {
    sbt_owner_of(token_identifier)
}

#[query(name = "operatorOf")]
#[candid_method(query, rename = "operatorOf")]
fn operator_of(token_identifier: TokenIdentifier) -> Result<Option<Principal>, NftError> {
    sbt_operator_of(token_identifier)
}

#[query(name = "ownerTokenMetadata", manual_reply = true)]
#[candid_method(query, rename = "ownerTokenMetadata")]
fn owner_token_metadata(owner: Principal) -> ManualReply<Result<Vec<TokenMetadata>, NftError>> {
    sbt_owner_token_metadata(owner)
}

#[query(name = "operatorTokenMetadata", manual_reply = true)]
#[candid_method(query, rename = "operatorTokenMetadata")]
fn operator_token_metadata(
    operator: Principal,
) -> ManualReply<Result<Vec<TokenMetadata>, NftError>> {
    sbt_operator_token_metadata(operator)
}

#[query(name = "ownerTokenIdentifiers", manual_reply = true)]
#[candid_method(query, rename = "ownerTokenIdentifiers")]
fn owner_token_identifiers(
    owner: Principal,
) -> ManualReply<Result<Vec<TokenIdentifier>, NftError>> {
    sbt_owner_token_identifiers(owner)
}

#[query(name = "operatorTokenIdentifiers", manual_reply = true)]
#[candid_method(query, rename = "operatorTokenIdentifiers")]
fn operator_token_identifiers(
    operator: Principal,
) -> ManualReply<Result<Vec<TokenIdentifier>, NftError>> {
    sbt_operator_token_identifiers(operator)
}

#[query(name = "tokenMetadata", manual_reply = true)]
#[candid_method(query, rename = "tokenMetadata")]
fn token_metadata(
    token_identifier: TokenIdentifier,
) -> ManualReply<Result<TokenMetadata, NftError>> {
    sbt_token_metadata(token_identifier)
}

// ======================
//      UPDATE CALLS
// ======================

#[update(name = "mint", guard = "is_canister_custodian")]
#[candid_method(update, rename = "mint")]
fn mint(
    to: Principal,
    token_identifier: TokenIdentifier,
    properties: Vec<(String, GenericValue)>,
) -> Result<Nat, NftError> {
    sbt_mint(to, token_identifier, properties)
}