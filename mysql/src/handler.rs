use hyper::{Body, Method, Request, Response, StatusCode};
use mysql_async::prelude::*;
use mysql_async::{params, Pool};
use projmysql::domain::Order;
pub async fn handle_request(
    req: Request<Body>,
    pool: Pool,
) -> Result<Response<Body>, anyhow::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::OPTIONS, "/") => Ok(response_build(&String::from(""))),
        (&Method::OPTIONS, "/create_order") => Ok(response_build(&String::from(""))),
        (&Method::OPTIONS, "/update_order") => Ok(response_build(&String::from(""))),
        (&Method::OPTIONS, "/delete_order") => Ok(response_build(&String::from(""))),
        (&Method::OPTIONS, "/orders") => Ok(response_build(&String::from(""))),

        (&Method::GET, "/") => Ok(Response::new(Body::from("Hi There!"))),
        (&Method::POST, "/") => Ok(Response::new(req.into_body())),
        (&Method::POST, "/create_order") => {
            let mut conn = pool.get_conn().await.unwrap();

            let byte_stream = hyper::body::to_bytes(req).await?;
            let order: Order = serde_json::from_slice(&byte_stream).unwrap();

            "INSERT INTO orders (order_id, product_id, quantity, amount, shipping, tax, shipping_address) VALUES (:order_id, :product_id, :quantity, :amount, :shipping, :tax, :shipping_address)"
                .with(params! {
                    "order_id" => order.order_id,
                    "product_id" => order.product_id,
                    "quantity" => order.quantity,
                    "amount" => order.amount,
                    "shipping" => order.shipping,
                    "tax" => order.tax,
                    "shipping_address" => &order.shipping_address.as_ref(),
                })
                .ignore(&mut conn)
                .await?;

            drop(conn);
            Ok(response_build("{\"status\":true}"))
        }

        (&Method::GET, "/health/readiness") => Ok(Response::new(Body::from(""))),
        (&Method::GET, "/health/liveness") => Ok(Response::new(Body::from(""))),
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn response_build(body: &str) -> Response<Body> {
    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header(
            "Access-Control-Allow-Headers",
            "api,Keep-Alive,User-Agent,Content-Type",
        )
        .body(Body::from(body.to_owned()))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mysql_async::{Opts, OptsBuilder, PoolConstraints, PoolOpts};
    #[tokio::test]
    async fn get_test() -> Result<(), Box<dyn std::error::Error>> {
        let req = hyper::Request::builder()
            .method(hyper::Method::GET)
            .uri("/")
            .header("user-agent", "rust-test")
            .body(hyper::Body::from(""))?;
        let opts = Opts::from_url(&*get_url()).unwrap();
        let builder = OptsBuilder::from_opts(opts);
        let constraints = PoolConstraints::new(5, 10).unwrap();
        let pool_opts = PoolOpts::default().with_constraints(constraints);
        let pool = Pool::new(builder.pool_opts(pool_opts));
        let resp = handle_request(req, pool).await?;
        let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
        let body = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert_eq!(body, "Hi There!");
        Ok(())
    }
    #[tokio::test]
    async fn post_test() -> Result<(), Box<dyn std::error::Error>> {
        let req = hyper::Request::builder()
            .method(hyper::Method::POST)
            .uri("/")
            .header("user-agent", "rust-test")
            .body(hyper::Body::from("echo"))?;
        let opts = Opts::from_url(&*get_url()).unwrap();
        let builder = OptsBuilder::from_opts(opts);
        let constraints = PoolConstraints::new(5, 10).unwrap();
        let pool_opts = PoolOpts::default().with_constraints(constraints);
        let pool = Pool::new(builder.pool_opts(pool_opts));
        let resp = handle_request(req, pool).await?;
        let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
        let body = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert_eq!(body, "echo");
        Ok(())
    }
    fn get_url() -> String {
        if let Ok(url) = std::env::var("DATABASE_URL") {
            let opts = Opts::from_url(&url).expect("DATABASE_URL invalid");
            if opts
                .db_name()
                .expect("a database name is required")
                .is_empty()
            {
                panic!("database name is empty");
            }
            url
        } else {
            "mysql://root:password@127.0.0.1:3306/ordersystem".into()
        }
    }
}
