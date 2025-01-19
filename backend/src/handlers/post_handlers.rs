use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::models::user::UserId;
use crate::models::post::{
    Post, PostListResponse, CreatePostRequest, CreatePostResponse,
    PostResponse, UpdatePostRequest, UpdatePostResponse
};

// Fetch all published posts
pub async fn get_posts_handler(pool: web::Data<PgPool>) -> HttpResponse {
    let result = sqlx::query_as!(
        Post,
        "SELECT id, title, content, published FROM posts WHERE published = true ORDER BY id DESC"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(posts) => HttpResponse::Ok().json(PostListResponse { success: true, posts }),
        Err(e) => {
            eprintln!("Failed to fetch posts: {:?}", e);
            HttpResponse::InternalServerError().json(PostListResponse { success: false, posts: vec![] })
        }
    }
}

// Fetch a single post by ID
pub async fn get_post_handler(pool: web::Data<PgPool>, post_id: web::Path<i32>) -> HttpResponse {
    let post_id = post_id.into_inner();

    let result = sqlx::query_as!(
        Post,
        "SELECT id, title, content, published FROM posts WHERE id = $1",
        post_id
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(post) => HttpResponse::Ok().json(PostResponse { success: true, post: Some(post) }),
        Err(e) => {
            eprintln!("Failed to fetch post: {:?}", e);
            HttpResponse::InternalServerError().json(PostResponse { success: false, post: None })
        }
    }
}

// Fetch all posts for a specific user
pub async fn get_user_posts_handler(pool: web::Data<PgPool>, user_id: UserId) -> HttpResponse {
    let user_id = user_id.0;

    let result = sqlx::query_as!(
        Post,
        "SELECT id, title, content, published FROM posts WHERE user_id = $1 ORDER BY id DESC",
        user_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(posts) => HttpResponse::Ok().json(PostListResponse { success: true, posts }),
        Err(e) => {
            eprintln!("Failed to fetch user posts: {:?}", e);
            HttpResponse::InternalServerError().json(PostListResponse { success: false, posts: vec![] })
        }
    }
}

// Create a new post
pub async fn create_post_handler(
    pool: web::Data<PgPool>,
    post_data: web::Json<CreatePostRequest>,
    user_id: UserId,
) -> HttpResponse {
    let user_id = user_id.0;

    // Validate input
    if post_data.title.trim().is_empty() || post_data.content.trim().is_empty() {
        return HttpResponse::BadRequest().json(CreatePostResponse {
            success: false,
            message: "Title and content cannot be empty.".into(),
            post_id: None,
        });
    }

    // Insert new post into the database
    let query = sqlx::query!(
        "INSERT INTO posts (user_id, title, content, published) VALUES ($1, $2, $3, $4) RETURNING id",
        user_id,
        post_data.title,
        post_data.content,
        post_data.published
    );

    match query.fetch_one(pool.get_ref()).await {
        Ok(record) => HttpResponse::Ok().json(CreatePostResponse {
            success: true,
            message: "Post created successfully!".into(),
            post_id: Some(record.id),
        }),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(CreatePostResponse {
                success: false,
                message: "Failed to create post.".into(),
                post_id: None,
            })
        }
    }
}

// Fetch a post for editing (only if it belongs to the user)
pub async fn get_post_for_edit_handler(
    pool: web::Data<PgPool>,
    user_id: UserId,
    post_id: web::Path<i32>,
) -> HttpResponse {
    let user_id = user_id.0;
    let post_id = post_id.into_inner();

    let result = sqlx::query_as!(
        Post,
        "SELECT id, title, content, published FROM posts WHERE id = $1 AND user_id = $2",
        post_id,
        user_id
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(post) => HttpResponse::Ok().json(PostResponse { success: true, post: Some(post) }),
        Err(e) => {
            eprintln!("Failed to fetch post for edit: {:?}", e);
            HttpResponse::InternalServerError().json(PostResponse { success: false, post: None })
        }
    }
}

// Update an existing post (only if it belongs to the user)
pub async fn update_post_handler(
    pool: web::Data<PgPool>,
    user_id: UserId,
    post_id: web::Path<i32>,
    post_data: web::Json<UpdatePostRequest>,
) -> HttpResponse {
    let user_id = user_id.0;
    let post_id = post_id.into_inner();

    let result = sqlx::query!(
        "UPDATE posts SET title = $1, content = $2, published = $3 WHERE id = $4 AND user_id = $5",
        post_data.title,
        post_data.content,
        post_data.published,
        post_id,
        user_id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(UpdatePostResponse {
            success: true,
            message: "Post updated successfully!".into(),
        }),
        Err(e) => {
            eprintln!("Failed to update post: {:?}", e);
            HttpResponse::InternalServerError().json(UpdatePostResponse {
                success: false,
                message: "Failed to update post.".into(),
            })
        }
    }
}