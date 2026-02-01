mod config;
mod db;
mod handlers;
mod swagger_doc;

use crate::config::CONFIG;
use crate::db::init_pool;

use actix_web::{middleware, web, App, HttpServer};

#[cfg(debug_assertions)]
use utoipa::OpenApi;
#[cfg(debug_assertions)]
use utoipa_swagger_ui::SwaggerUi;
#[cfg(debug_assertions)]
use crate::swagger_doc::ApiDoc;

fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope(CONFIG.api_scope)
        .service(handlers::index_service)
        .service(handlers::check_db);
    conf.service(scope);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = init_pool().await;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://{}:{}", CONFIG.host, CONFIG.port);
    #[cfg(debug_assertions)]
    log::info!("doc available at http://{}:{}/docs/", CONFIG.host, CONFIG.port);

    HttpServer::new(move || {
        let mut app = App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(config);

        #[cfg(debug_assertions)]
        {
            app = app.service(
                SwaggerUi::new("/docs/{_:.*}")
                    .url("/openapi.json", ApiDoc::openapi()),
            );
        }

        app
    })
        .bind((CONFIG.host, CONFIG.port))?
        .run()
        .await
}