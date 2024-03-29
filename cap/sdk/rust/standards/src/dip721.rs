use std::{collections::HashMap, convert::TryInto};

use candid::{CandidType, Deserialize, Principal};
use cap_sdk::{DetailValue, IndefiniteEvent, IntoEvent, TryFromEvent};
use thiserror::Error;

#[derive(Debug, Clone, Copy, CandidType, Deserialize)]
pub enum sbtTransactionType {
    TransferFrom(TransferFrom),
    Approve(Approve),
    SetApprovalForAll(SetApprovalForAll),
    Mint(Mint),
    Burn(Burn),
}

#[derive(Debug, Clone, Copy, CandidType, Deserialize)]
pub struct TransferFrom {
    pub token_id: u64,
    pub from: Principal,
    pub to: Principal,
    pub caller: Option<Principal>,
}

#[derive(Debug, Clone, Copy, CandidType, Deserialize)]
pub struct Approve {
    pub token_id: u64,
    pub from: Principal,
    pub to: Principal,
}

#[derive(Debug, Clone, Copy, CandidType, Deserialize)]
pub struct SetApprovalForAll {
    pub from: Principal,
    pub to: Principal,
}

#[derive(Debug, Clone, Copy, CandidType, Deserialize)]
pub struct Mint {
    pub token_id: u64,
}

#[derive(Debug, Clone, Copy, CandidType, Deserialize)]
pub struct Burn {
    pub token_id: u64,
}
#[cfg(feature = "sdk-impls")]
impl IntoEvent for sbtTransactionType {
    fn operation(&self) -> Option<&'static str> {
        Some(match *self {
            Self::TransferFrom(_) => "transfer_from",
            Self::Approve(_) => "approve",
            Self::SetApprovalForAll(_) => "set_approval_for_all",
            Self::Mint(_) => "mint",
            Self::Burn(_) => "burn",
        })
    }

    fn details(&self) -> Vec<(String, cap_sdk::DetailValue)> {
        match *self {
            Self::TransferFrom(transfer) => {
                let mut data = vec![
                    ("token_id".to_owned(), transfer.token_id.into()),
                    ("from".to_owned(), transfer.from.into()),
                    ("to".to_owned(), transfer.to.into()),
                ];

                if let Some(caller) = transfer.caller {
                    data.push(("caller".to_owned(), caller.into()));
                }

                data
            }
            Self::Approve(approve) => {
                vec![
                    ("token_id".to_owned(), approve.token_id.into()),
                    ("from".to_owned(), approve.from.into()),
                    ("to".to_owned(), approve.to.into()),
                ]
            }
            Self::SetApprovalForAll(set_approval) => {
                vec![
                    ("from".to_owned(), set_approval.from.into()),
                    ("to".to_owned(), set_approval.to.into()),
                ]
            }
            Self::Mint(mint) => {
                vec![("token_id".to_owned(), mint.token_id.into())]
            }
            Self::Burn(burn) => {
                vec![("token_id".to_owned(), burn.token_id.into())]
            }
        }
    }
}

#[cfg(feature = "sdk-impls")]
impl TryFromEvent for sbtTransactionType {
    type Error = sbtTransactionDecodeError;

    fn try_from_event(
        event: impl Into<IndefiniteEvent>,
    ) -> Result<Self, sbtTransactionDecodeError> {
        let event = event.into();
        let map = event.details.iter().cloned().collect::<HashMap<_, _>>();

        Ok(match event.operation.as_str() {
            "transfer_from" => {
                let caller = {
                    if let Ok(caller) = try_get_and_clone(&map, "caller") {
                        Some(caller.try_into().map_failure("caller")?)
                    } else {
                        None
                    }
                };

                sbtTransactionType::TransferFrom(TransferFrom {
                    token_id: try_get_and_clone(&map, "token_id")?
                        .try_into()
                        .map_failure("token_id")?,
                    from: try_get_and_clone(&map, "from")?
                        .try_into()
                        .map_failure("from")?,
                    to: try_get_and_clone(&map, "to")?
                        .try_into()
                        .map_failure("to")?,
                    caller,
                })
            }
            "approve" => sbtTransactionType::Approve(Approve {
                token_id: try_get_and_clone(&map, "token_id")?
                    .try_into()
                    .map_failure("token_id")?,
                from: try_get_and_clone(&map, "from")?
                    .try_into()
                    .map_failure("from")?,
                to: try_get_and_clone(&map, "to")?
                    .try_into()
                    .map_failure("to")?,
            }),
            "set_approval_for_all" => sbtTransactionType::SetApprovalForAll(SetApprovalForAll {
                from: try_get_and_clone(&map, "from")?
                    .try_into()
                    .map_failure("from")?,
                to: try_get_and_clone(&map, "to")?
                    .try_into()
                    .map_failure("to")?,
            }),
            "mint" => sbtTransactionType::Mint(Mint {
                token_id: try_get_and_clone(&map, "token_id")?
                    .try_into()
                    .map_failure("token_id")?,
            }),
            "burn" => sbtTransactionType::Burn(Burn {
                token_id: try_get_and_clone(&map, "token_id")?
                    .try_into()
                    .map_failure("token_id")?,
            }),

            operation => {
                return Err(sbtTransactionDecodeError::InvalidOperation(
                    operation.to_owned(),
                ))
            }
        })
    }
}

#[cfg(feature = "sdk-impls")]
#[derive(Debug, Error)]
pub enum sbtTransactionDecodeError {
    #[error("missing key {0}")]
    MissingKey(String),
    #[error("couldn't convert item with key {0} to DetailValue")]
    ConversionError(String),
    #[error("invalid operation {0}")]
    InvalidOperation(String),
}

#[cfg(feature = "sdk-impls")]
trait MapFailed<T, E> {
    fn map_failure(self, key: &'static str) -> Result<T, E>;
}

#[cfg(feature = "sdk-impls")]
impl<T, O> MapFailed<T, sbtTransactionDecodeError> for Result<T, O> {
    fn map_failure(self, key: &'static str) -> Result<T, sbtTransactionDecodeError> {
        self.map_err(|_| sbtTransactionDecodeError::ConversionError(key.to_owned()))
    }
}

#[cfg(feature = "sdk-impls")]
fn try_get_and_clone(
    map: &HashMap<String, DetailValue>,
    key: &'static str,
) -> Result<DetailValue, sbtTransactionDecodeError> {
    if let Some(item) = map.get(key) {
        Ok(item.clone())
    } else {
        Err(sbtTransactionDecodeError::MissingKey(key.to_owned()))
    }
}
