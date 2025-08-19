use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, process::Stdio, time::Duration};
use tempfile::TempDir;
use tokio::{io::AsyncReadExt, process::Command, time::timeout};
use which::which;

mod m;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TaskSpec {
    name: String,
    // Prompt wäre für ein LLM – hier rein informativ
    prompt: String,
    // sehr simple Tests: input -> expected output als JSON-Strings
    tests: Vec<(u64, bool)>,
    // Signatur & Funktionsnamen sind hier fix für den POC
}

#[derive(Debug)]
struct VerifyResult {
    passed: usize,
    total: usize,
    stdout: String,
    stderr: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    ensure_cargo()?;

    // 1) Task definieren (Beispiel: is_prime)
    let task = make_is_prime_task();

    // 2) „Solver“ – heute hart codierte Kandidaten (morgen: LLM)
    let candidates = vec![
        // absichtlich schlechte Lösung (immer false)
        r#"pub fn is_prime(n: u64) -> bool { false }"#.to_string(),
        // naive Lösung
        r#"
        pub fn is_prime(n: u64) -> bool {
            if n < 2 { return false; }
            if n % 2 == 0 { return n == 2; }
            let mut i = 3u64;
            while i*i <= n {
                if n % i == 0 { return false; }
                i += 2;
            }
            true
        }"#.to_string(),
    ];

    // 3) Verify + Reward
    let mut best: Option<(usize, VerifyResult, String)> = None;

    for (idx, code) in candidates.iter().enumerate() {
        println!("== Kandidat #{idx} testen ==");
        let verify = verify_candidate(&task, code).await?;
        let reward = verify.passed;
        println!(
            "Ergebnis: {}/{} Tests bestanden\nSTDERR:\n{}\n",
            verify.passed, verify.total, verify.stderr
        );

        if best.as_ref().map(|b| reward > b.0).unwrap_or(true) {
            best = Some((reward, verify, code.clone()));
        }
    }

    if let Some((reward, _vr, code)) = best {
        println!("== Bester Kandidat mit Reward {reward} ==");
        println!("{code}");
    }

    Ok(())
}

fn ensure_cargo() -> Result<()> {
    which("cargo")
        .map(|_| ())
        .context("`cargo` nicht gefunden – bitte Rust installieren (rustup).")
}

fn make_is_prime_task() -> TaskSpec {
    // ein paar deterministische Tests
    let tests = vec![
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (17, true),
        (18, false),
        (19, true),
        (20, false),
        (7919, true),
    ];

    TaskSpec {
        name: "is_prime".into(),
        prompt: "Implementiere fn is_prime(n: u64) -> bool".into(),
        tests,
    }
}

async fn verify_candidate(task: &TaskSpec, lib_code: &str) -> Result<VerifyResult> {
    // 1) Temp-Crate anlegen
    let dir = TempDir::new()?;
    init_lib_crate(dir.path()).await?;

    // 2) src/lib.rs schreiben
    fs::create_dir_all(dir.path().join("src"))?;
    fs::write(dir.path().join("src/lib.rs"), lib_code)?;

    // 3) tests/solver.rs schreiben
    let tests_content = make_tests_module(task);
    fs::create_dir_all(dir.path().join("tests"))?;
    fs::write(dir.path().join("tests/solver.rs"), tests_content)?;

    // 4) cargo test mit Timeout
    let mut cmd = Command::new("cargo");
    cmd.arg("test")
        .arg("--quiet")
        .current_dir(dir.path())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let child_fut = cmd.spawn()?.wait_with_output();
    let output = timeout(Duration::from_secs(15), child_fut)
        .await
        .map_err(|_| anyhow!("Testlauf timed out"))??;

    let mut stdout = String::new();
    let mut stderr = String::new();
    stdout.push_str(&String::from_utf8_lossy(&output.stdout));
    stderr.push_str(&String::from_utf8_lossy(&output.stderr));

    // 5) sehr simple Auswertung: zähle „test tests::case_… … ok“
    let total = task.tests.len();
    let passed = count_passed(&stdout, total);

    Ok(VerifyResult {
        passed,
        total,
        stdout,
        stderr,
    })
}

fn count_passed(stdout: &str, total: usize) -> usize {
    // minimalistisch: wir zählen „ok“ in testzeilen
    let mut ok = 0usize;
    for line in stdout.lines() {
        // z.B.: "test tests::case_000 ... ok"
        if line.contains("test tests::case_") && line.trim_end().ends_with("ok") {
            ok += 1;
        }
    }
    ok.min(total)
}

async fn init_lib_crate(path: &Path) -> Result<()> {
    // `cargo init --lib`
    let status = Command::new("cargo")
        .arg("init")
        .arg("--lib")
        .arg("--quiet")
        .arg(path)
        .status()
        .await?;
    if !status.success() {
        return Err(anyhow!("cargo init fehlgeschlagen"));
    }

    // minimalen Cargo.toml sichern (Edition 2021 reicht für POC)
    let cargo_toml = r#"[package]
name = "candidate"
version = "0.1.0"
edition = "2021"
[dependencies]
"#;
    fs::write(path.join("Cargo.toml"), cargo_toml)?;
    Ok(())
}

fn make_tests_module(task: &TaskSpec) -> String {
    // generiert einen Test pro (input, expected)
    // bindet `use candidate::*;` ein und ruft is_prime auf
    let mut cases = String::new();
    for (i, (inp, exp)) in task.tests.iter().enumerate() {
        cases.push_str(&format!(
            r#"
    #[test]
    fn case_{i:03}() {{
        let got = candidate::is_prime({inp}u64);
        assert_eq!(got, {exp});
    }}
"#,
        ));
    }

    format!(
        r#"
mod tests {{
    // Tests greifen auf crate-Namen aus Cargo.toml („candidate“) zu
    {}
}}
"#,
        cases
    )
}
