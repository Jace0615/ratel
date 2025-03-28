#![allow(unused_variables, unused)]
use crate::Result;
use bdk::prelude::*;

use by_types::QueryResponse;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::AdditionalResource;

#[derive(Debug, Clone, Eq, PartialEq, Default, Copy, ApiModel, Translate)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum FeatureStatus {
    #[default]
    Todo = 0,
    Done = 1,
    InProgress = 2,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Copy, ApiModel, Translate)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum Network {
    #[default]
    Solana = 0,
    Ethereum = 1,
}

#[api_model(base = "/v1/patron", table = patrons, iter_type=QueryResponse)]
pub struct Patron {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, many_to_one = users, action = create)]
    pub user_id: i64,
    #[api_model(summary, action = create)]
    pub wallet_address: String,
    #[api_model(summary, action = create)]
    pub amount: i64,
    #[api_model(summary, action = create)]
    pub network: Network,
    #[api_model(summary, one_to_many = feature, action = create)]
    pub features: Vec<Feature>,
}

#[api_model(base = "/v1/feature", table = features, iter_type=QueryResponse)]
pub struct Feature {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, many_to_one = patrons)]
    pub patron_id: i64,
    #[api_model(summary, action = create)]
    pub title: String,
    #[api_model(summary, action = create)]
    pub reference: String,
    #[api_model(summary, action = create)]
    pub description: String,
    #[api_model(action = create, type = JSONB)]
    pub attaches: Vec<AdditionalResource>,
    #[api_model(summary, type = INTEGER, action = create, queryable)]
    pub status: FeatureStatus,
}
