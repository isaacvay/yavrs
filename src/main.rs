mod components;
pub mod i18n;
mod layouts;
mod routes;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // YavApp automatically loads [server] configuration from package.yavml
    // and auto_routes! discovers and registers all routes under src/routes
    let app = yav::server::YavApp::new()
        .static_dir("public")
        .not_found_handler(routes::not_found::not_found);
    let app = yav::auto_routes!(app);

    println!("🚀 Yav server running at http://{}:{}", app.host, app.port);
    app.serve().await.unwrap();
}
