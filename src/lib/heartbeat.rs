use actix_web::{web, HttpResponse, Responder};
use log::{info};

pub async fn heartbeat() -> impl Responder {
    info!("Heartbeat Ping!");
    HttpResponse::Ok().body("OK")
}

pub fn configure_heartbeat(cfg: &mut web::ServiceConfig) {
    cfg.route("/heartbeat", web::get().to(heartbeat));
}