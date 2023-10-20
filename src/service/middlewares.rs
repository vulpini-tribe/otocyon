use actix_web::middleware;

pub struct Middlewares {
    pub headers: middleware::DefaultHeaders,
}

impl Middlewares {
    pub fn new() -> Middlewares {
        Middlewares {
            headers: middleware::DefaultHeaders::new()
                .add((
                    "Authorization",
                    format!("Bearer {}", std::env::var("APIDECK_API_KEY").unwrap()),
                ))
                .add(("x-apideck-app-id", std::env::var("APIDECK_APP_ID").unwrap())),
        }
    }
}
