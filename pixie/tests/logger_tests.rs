use pixie::{log_error, log_info, log_warn};

/// Vérifie que les helpers de log sont appelables sans panic.
#[test]
fn log_helpers_sont_sans_panic() {
    log_info(format_args!("info"));
    log_warn(format_args!("warn"));
    log_error(format_args!("error"));
}
