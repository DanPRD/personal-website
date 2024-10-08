use axum::{http::StatusCode, response::{Html, IntoResponse}, routing::get, Router};
use tower_http::{services::{ServeDir, ServeFile}, trace::TraceLayer};
use tracing;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use askama::Template;

#[derive(Template)]
#[template(path="homepage.html")]
struct HomePageTemplate<'a> {
    name: &'a str
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!(
                "{}=debug,tower_http=debug,axum::rejection=trace",
                env!("CARGO_CRATE_NAME")
            )
            .into()
        }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    let blog_pages = Router::new()
        .route("/", get(blog))
        .route("/firstblogname", get(get_blog));


    let projects = Router::new()
        .route("/", get(projects))
        .route("/biome-generator", get(biome_generator));
    

    let root_app = Router::new()
        .nest("/projects", projects)
        .nest("/blog", blog_pages)
        .nest_service("/favicon.ico", ServeFile::new("server_files\\favicon.ico"))
        .nest_service("/static", ServeDir::new("server_files\\static").not_found_service(ServeFile::new("server_files\\static\\404.txt")))
        .route("/", get(index))
        .fallback_service(ServeFile::new("server_files\\static\\404.txt"))
        .layer(TraceLayer::new_for_http());


    let listener = tokio::net::TcpListener::bind("127.0.0.1:1111").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, root_app).await.unwrap();
}

async fn index() -> impl IntoResponse {
    let template = HomePageTemplate {name: "Dan"};
    let html = template.render().unwrap();
    (StatusCode::OK, Html(html))
}


async fn biome_generator() -> &'static str{
    "this is a biome gen"
}

async  fn get_blog() -> &'static str {
    "my first blog hi guys"
}

async fn projects() -> &'static str {
    "projects"
}

async fn blog() -> &'static str {
    "blog"
}


