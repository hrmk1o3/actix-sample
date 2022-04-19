use actix_web::{web, App, HttpServer};
use app::{
    config::Config,
    handlers::{add_user, get_users},
};
use dotenv::dotenv;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Read configuration from the `.env` file.
    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::resource("/users")
                .route(web::post().to(add_user))
                .route(web::get().to(get_users)),
        )
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
