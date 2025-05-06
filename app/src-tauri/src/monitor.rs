pub fn start() {
    let _guard = sentry::init((
        "https://fcb721997d24a33b1bb3a13bdce4bd05@o83602.ingest.us.sentry.io/4507806662131712",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));
}
