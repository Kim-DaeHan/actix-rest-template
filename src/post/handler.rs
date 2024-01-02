use super::error::PostError;
use super::model::{Post, PostData};
use crate::database::PgPool;
use actix_web::Responder;
use actix_web::{http::header::ContentType, web, web::Data, HttpRequest, HttpResponse, Result};
use log::{info, warn};
use serde_json::to_vec;

pub async fn get_posts(pool: Data<PgPool>) -> Result<impl Responder, PostError> {
    info!("로깅 테스트");
    warn!("로깅 테스트2");

    let post_list = Post::get_posts_load(&pool);

    println!("{:?}", post_list);
    if let Ok(post_data) = Post::get_posts(&pool) {
        let json_bytes = to_vec(&post_data).expect("Failed to serialize posts to JSON");

        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(json_bytes))
    } else {
        // 서버 에러
        Err(PostError::InternalError)
    }
}

pub async fn get_posts_by_id(
    req: HttpRequest,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    if let Some(post_id) = req.match_info().get("id") {
        if let Ok(post_data) = Post::get_posts_by_id(&pool, post_id) {
            let json_bytes = to_vec(&post_data).expect("Failed to serialize posts to JSON");
            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(json_bytes))
        } else {
            // id로 조회했는데 없을 경우
            // Err(MyError::InternalError)
            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body("no result"))
        }
    } else {
        // id가 없는 경우에도 에러 처리
        Err(PostError::BadClientData)
    }
}

pub async fn create_posts(
    _body: web::Json<PostData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    let post_data = _body.into_inner();
    PostData::create_posts(post_data, &pool);

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("created new post"))
}

pub async fn update_posts(
    _body: web::Json<PostData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    let post_data = _body.into_inner();
    if post_data.id.is_some() {
        if PostData::update_posts(post_data, &pool) == Ok(0) {
            Err(PostError::BadClientData)
        } else {
            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body("updated new post"))
        }
    } else {
        //return이 있으면 update_posts(전체 함수)의 반환값, 없으면 해당 블록의 반환 값
        Err(PostError::BadClientData)
    }
}

pub async fn delete_posts_by_id(
    req: HttpRequest,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    if let Some(post_id) = req.match_info().get("id") {
        if Post::delete_posts_by_id(&pool, post_id) == Ok(0) {
            Err(PostError::BadClientData)
        } else {
            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body("deleted post"))
        }
    } else {
        // id가 없는 경우에도 에러 처리
        Err(PostError::BadClientData)
    }
}
