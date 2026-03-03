use crate::core::controllers::health::HealthResponse;

use crate::core::controllers::login_user::{LoginRequest, LoginResponse};
use crate::core::controllers::logout_user::{LogoutResponse, SearchParams};
use crate::core::controllers::register_user::{InSpecs, RegisterResponse};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::core::controllers::health::health,        
        crate::core::controllers::register_user::register_user,
        crate::core::controllers::login_user::login_user,  
        crate::core::controllers::logout_user::logout_user,
    ),
    components(schemas(
        HealthResponse,
        InSpecs,
        RegisterResponse,
        LoginRequest,
        LoginResponse,       
        SearchParams,
        LogoutResponse
    )),
    tags(
        (name = "Health", description = "Health endpoints"),
        (name = "Register users", description = "This is the endpoint to sign up users on the Chat Auth Server"),
        (name = "Login users", description = "Login endpoints with Username (Email address) and Password"),
      (name = "Logout users", description = "This is the endpoint to sign out or logout users from the Chat Auth Server"),
    )
)]
pub struct ApiDoc;
