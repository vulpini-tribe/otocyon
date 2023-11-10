use actix_web::error;
use actix_web::web::Data;
use redis::Client;

pub async fn set_one(redis: &Data<Client>, k: String, v: String) -> String {
    let redis_connection = redis
        .get_multiplexed_tokio_connection()
        .await
        .map_err(error::ErrorInternalServerError);

    let response = redis::Cmd::set(k, v)
        .query_async::<_, String>(&mut redis_connection.unwrap())
        .await
        .map_err(error::ErrorInternalServerError);

    response.unwrap_or(String::from("error"))
}

pub async fn get_one(redis: &Data<Client>, key: String) -> String {
    let redis_connection = redis
        .get_multiplexed_tokio_connection()
        .await
        .map_err(error::ErrorInternalServerError);

    let response = redis::Cmd::get(key)
        .query_async(&mut redis_connection.unwrap())
        .await
        .map_err(error::ErrorInternalServerError);

    response.unwrap_or(String::from("error"))
}

pub async fn del_one(redis: &Data<Client>, key: String) -> String {
    let redis_connection = redis
        .get_multiplexed_tokio_connection()
        .await
        .map_err(error::ErrorInternalServerError);

    let response = redis::Cmd::del(key)
        .query_async::<_, String>(&mut redis_connection.unwrap())
        .await
        .map_err(error::ErrorInternalServerError);

    response.unwrap()
}
