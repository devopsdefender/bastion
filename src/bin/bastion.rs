//! `bastion serve` — standalone block-aware terminal on localhost.
//!
//! Useful for local iteration and as a reference for embedding.

use axum::Router;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(|s| s.as_str()) != Some("serve") {
        eprintln!("usage: bastion serve [--port N] [--bind ADDR]");
        std::process::exit(2);
    }

    let mut port: u16 = 7681;
    let mut bind: String = "127.0.0.1".into();
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--port" => {
                port = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(port);
                i += 2;
            }
            "--bind" => {
                if let Some(s) = args.get(i + 1) {
                    bind = s.clone();
                }
                i += 2;
            }
            other => {
                eprintln!("bastion: unknown arg {other}");
                std::process::exit(2);
            }
        }
    }

    let addr = format!("{bind}:{port}");
    eprintln!("bastion: listening on http://{addr}/");
    let mgr = bastion::Manager::new();
    let app: Router = bastion::router(mgr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .await
        .map_err(|e| std::io::Error::other(format!("{e}")))
}
