use axum::response::Redirect;
use axum_extra::extract::WithRejection;
use axum_oidc::OidcRpInitiatedLogout;

use crate::{config::AppConfig, error::WithAppRejection};

#[utoipa::path(get, path = "/login")]
pub async fn login() -> Redirect {
    Redirect::temporary("/projects")
}

#[utoipa::path(get, path = "/logout")]
pub async fn logout(
    WithRejection(logout, _): WithAppRejection<OidcRpInitiatedLogout>,
) -> OidcRpInitiatedLogout {
    logout.with_post_logout_redirect(AppConfig::get().oidc.redirect_url.parse().unwrap())
}
