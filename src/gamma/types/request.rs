#![allow(
    clippy::module_name_repetitions,
    reason = "Request suffix is intentional for clarity"
)]

use bon::Builder;
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as, skip_serializing_none};

use crate::gamma::types::{ParentEntityType, RelatedTagsStatus};
use crate::types::{Address, B256, Decimal, U256};

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Default, Serialize)]
#[non_exhaustive]
pub struct TeamsRequest {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub order: Option<String>,
    pub ascending: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub league: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub name: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub abbreviation: Vec<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Default, Serialize)]
#[non_exhaustive]
pub struct TagsRequest {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub order: Option<String>,
    pub ascending: Option<bool>,
    pub include_template: Option<bool>,
    pub is_carousel: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[non_exhaustive]
pub struct TagByIdRequest {
    #[serde(skip_serializing)]
    #[builder(into)]
    pub id: String,
    pub include_template: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[non_exhaustive]
pub struct TagBySlugRequest {
    #[serde(skip_serializing)]
    #[builder(into)]
    pub slug: String,
    pub include_template: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[non_exhaustive]
pub struct RelatedTagsByIdRequest {
    #[serde(skip_serializing)]
    #[builder(into)]
    pub id: String,
    pub omit_empty: Option<bool>,
    pub status: Option<RelatedTagsStatus>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[non_exhaustive]
pub struct RelatedTagsBySlugRequest {
    #[serde(skip_serializing)]
    #[builder(into)]
    pub slug: String,
    pub omit_empty: Option<bool>,
    pub status: Option<RelatedTagsStatus>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Default, Serialize)]
#[non_exhaustive]
pub struct EventsRequest {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub order: Vec<String>,
    pub ascending: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub id: Vec<String>,
    #[builder(into)]
    pub tag_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub exclude_tag_id: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub slug: Vec<String>,
    pub tag_slug: Option<String>,
    pub related_tags: Option<bool>,
    pub active: Option<bool>,
    pub archived: Option<bool>,
    pub featured: Option<bool>,
    pub cyom: Option<bool>,
    pub include_chat: Option<bool>,
    pub include_template: Option<bool>,
    pub recurrence: Option<String>,
    pub closed: Option<bool>,
    pub liquidity_min: Option<Decimal>,
    pub liquidity_max: Option<Decimal>,
    pub volume_min: Option<Decimal>,
    pub volume_max: Option<Decimal>,
    pub start_date_min: Option<DateTime<Utc>>,
    pub start_date_max: Option<DateTime<Utc>>,
    pub end_date_min: Option<DateTime<Utc>>,
    pub end_date_max: Option<DateTime<Utc>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[non_exhaustive]
pub struct EventByIdRequest {
    #[serde(skip_serializing)]
    #[builder(into)]
    pub id: String,
    pub include_chat: Option<bool>,
    pub include_template: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[non_exhaustive]
pub struct EventBySlugRequest {
    #[serde(skip_serializing)]
    #[builder(into)]
    pub slug: String,
    pub include_chat: Option<bool>,
    pub include_template: Option<bool>,
}

#[derive(Debug, Clone, Builder, Serialize)]
#[non_exhaustive]
pub struct EventTagsRequest {
    #[serde(skip_serializing)]
    #[builder(into)]
    pub id: String,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Default, Serialize)]
#[non_exhaustive]
pub struct MarketsRequest {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub order: Option<String>,
    pub ascending: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub id: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub slug: Vec<String>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub clob_token_ids: Vec<U256>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub condition_ids: Vec<B256>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub market_maker_address: Vec<Address>,
    pub liquidity_num_min: Option<Decimal>,
    pub liquidity_num_max: Option<Decimal>,
    pub volume_num_min: Option<Decimal>,
    pub volume_num_max: Option<Decimal>,
    pub start_date_min: Option<DateTime<Utc>>,
    pub start_date_max: Option<DateTime<Utc>>,
    pub end_date_min: Option<DateTime<Utc>>,
    pub end_date_max: Option<DateTime<Utc>>,
    #[builder(into)]
    pub tag_id: Option<String>,
    pub related_tags: Option<bool>,
    pub cyom: Option<bool>,
    pub uma_resolution_status: Option<String>,
    pub game_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub sports_market_types: Vec<String>,
    pub rewards_min_size: Option<Decimal>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub question_ids: Vec<B256>,
    pub include_tag: Option<bool>,
    pub closed: Option<bool>,
    /// Filter for markets mirrored from Polymarket.
    pub mirror: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[non_exhaustive]
pub struct MarketByIdRequest {
    #[serde(skip_serializing)]
    #[builder(into)]
    pub id: String,
    pub include_tag: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[non_exhaustive]
pub struct MarketBySlugRequest {
    #[serde(skip_serializing)]
    #[builder(into)]
    pub slug: String,
    pub include_tag: Option<bool>,
}

#[derive(Debug, Clone, Builder, Serialize)]
#[non_exhaustive]
pub struct MarketTagsRequest {
    #[serde(skip_serializing)]
    #[builder(into)]
    pub id: String,
}

/// Filters for a single page of published market clarifications.
#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Default, Serialize)]
#[non_exhaustive]
pub struct MarketClarificationsRequest {
    /// Maximum number of notes (0–100, default 20 on the server).
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    /// Comma-separated fields to order by.
    pub order: Option<String>,
    pub ascending: Option<bool>,
    /// Numeric Kuest market IDs or market condition hashes.
    #[serde(rename = "market_id", skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub market_ids: Vec<String>,
    #[serde(rename = "event_id", skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub event_ids: Vec<u64>,
    #[serde(rename = "question_id", skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub question_ids: Vec<B256>,
    #[serde(rename = "tx_hash", skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub tx_hashes: Vec<B256>,
    #[serde(rename = "state", skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub states: Vec<String>,
    pub show_in_frontend: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Default, Serialize)]
#[non_exhaustive]
pub struct SeriesListRequest {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub order: Option<String>,
    pub ascending: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub slug: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub categories_ids: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub categories_labels: Vec<String>,
    pub closed: Option<bool>,
    pub include_chat: Option<bool>,
    pub recurrence: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[non_exhaustive]
pub struct SeriesByIdRequest {
    #[serde(skip_serializing)]
    #[builder(into)]
    pub id: String,
    pub include_chat: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[non_exhaustive]
pub struct CommentsRequest {
    pub parent_entity_type: ParentEntityType,
    #[builder(into)]
    pub parent_entity_id: String,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub order: Option<String>,
    pub ascending: Option<bool>,
    pub get_positions: Option<bool>,
    pub holders_only: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[non_exhaustive]
pub struct CommentsByIdRequest {
    #[serde(skip_serializing)]
    #[builder(into)]
    pub id: String,
    pub get_positions: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[non_exhaustive]
pub struct CommentsByUserAddressRequest {
    #[serde(skip_serializing)]
    pub user_address: Address,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub order: Option<String>,
    pub ascending: Option<bool>,
}

#[derive(Debug, Clone, Builder, Serialize)]
#[non_exhaustive]
pub struct PublicProfileRequest {
    pub address: Address,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[non_exhaustive]
pub struct SearchRequest {
    #[builder(into)]
    pub q: String,
    pub cache: Option<bool>,
    pub events_status: Option<String>,
    pub limit_per_type: Option<i32>,
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub events_tag: Vec<String>,
    pub keep_closed_markets: Option<i32>,
    pub sort: Option<String>,
    pub ascending: Option<bool>,
    pub search_tags: Option<bool>,
    pub search_profiles: Option<bool>,
    pub recurrence: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub exclude_tag_id: Vec<String>,
    pub optimized: Option<bool>,
}
