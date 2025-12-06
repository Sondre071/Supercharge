use std::io::{BufRead, BufReader, Read};
use std::process::{Child, Command, Stdio};

pub fn run_and_collect_lines(binary: &str, args: Vec<String>) -> Result<Vec<String>, String> {
    let mut child = Command::new(binary)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to execute {}: {}", binary, e))?;

    let stdout = child.stdout.take().expect("Failed to get stdout");
    let stderr = child.stderr.take().expect("Failed to get stderr");

    let reader = BufReader::new(stdout);
    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    let mut stderr_reader = BufReader::new(stderr);
    let mut stderr_buf = String::new();
    stderr_reader.read_to_string(&mut stderr_buf).unwrap_or(0);

    let status = child.wait().expect("Failed to wait for child process");

    if !status.success() {
        return Err(format!(
            "Status: {} | stderr: {}",
            status,
            stderr_buf.trim()
        ));
    }

    Ok(lines)
}

pub fn run_streaming(binary: &str, args: Vec<String>) -> Result<StreamingReader, String> {
    let mut child = Command::new(binary)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to execute {}: {}", binary, e))?;

    let stdout = child.stdout.take().expect("Failed to get stdout");
    let stderr = child.stderr.take().expect("Failed to get stderr");

    Ok(StreamingReader {
        child,
        stdout: Some(Box::new(stdout)),
        stderr: Some(Box::new(stderr)),
    })
}

pub struct StreamingReader {
    child: Child,
    stdout: Option<Box<dyn Read + Send>>,
    stderr: Option<Box<dyn Read + Send>>,
}

impl StreamingReader {
    pub fn stdout(&mut self) -> &mut dyn Read {
        self.stdout.as_mut().expect("Stdout not available").as_mut()
    }

    pub fn wait(mut self) -> Result<(), String> {
        let mut stderr_buf = String::new();
        if let Some(stderr) = self.stderr.take() {
            let mut reader = BufReader::new(stderr);
            let _ = reader.read_to_string(&mut stderr_buf);
        }

        let status = self.child.wait().map_err(|e| e.to_string())?;

        if !status.success() {
            return Err(format!(
                "Status: {} | stderr: {}",
                status,
                stderr_buf.trim()
            ));
        }

        Ok(())
    }
}
