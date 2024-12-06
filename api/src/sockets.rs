
use actix_web_actors::ws;
use actix::Actor;
use std::time::Duration;
use tokio::time::sleep;
use actix::prelude::*;

// Define the WebSocket actor
struct ProgressActor;

impl Actor for ProgressActor {
    type Context = ws::WebsocketContext<Self>;
}

// Define a message handler for sending progress updates
impl ProgressActor {
    async fn send_progress(ctx: &mut ws::WebsocketContext<Self>, progress: u32) {
        let message = format!("Classifying... {}%", progress);
        ctx.text(message);
    }
}

// The message handler to simulate image classification progress
async fn simulate_classification(ctx: &mut ws::WebsocketContext<ProgressActor>) {
    for i in 0..=100 {
        sleep(Duration::from_millis(100)).await;  // Simulating processing
        ProgressActor::send_progress(ctx, i).await;
    }
}

// WebSocket handler that listens for messages from the frontend and sends back updates
async fn ws_handler(req: actix_web::HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    ws::start(ProgressActor {}, &req, stream)
}

// HTTP handler for starting classification
async fn start_classification() -> impl Responder {
    // This is where you would start the image classification process.
    // For demo purposes, we will simulate the classification process.
    HttpResponse::Ok().body("Classification Started")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(start_classification))
            .route("/ws", web::get().to(ws_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
