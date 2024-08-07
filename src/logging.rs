use rlg::log::Log;
use rlg::log_format::LogFormat;
use rlg::log_level::LogLevel;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub fn setup_logging() {
    let home_dir = home::home_dir().unwrap();
    let path = home_dir.join(".config/pmx-1/logs");

    unsafe {
        std::env::set_var("LOG_FILE_PATH", path);
    }
}

pub struct Logger {
    component: String,
    sender: UnboundedSender<Log>,
}

impl Logger {
    pub fn new(component: String, sender: UnboundedSender<Log>) -> Logger {
        Logger { component, sender }
    }

    pub fn log_info(&self, message: &str) {
        let time_stamp = format!("{:?}", chrono::offset::Local::now());
        let log_entry = Log::new(
            "1",
            &time_stamp,
            &LogLevel::INFO,
            &self.component,
            message,
            &LogFormat::NDJSON,
        );
        self.sender.send(log_entry).unwrap();
    }
}

pub struct LoggerFactory {
    sender: UnboundedSender<Log>,
}

impl LoggerFactory {
    pub fn new(sender: UnboundedSender<Log>) -> Self {
        LoggerFactory { sender }
    }

    pub fn new_logger(&self, component: String) -> Logger {
        Logger::new(component, self.sender.clone())
    }
}

pub fn run_logging_loop(mut receiver: UnboundedReceiver<Log>) {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            loop {
                if let Some(log) = receiver.recv().await {
                    log.log().await.unwrap();
                };
            }
        });
}
