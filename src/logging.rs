use flexi_logger::{LogSpecBuilder, Logger};

pub fn configure_logging(logging_opts: &crate::cli::LoggingOpts) {
    let mut builder = LogSpecBuilder::new(); // default is LevelFilter::Off
    builder.default(logging_opts.to_level_filter());

    if logging_opts.verbose + 2 < 3 {
        for library in &[
            "want",
            "hyper",
            "mio",
            "rustls",
            "tokio_threadpool",
            "tokio_reactor",
        ] {
            builder.remove(library);
        }
    }

    Logger::with(builder.build())
        // your logger configuration goes here, as usual
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));
}
