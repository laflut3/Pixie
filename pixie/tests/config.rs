use pixie::runtime_config;
use std::{
    env, fs,
    path::PathBuf,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

static ENV_LOCK: Mutex<()> = Mutex::new(());

struct EnvGuard {
    key: &'static str,
    old: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let old = env::var(key).ok();
        unsafe { env::set_var(key, value) };
        Self { key, old }
    }

    fn unset(key: &'static str) -> Self {
        let old = env::var(key).ok();
        unsafe { env::remove_var(key) };
        Self { key, old }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.old {
            Some(value) => unsafe { env::set_var(self.key, value) },
            None => unsafe { env::remove_var(self.key) },
        }
    }
}

fn write_temp_config(content: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time should be monotonic")
        .as_nanos();
    let path = env::temp_dir().join(format!("pixie-config-{nanos}.yml"));
    fs::write(&path, content).expect("temp config should be writable");
    path
}

#[test]
fn reads_yaml_from_pixie_config_path() {
    let _lock = ENV_LOCK.lock().expect("env lock poisoned");
    let _addr = EnvGuard::unset("PIXIE_ADDR");
    let _threads = EnvGuard::unset("PIXIE_THREADS");

    let path = write_temp_config("host: 0.0.0.0\nport: 9090\nworkers: 8\n");
    let _config = EnvGuard::set("PIXIE_CONFIG", path.to_str().expect("utf-8 path"));

    let config = runtime_config().expect("runtime config should load");
    assert_eq!(config.addr, "0.0.0.0:9090");
    assert_eq!(config.workers, 8);

    let _ = fs::remove_file(path);
}

#[test]
fn keeps_defaults_for_invalid_workers() {
    let _lock = ENV_LOCK.lock().expect("env lock poisoned");
    let _addr = EnvGuard::unset("PIXIE_ADDR");
    let _threads = EnvGuard::unset("PIXIE_THREADS");

    let path = write_temp_config("host: 127.0.0.1\nport: 8080\nworkers: 0\n");
    let _config = EnvGuard::set("PIXIE_CONFIG", path.to_str().expect("utf-8 path"));

    let config = runtime_config().expect("runtime config should load");
    assert_eq!(config.addr, "127.0.0.1:8080");
    assert_eq!(config.workers, 4);

    let _ = fs::remove_file(path);
}

#[test]
fn uses_hardcoded_defaults_when_no_yaml_is_found() {
    let _lock = ENV_LOCK.lock().expect("env lock poisoned");
    let _addr = EnvGuard::set("PIXIE_ADDR", "0.0.0.0:9999");
    let _threads = EnvGuard::set("PIXIE_THREADS", "32");

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time should be monotonic")
        .as_nanos();
    let missing_path = env::temp_dir().join(format!("pixie-missing-config-{nanos}.yml"));
    let _config = EnvGuard::set(
        "PIXIE_CONFIG",
        missing_path.to_str().expect("utf-8 path"),
    );

    let config = runtime_config().expect("runtime config should load");
    assert_eq!(config.addr, "127.0.0.1:80");
    assert_eq!(config.workers, 4);
}
