use actix_web::{get, HttpResponse, Responder};

#[get("")]
pub async fn get_all() -> actix_web::Result<impl Responder> {
    Ok(HttpResponse::Ok()
        .insert_header(("Access-Control-Expose-Headers", "X-Total-Count"))
        .insert_header(("X-Total-Count", format!("{}", 0)))
        .json({}))
}
