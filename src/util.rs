#[macro_export]
macro_rules! log_err {
    ($task:expr) => {
        if let Err(e) = $task {
            eprintln!("{e}");
        }
    };
}

#[macro_export]
macro_rules! log_cmd {
    ($text:expr $(, $arg:expr)*) => {
        println!(
            "{}; {}",
            chrono::Local::now().format("%H:%M:%S, %m/%d/%Y").to_string(),
            format!($text $(, $arg)*)
        )
    };
}
