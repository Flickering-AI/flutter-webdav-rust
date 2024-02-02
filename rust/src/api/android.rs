#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString, JObject};
    use self::jni::sys::jint;
    use self::jni::sys::jstring;
    use self::jni::strings::JNIString;
    use jni::objects::JValue;
    use std::convert::Infallible;
    use dav_server::{fakels::FakeLs, localfs::LocalFs, DavHandler};
    use std::{sync::mpsc, thread, time::Duration};
    use tokio::runtime::Runtime;
    use tokio::time::*;

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

    #[no_mangle]
    pub unsafe extern "system" fn Java_expo_modules_webdavserver_ExpoWebdavServerModule_asyncComputation(
        env: JNIEnv,
        _class: JClass,
        callback: JObject,
    ) {
        // `JNIEnv` cannot be sent across thread boundaries. To be able to use JNI
        // functions in other threads, we must first obtain the `JavaVM` interface
        // which, unlike `JNIEnv` is `Send`.
        let jvm = env.get_java_vm().unwrap();

        // We need to obtain global reference to the `callback` object before sending
        // it to the thread, to prevent it from being collected by the GC.
        let callback = env.new_global_ref(callback).unwrap();

        // Use channel to prevent the Java program to finish before the thread
        // has chance to start.
        let (tx, rx) = mpsc::channel();

        let _ = thread::spawn(move || {
            // Signal that the thread has started.
            tx.send(()).unwrap();

            // Use the `JavaVM` interface to attach a `JNIEnv` to the current thread.
            let mut env = jvm.attach_current_thread().unwrap();
            env.call_method(&callback, "asyncCallback", "(I)V", &[(11 as jint).into()])
                                                .unwrap();
            for i in 0..11 {
                let progress = (i * 10) as jint;
                // Now we can use all available `JNIEnv` functionality normally.
                env.call_method(&callback, "asyncCallback", "(I)V", &[progress.into()])
                                    .unwrap();
                thread::sleep(Duration::from_millis(1000));
            }
            // The current thread is detached automatically when `env` goes out of scope.
        });

        // Wait until the thread has started.
        rx.recv().unwrap();
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_expo_modules_webdavserver_ExpoWebdavServerModule_webdav<'local>(
        mut env: JNIEnv<'local>,
        _class: JClass<'local>,
        dir: JString<'local>,
    ) -> JString<'local> {
        // First, we have to get the string out of java. Check out the `strings`
        // module for more info on how this works.
        let input: String = env
            .get_string(&dir)
            .expect("Couldn't get java string!")
            .into();
        let addr = ([0, 0, 0, 0], 8080).into();

        let dav_server = DavHandler::builder()
            .filesystem(LocalFs::new(input.clone(), false, false, false))
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
        let mut rt = Runtime::new().unwrap();
        rt.block_on(async move {
            let _ = hyper::Server::bind(&addr)
                    .serve(make_service)
                    .await
                    .map_err(|e| eprintln!("server error: {}", e));
        });

        // Then we have to create a new java string to return. Again, more info
        // in the `strings` module.
        let output = env
            .new_string(input.clone())
            .expect("Couldn't create java string!");
        output
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_expo_modules_webdavserver_ExpoWebdavServerModule_test(
        env: JNIEnv,
        class: JClass,
        a: jint,
        b: jint
    ) -> jint {
        a + b
    }
}