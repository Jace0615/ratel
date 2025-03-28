#![allow(unused_variables, unused)]
use crate::{Result, User};
use bdk::prelude::*;
use by_types::QueryResponse;
use validator::ValidationError;

#[api_model(base = "/v1/topics/:topic-id/comments", table = comments, action_by_id = [like, unlike], iter_type = QueryResponse)]
pub struct Comment {
    #[api_model(summary, primary_key, read_action = [get_comment])]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: u64,

    #[api_model(many_to_one = topics)]
    pub topic_id: i64,
    #[api_model(many_to_one = users)]
    pub author_id: i64,

    // TODO: custom function from topics and users joined table
    // #[api_model(summary)]
    // pub choice: Option<String>,
    #[api_model(summary, action = comment, related = String)]
    pub content: String,

    #[api_model(summary, one_to_many = user_comments, foreign_key = comment_id, aggregator = count)]
    #[serde(default)]
    pub likes: u64,
    #[api_model(many_to_many = user_comments, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = comment_id, aggregator = exist, exist, unique)]
    #[serde(default)]
    pub is_liked: bool,
}

#[api_model(base = "/v1/topics/:topic-id/comments", table = user_comments, iter_type = QueryResponse)]
pub struct CommentLike {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: u64,
    #[api_model(many_to_one = comments)]
    pub comment_id: i64,
    #[api_model(many_to_one = users)]
    pub user_id: i64,
}
