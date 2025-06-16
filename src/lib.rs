

pub mod configuration;
pub mod routes;
pub mod startup;





#[cfg(test)]
mod tests {
    use actix_web::web;
    use crate::routes::health_check;

    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        assert!(response.status().is_success());
    }
}
