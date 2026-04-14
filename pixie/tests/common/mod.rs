#![allow(dead_code)]

use std::{
    env, fs,
    io::{self, Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    path::{Path, PathBuf},
    sync::Mutex,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Verrou global pour sérialiser les tests qui modifient l'environnement.
pub static ENV_LOCK: Mutex<()> = Mutex::new(());

/// Récupère le verrou environnement même si un test précédent a paniqué.
pub fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.lock().unwrap_or_else(|err| err.into_inner())
}

/// Garde qui restaure une variable d'environnement en fin de test.
pub struct EnvGuard {
    key: &'static str,
    old: Option<String>,
}

impl EnvGuard {
    /// Définit une variable d'environnement pour la durée du scope.
    pub fn set(key: &'static str, value: &str) -> Self {
        let old = env::var(key).ok();
        unsafe { env::set_var(key, value) };
        Self { key, old }
    }
}

impl Drop for EnvGuard {
    /// Restaure la valeur précédente de la variable d'environnement.
    fn drop(&mut self) {
        match &self.old {
            Some(value) => unsafe { env::set_var(self.key, value) },
            None => unsafe { env::remove_var(self.key) },
        }
    }
}

/// Fichier temporaire supprimé automatiquement.
pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    /// Crée un fichier temporaire YAML avec le contenu donné.
    pub fn new(prefix: &str, content: &str) -> Self {
        let path = unique_path(prefix, "yml");
        fs::write(&path, content).expect("écriture du fichier temporaire impossible");
        Self { path }
    }

    /// Retourne le chemin UTF-8 du fichier temporaire.
    pub fn as_str(&self) -> &str {
        self.path.to_str().expect("chemin non UTF-8")
    }
}

impl Drop for TempFile {
    /// Supprime le fichier temporaire s'il existe encore.
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

/// Dossier temporaire supprimé automatiquement.
pub struct TempDir {
    /// Chemin absolu du dossier temporaire.
    pub path: PathBuf,
}

impl TempDir {
    /// Crée un nouveau dossier temporaire vide.
    pub fn new(prefix: &str) -> Self {
        let path = unique_path(prefix, "dir");
        fs::create_dir_all(&path).expect("création du dossier temporaire impossible");
        Self { path }
    }

    /// Écrit un fichier dans le dossier temporaire.
    pub fn write(&self, relative: &str, content: &str) {
        let file = self.path.join(relative);
        if let Some(parent) = file.parent() {
            fs::create_dir_all(parent).expect("création du parent impossible");
        }
        fs::write(file, content).expect("écriture du fichier temporaire impossible");
    }
}

impl Drop for TempDir {
    /// Supprime récursivement le dossier temporaire.
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

/// Construit un chemin temporaire unique.
pub fn unique_path(prefix: &str, suffix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("horloge système invalide")
        .as_nanos();

    env::temp_dir().join(format!("{prefix}-{nanos}.{suffix}"))
}

/// Réserve un port local disponible en loopback.
pub fn reserve_local_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind local impossible");
    let port = listener
        .local_addr()
        .expect("adresse locale introuvable")
        .port();
    drop(listener);
    port
}

/// Attend qu'un serveur TCP soit joignable jusqu'au timeout.
pub fn wait_for_server(addr: &str, timeout: Duration) {
    let start = SystemTime::now();

    loop {
        if TcpStream::connect(addr).is_ok() {
            return;
        }

        let elapsed = SystemTime::now()
            .duration_since(start)
            .expect("durée invalide");

        if elapsed > timeout {
            panic!("serveur non disponible à {addr} après {timeout:?}");
        }

        thread::sleep(Duration::from_millis(20));
    }
}

/// Envoie une requête HTTP brute et renvoie la réponse complète.
pub fn send_http_request(addr: &str, request: &str) -> io::Result<String> {
    let mut stream = TcpStream::connect(addr)?;
    stream.write_all(request.as_bytes())?;
    stream.shutdown(Shutdown::Write)?;

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf)?;
    String::from_utf8(buf)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))
}

/// Retourne vrai si le chemin pointe vers un dossier existant.
pub fn is_dir(path: &Path) -> bool {
    path.is_dir()
}
