use std::convert::Infallible;
use dav_server::{fakels::FakeLs, localfs::LocalFs, DavHandler};

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

// #[flutter_rust_bridge::frb()] 
pub async fn webdav(dir: String) -> bool {
    let addr = ([0, 0, 0, 0], 8080).into();
 
    let dav_server = DavHandler::builder()
        .filesystem(LocalFs::new(dir.clone(), false, false, false))
        .locksystem(FakeLs::new())
        .build_handler();

    let make_service = hyper::service::make_service_fn(move |_| {
        let dav_server = dav_server.clone();
        async move {
            let func = move |req| {
                let dav_server = dav_server.clone();
                async move {
                    Ok::<_, Infallible>(dav_server.handle(req).await)
                }
            };
            Ok::<_, Infallible>(hyper::service::service_fn(func))
        }
    });

    println!("Serving {} on {}", dir, addr);
    let _ = hyper::Server::bind(&addr)
        .serve(make_service)
        .await
        .map_err(|e| eprintln!("server error: {}", e));
    true
} 

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
