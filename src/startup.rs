pub struct Application {
    port: u16,
}

impl Application {
    pub async fn build(
        settings: crate::settings::Settings,
        test_pool: Option<sqlx::postgres::PgPool>,
    ) -> Result<Self, std::io::Error> {
        let store = if let Some(pool) = test_pool {
            crate::store::Store { connection: pool }
        } else {
            let db_url = std::env::var("DATABASE_URL").expect("Failed to get DATABASE_URL.");
            crate::store::Store::new(&db_url).await
        };

        sqlx::migrate!()
            .run(&store.clone().connection)
            .await
            .expect("Failed to migrate");

        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );

        let listener = std::net::TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();

        run(listener, store, settings).await;

        Ok(Self { port })
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

async fn run(
    listener: std::net::TcpListener,
    store: crate::store::Store,
    _settings: crate::settings::Settings,
) {
    // let redis_url = std::env::var("REDIS_URL").expect("Failed to get REDIS_URL.");

    // Redis connection pool
    // let cfg = deadpool_redis::Config::from_url(redis_url.clone());
    // let redis_pool = cfg
    //     .create_pool(Some(deadpool_redis::Runtime::Tokio1))
    //     .expect("Cannot create deadpool redis.");

    // build our application with a route
    let app = axum::Router::new()
        // `GET /` goes to `root`
        .route("/", axum::routing::get(root))
        .with_state(store);

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}
// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
