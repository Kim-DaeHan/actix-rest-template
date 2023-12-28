use crate::error::MyError;
use crate::models::posts::Post;
use crate::schema::posts::{self, dsl::*};
use crate::PgPool;
use actix_web::{http::header::ContentType, web::Data, HttpRequest, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::to_vec;

pub async fn get_posts(pool: Data<PgPool>) -> Result<HttpResponse> {
    let conn = &mut pool.get().expect("Couldn't get DB connection from pool");
    let post_list = posts.load::<Post>(conn).expect("error");
    let post_data = posts
        .select((body, title, posts::id, published))
        .load::<(String, String, String, bool)>(conn)
        .expect("error");
    let json_bytes = to_vec(&post_data).expect("Failed to serialize posts to JSON");

    println!("{:?}", post_list);

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json_bytes))
}

pub async fn get_posts_by_id(
    req: HttpRequest,
    pool: Data<PgPool>,
) -> Result<HttpResponse, MyError> {
    if let Some(post_id) = req.match_info().get("id") {
        let conn = &mut pool.get().expect("Couldn't get DB connection from pool");

        if let Ok(post_data) = posts
            // .find(post_id)
            .filter(posts::id.eq(post_id))
            .select((body, title, posts::id, published))
            .get_result::<(String, String, String, bool)>(conn)
        // .first::<(String, String, String, bool)>(conn)
        // .load::<(String, String, String, bool)>(conn)
        {
            let json_bytes = to_vec(&post_data).expect("Failed to serialize posts to JSON");
            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(json_bytes))
        } else {
            // id로 조회했는데 없을 경우
            // Err(MyError::InternalError)
            Ok(HttpResponse::Ok().body("no result"))
        }
    } else {
        // id가 없는 경우에도 에러 처리
        Err(MyError::BadClientData)
    }
}
