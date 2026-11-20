#![allow(dead_code)]

use std::{
    env, fs,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    sync::Mutex,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Global lock to serialize tests that modify the environment.
pub static ENV_LOCK: Mutex<()> = Mutex::new(());

/// Acquires the environment lock even if a previous test panicked.
pub fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.lock().unwrap_or_else(|err| err.into_inner())
}

/// Guard that restores an environment variable when a test ends.
pub struct EnvGuard {
    key: &'static str,
    old: Option<String>,
}

impl EnvGuard {
    /// Sets an environment variable for the lifetime of the scope.
    pub fn set(key: &'static str, value: &str) -> Self {
        let old = env::var(key).ok();
        unsafe { env::set_var(key, value) };
        Self { key, old }
    }
}

impl Drop for EnvGuard {
    /// Restores the previous value of the environment variable.
    fn drop(&mut self) {
        match &self.old {
            Some(value) => unsafe { env::set_var(self.key, value) },
            None => unsafe { env::remove_var(self.key) },
        }
    }
}

/// Temporary file that is removed automatically.
pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    /// Creates a temporary YAML file with the provided content.
    pub fn new(prefix: &str, content: &str) -> Self {
        let path = unique_path(prefix, "yml");
        fs::write(&path, content).expect("could not write temporary file");
        Self { path }
    }

    /// Returns the temporary file path as UTF-8.
    pub fn as_str(&self) -> &str {
        self.path.to_str().expect("chemin non UTF-8")
    }
}

impl Drop for TempFile {
    /// Removes the temporary file if it still exists.
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

/// Temporary directory that is removed automatically.
pub struct TempDir {
    /// Absolute path to the temporary directory.
    pub path: PathBuf,
}

impl TempDir {
    /// Creates a new empty temporary directory.
    pub fn new(prefix: &str) -> Self {
        let path = unique_path(prefix, "dir");
        fs::create_dir_all(&path).expect("could not create temporary directory");
        Self { path }
    }

    /// Writes a file in the temporary directory.
    pub fn write(&self, relative: &str, content: &str) {
        let file = self.path.join(relative);
        if let Some(parent) = file.parent() {
            fs::create_dir_all(parent).expect("could not create parent directory");
        }
        fs::write(file, content).expect("could not write temporary file");
    }
}

impl Drop for TempDir {
    /// Recursively removes the temporary directory.
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

/// Builds a unique temporary path.
pub fn unique_path(prefix: &str, suffix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is invalid")
        .as_nanos();

    env::temp_dir().join(format!("{prefix}-{nanos}.{suffix}"))
}

/// Reserves an available local loopback port.
pub fn reserve_local_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind local impossible");
    let port = listener
        .local_addr()
        .expect("adresse locale introuvable")
        .port();
    drop(listener);
    port
}

/// Waits until a TCP server is reachable or the timeout expires.
pub fn wait_for_server(addr: &str, timeout: Duration) {
    let start = SystemTime::now();

    loop {
        if TcpStream::connect(addr).is_ok() {
            return;
        }

        let elapsed = SystemTime::now()
            .duration_since(start)
            .expect("duration is invalid");

        if elapsed > timeout {
            panic!("server at {addr} was unavailable after {timeout:?}");
        }

        thread::sleep(Duration::from_millis(20));
    }
}

/// Sends a raw HTTP request and returns the complete response.
pub fn send_http_request(addr: &str, request: &str) -> io::Result<String> {
    let mut stream = TcpStream::connect(addr)?;
    stream.write_all(request.as_bytes())?;
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf)?;
    String::from_utf8(buf)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))
}
