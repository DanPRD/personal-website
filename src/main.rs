#![allow(dead_code)]
use axum::{http::StatusCode, response::{Html, IntoResponse}, routing::get, Router};
use tower_http::{services::{ServeDir, ServeFile}, trace::TraceLayer};
use tracing;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use askama::Template;

struct Meta<'a> {
    title: &'a str,
    desc: &'a str,
    url: &'a str
}

#[derive(Template)]
#[template(path="homepage.html")]
struct HomePageTemplate<'a> {
    meta: Meta<'a>,
    name: &'a str
}

#[derive(Template)]
#[template(path="projects.html")]
struct ProjectPageTemplate<'a> {
    meta: Meta<'a>,
}

#[derive(Template)]
#[template(path="blog.html")]
struct BlogHomePageTemplate<'a> {
    meta: Meta<'a>,
}

#[derive(Template)]
#[template(path="blogpost.html")]
struct BlogPostTemplate<'a> {
    meta: Meta<'a>,
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
    let template = HomePageTemplate {
        meta: Meta {title: "Home", desc: "Personal blog about programming and projects i make :3", url: ""},
        name: "Dan"
    };
    let html = template.render().unwrap();
    (StatusCode::OK, Html(html))
}


async fn biome_generator() -> &'static str{
    "this is a biome gen"
}

async  fn get_blog() -> impl IntoResponse {
    let template = BlogPostTemplate {
        meta: Meta {title: "Projects", desc: "Display for any projects i have made", url: "blog/firstblogname"},
    };
    let html = template.render().unwrap();
    (StatusCode::OK, Html(html))
}

async fn projects() -> impl IntoResponse {
    let template = ProjectPageTemplate {
        meta: Meta {title: "Projects", desc: "Display for any projects i have made", url: "projects"},
    };
    let html = template.render().unwrap();
    (StatusCode::OK, Html(html))
}

async fn blog() -> impl IntoResponse {
    let template = BlogHomePageTemplate {
        meta: Meta {title: "Blog", desc: "Blog homepage where you can find all my posts", url: "blog"},
    };
    let html = template.render().unwrap();
    (StatusCode::OK, Html(html))
}

