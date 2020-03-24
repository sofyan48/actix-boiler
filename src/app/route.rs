use actix_web::App;
use actix_web::http::Method;
use app::controller::{v1};
use app::state::AppState;

pub fn setup_routes(app: App<AppState>) -> App<AppState> {
    app
        .route("/users", Method::GET, v1::user::list)
}
