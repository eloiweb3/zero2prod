use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn health_check() ->  HttpResponse {
     HttpResponse::Ok().finish()

}

#[tokio::main]

async fn main() ->Result<(), std::io::Error>{
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use actix_web::web;
    use crate::health_check;

    #[tokio::test]
    async fn health_check_succceeds() {
        let response = health_check().await;
        assert!(response.status().is_success());
    }
}
