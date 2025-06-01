use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

async fn health_check() ->  HttpResponse {
     HttpResponse::Ok().finish()

}

pub  fn run() ->Result<Server, std::io::Error>{
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
        .bind("127.0.0.1:8080")?
        .run();

    Ok(server)
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
