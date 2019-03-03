//! Various utilities.

use crate::errors::{bail, Result, ResultExt};

use log::trace;
use std;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::BuildHasher;
use std::hash::Hash;
use std::io::stdout;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::sync::Mutex;

#[cfg(windows)]
/// Returns proper executable file suffix on current platform.
/// Returns `".exe"` on Windows and `""` on other platforms.
pub fn exe_suffix() -> &'static str {
    ".exe"
}

#[cfg(not(windows))]
/// Returns proper executable file suffix on current platform.
/// Returns `".exe"` on Windows and `""` on other platforms.
pub fn exe_suffix() -> &'static str {
    ""
}

/// Creates and empty collection at `hash[key]` if there isn't one already.
/// Adds `value` to `hash[key]` collection.
pub fn add_to_multihash<K, T, V, S>(hash: &mut HashMap<K, V, S>, key: K, value: T)
where
    K: Eq + Hash + Clone,
    V: Default + Extend<T>,
    S: BuildHasher,
{
    use std::collections::hash_map::Entry;
    match hash.entry(key) {
        Entry::Occupied(mut entry) => entry.get_mut().extend(std::iter::once(value)),
        Entry::Vacant(entry) => {
            let mut r = V::default();
            r.extend(std::iter::once(value));
            entry.insert(r);
        }
    }
}

/// Runs a command and checks that it was successful
pub fn run_command(command: &mut Command) -> Result<()> {
    trace!("Executing command: {:?}", command);
    let status = command
        .status()
        .with_context(|_| format!("failed to run command: {:?}", command))?;
    if status.success() {
        Ok(())
    } else {
        bail!("command failed with {}: {:?}", status, command);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutput {
    pub status: ::std::process::ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

impl CommandOutput {
    pub fn is_success(&self) -> bool {
        self.status.success()
    }
}

/// Runs a command and returns its output regardless of
/// whether it was successful
pub fn run_command_and_capture_output(command: &mut Command) -> Result<CommandOutput> {
    trace!("Executing command: {:?}", command);
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());
    let output = command
        .output()
        .with_context(|_| format!("failed to run command: {:?}", command))?;
    Ok(CommandOutput {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        status: output.status,
    })
}

/// Runs a command and returns its stdout if it was successful
pub fn get_command_output(command: &mut Command) -> Result<String> {
    trace!("Executing command: {:?}", command);
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());
    let output = command
        .output()
        .with_context(|_| format!("failed to run command: {:?}", command))?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)
            .with_context(|_| "comand output is not valid unicode")?)
    } else {
        use std::io::Write;
        let mut stderr = std::io::stderr();
        writeln!(stderr, "Stdout:")?;
        stderr
            .write_all(&output.stdout)
            .with_context(|_| "output failed")?;
        writeln!(stderr, "Stderr:")?;
        stderr
            .write_all(&output.stderr)
            .with_context(|_| "output failed")?;
        bail!("command failed with {}: {:?}", output.status, command);
    }
}

/// Perform a map operation that can fail
pub trait MapIfOk<A> {
    /// Call closure `f` on each element of the collection and return
    /// `Vec` of values returned by the closure. If closure returns `Err`
    /// at some iteration, return that `Err` instead.
    fn map_if_ok<B, E, F: FnMut(A) -> std::result::Result<B, E>>(
        self,
        f: F,
    ) -> std::result::Result<Vec<B>, E>;
}

impl<A, T: IntoIterator<Item = A>> MapIfOk<A> for T {
    fn map_if_ok<B, E, F>(self, f: F) -> std::result::Result<Vec<B>, E>
    where
        F: FnMut(A) -> std::result::Result<B, E>,
    {
        self.into_iter().map(f).collect()
    }
}

/// Reads environment variable `env_var_name`, adds `new_paths`
/// to acquired list of paths and returns the list formatted as path list
/// (without applying it).
pub fn add_env_path_item(
    env_var_name: &str,
    mut new_paths: Vec<PathBuf>,
) -> Result<std::ffi::OsString> {
    use std::env;
    for path in env::split_paths(&env::var(env_var_name).unwrap_or_default()) {
        if new_paths.iter().find(|&x| x == &path).is_none() {
            new_paths.push(path);
        }
    }
    Ok(env::join_paths(new_paths).with_context(|_| "env::join_paths failed")?)
}

pub trait Inspect {
    fn inspect(self, text: impl Display) -> Self;
}

impl<T: Debug> Inspect for T {
    fn inspect(self, text: impl Display) -> Self {
        println!("{} {:?}", text, self);
        self
    }
}

#[derive(Debug)]
struct ProgressBarInner {
    message: String,
    count: u64,
    pos: u64,
    last_line_len: usize,
}

#[derive(Clone, Debug)]
pub struct ProgressBar(Arc<Mutex<ProgressBarInner>>);

impl ProgressBar {
    pub fn new(count: u64, message: impl Into<String>) -> Self {
        let mut progress_bar = ProgressBarInner {
            count,
            message: message.into(),
            pos: 0,
            last_line_len: 0,
        };
        progress_bar.print();
        ProgressBar(Arc::new(Mutex::new(progress_bar)))
    }

    pub fn add(&self, n: u64) {
        self.0.lock().unwrap().inc(n);
    }
}

impl ProgressBarInner {
    fn clear_line(&self) {
        print!("\r");
        for _ in 0..self.last_line_len {
            print!(" ");
        }
        print!("\r");
    }
    fn print(&mut self) {
        self.clear_line();
        let message = format!("{}: {} / {}", self.message, self.pos, self.count);
        self.last_line_len = message.len();
        print!("{}\r", message);
        stdout().flush().unwrap();
    }

    fn inc(&mut self, n: u64) {
        self.pos += n;
        self.print();
    }
}
