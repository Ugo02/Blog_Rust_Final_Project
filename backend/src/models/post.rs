use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub published: bool,
}

#[derive(Serialize)]
pub struct PostListResponse {
    pub success: bool,
    pub posts: Vec<Post>,
}

#[derive(Serialize)]
pub struct PostResponse {
    pub success: bool,
    pub post: Option<Post>,
}

#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub published: bool,
}

#[derive(Serialize)]
pub struct CreatePostResponse {
    pub success: bool,
    pub message: String,
    pub post_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdatePostRequest {
    pub title: String,
    pub content: String,
    pub published: bool,
}

#[derive(Serialize)]
pub struct UpdatePostResponse {
    pub success: bool,
    pub message: String,
}


