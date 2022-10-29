use http::{Method, StatusCode};
use mason_registry_api::{
    npm::{client::NpmClient, manager::NpmManager},
    parse_url, QueryParams,
};

use std::error::Error;

use vercel_lambda::{error::VercelError, lambda, Body, IntoResponse, Request, Response};

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    if request.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)?);
    }

    let url = parse_url(&request)?;
    let query_params: QueryParams = (&url).into();
    let npm_package = (&query_params).into();
    let manager = NpmManager::new(NpmClient::new());

    match manager.get_all_package_versions(&npm_package) {
        Ok(versions) => mason_registry_api::ok_json(versions),
        Err(err) => mason_registry_api::err_json(err),
    }
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
