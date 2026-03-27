use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub credit_balance: f64,
    pub tier: OrganizationTier,
    pub usage: OrganizationUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationTier {
    pub max_monthly_credit_spend: f64,
    pub models: HashMap<String, OrganizationTierModelLimits>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationTierModelLimits {
    pub max_concurrent_generations: u32,
    pub max_daily_generations: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationUsage {
    pub models: HashMap<String, OrganizationUsageModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationUsageModel {
    pub daily_generations: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UsageQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

impl UsageQueryRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn before_date(mut self, date: impl Into<String>) -> Self {
        self.before_date = Some(date.into());
        self
    }

    pub fn start_date(mut self, date: impl Into<String>) -> Self {
        self.start_date = Some(date.into());
        self
    }

    pub fn end_date(self, date: impl Into<String>) -> Self {
        self.before_date(date)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsageResponse {
    pub models: Vec<String>,
    pub results: Vec<UsageResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsageResult {
    pub date: String,
    pub used_credits: Vec<UsedCredit>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsedCredit {
    pub amount: f64,
    pub model: String,
}
