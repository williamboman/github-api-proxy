use http::{Method, StatusCode};
use mason_registry_api::{
    parse_url,
    pypi::{client::PyPiClient, manager::PyPiManager},
    QueryParams,
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
    let pypi_package = (&query_params).into();
    let version = query_params.get("version").unwrap();
    let manager = PyPiManager::new(PyPiClient::new());

    match manager.get_project_version(&pypi_package, version) {
        Ok(package) => mason_registry_api::ok_json(package.info),
        Err(err) => mason_registry_api::err_json(err),
    }
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
