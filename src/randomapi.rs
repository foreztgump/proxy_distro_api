use actix_web::{HttpResponse, web};
use actix_web::web::Path;
use crate::utils::get_proxy;

#[get("/random/proxy/{country}")]
pub async fn random_proxy(path: Path<String>) -> HttpResponse {
    let proxy = web::block(move || get_proxy(path.clone())).await;

    match proxy {
        Ok(proxy) => {
            HttpResponse::Ok()
                .body(proxy)
        }
        _ => HttpResponse::NoContent()
            .await
            .unwrap(),
    }
}