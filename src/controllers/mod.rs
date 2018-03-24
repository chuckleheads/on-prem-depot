use rocket::Route;

pub mod ext;
pub mod admin;
pub mod origins;
pub mod profile;
pub mod user;

pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(ext::routes());
    routes.extend(admin::routes());
    routes.extend(origins::routes());
    routes.extend(profile::routes());
    routes.extend(user::routes());
    routes
}
