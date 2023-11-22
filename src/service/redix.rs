use actix_web::error;
use actix_web::web::Data;
use redis::{Client, RedisError};
use std::sync::Arc;

pub async fn set_one(redis: &Data<Client>, k: String, v: String) -> Result<String, RedisError> {
    let redis_connection = redis.get_multiplexed_tokio_connection().await;

    let response = match redis_connection {
        Ok(mut connection) => redis::Cmd::set(k, v).query_async(&mut connection).await,
        Err(err) => {
            log::error!("redis connection error: {}", err);

            Err(err)
        }
    };

    response
}

pub async fn get_one(
    redis: &Data<Client>,
    key: Arc<String>,
) -> Result<(String, String), RedisError> {
    let redis_connection = redis.get_multiplexed_tokio_connection().await;

    let response = match redis_connection {
        Ok(mut connection) => {
            let name = redis::Cmd::get(key.to_string())
                .query_async(&mut connection)
                .await;

            Ok((key.to_string(), name.unwrap()))
        }
        Err(err) => {
            log::error!("redis connection error: {}", err);

            Err(err)
        }
    };

    response
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
