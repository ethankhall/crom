use chrono::Local;
use std::io::{stderr, stdout};

use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::Level;

pub fn configure_logging(verbose: i32, warn: bool, quite: bool) {
    let level: Level = if quite {
        log_level(0)
    } else if warn {
        log_level(1)
    } else {
        log_level(verbose + 2)
    };

    let mut dispatch = Dispatch::new();
    if verbose + 2 < 6 {
        for library in &[
            "want",
            "hyper",
            "mio",
            "rustls",
            "tokio_threadpool",
            "tokio_reactor",
        ] {
            dispatch = dispatch.level_for(*library, Level::Warn.to_level_filter());
        }
    }

    let result = configure_logging_output(level, dispatch)
        .level(level.to_level_filter())
        .chain(
            Dispatch::new()
                .filter(|log_meta| Level::Warn <= log_meta.level())
                .chain(stdout()),
        )
        .chain(
            Dispatch::new()
                .filter(|log_meta| Level::Error == log_meta.level())
                .chain(stderr()),
        )
        .apply();

    if result.is_err() {
        panic!("Logger already initialized...");
    }
}

fn log_level(number_of_verbose: i32) -> Level {
    match number_of_verbose {
        0 => Level::Error,
        1 => Level::Warn,
        2 => Level::Info,
        3 => Level::Debug,
        _ => Level::Trace,
    }
}

fn configure_logging_output(logging_level: Level, dispatch: Dispatch) -> Dispatch {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Magenta)
        .error(Color::Red)
        .debug(Color::Blue);

    if logging_level == Level::Trace || logging_level == Level::Debug {
        dispatch.format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d - %H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
    } else {
        dispatch.format(move |out, message, record| {
            if record.level() == Level::Error {
                out.finish(format_args!(
                    "[{}] {}",
                    colors.color(record.level()),
                    message
                ));
            } else {
                out.finish(format_args!("{}", message));
            }
        })
    }
}
