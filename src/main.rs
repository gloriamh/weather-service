use {
    hyper::{
        Body, Method, Request, Response, Server, StatusCode,
        service::service_fn,
        rt::{run},
    },
    futures::{
        compat::Future01CompatExt,
        future::{FutureExt, TryFutureExt},
    },
    queryst,
    reqwest,
    serde_json,
    std::net::SocketAddr,
};

mod weather;

async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());

    let query_string = req.uri().query().unwrap_or("");
    let query = queryst::parse(query_string).unwrap_or(serde_json::Value::Null);

    let city = query["city"].as_str().unwrap_or("").to_string();
    let units = query["units"].as_str().unwrap_or("metric").to_string();

    if city == "" {
        *response.status_mut() = StatusCode::BAD_REQUEST;
        return Ok(response)
    }

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/weather") => {
            let temperature = weather::get_current_temperature(city, units);
            *response.body_mut() = Body::from(serde_json::json!({
                "current_temperature": temperature}).to_string())
        }
        (&Method::GET, "/forecast") => {
            let forecast = weather::get_forecast(city, units);
            *response.body_mut() = Body::from(forecast)
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    let serve_future = Server::bind(&addr)
        .serve(|| service_fn(|req| serve_req(req).boxed().compat()));

    if let Err(e) = serve_future.compat().await {
        eprintln!("server error: {}", e);
    }
}

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let futures_03_future = run_server(addr);
    let futures_01_future = futures_03_future.unit_error().boxed().compat();

    run(futures_01_future);
}
