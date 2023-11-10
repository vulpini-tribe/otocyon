use actix_web::error;
use actix_web::web::Data;
use actix_web::HttpResponse;
use redis::Client;

pub async fn set_one(redis: Data<Client>, k: String, v: String) -> Result<HttpResponse, String> {
    let redis_connection = redis
        .get_multiplexed_tokio_connection()
        .await
        .map_err(error::ErrorInternalServerError);

    let response = redis::Cmd::set(k, v)
        .query_async::<_, String>(&mut redis_connection.unwrap())
        .await
        .map_err(error::ErrorInternalServerError);

    if response.unwrap() == "OK" {
        Ok(HttpResponse::Ok().body("Ok"))
    } else {
        Ok(HttpResponse::InternalServerError().finish())
    }
}
