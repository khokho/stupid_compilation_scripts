use anyhow::{Context, Result};
use std::fs::{File, read_to_string};
use std::io::{stdout, Read, Write};
use std::process::Command;
use std::time::{Duration, Instant};

use clap::{App, Arg};
use sha2::{Digest, Sha256};

#[derive(thiserror::Error, Debug)]
#[error("compile failed")]
struct CompileError();

#[derive(thiserror::Error, Debug)]
#[error("run returned nonzero")]
struct NonZeroRun();

enum CodeState {
    Changed(Vec<u8>),
    Same,
}

enum RunResult {
    Ran(Duration),
    Failed,
}

fn compile(path: &str, out: &str) -> Result<()> {
    let mut cmd = Command::new("g++");
    cmd.arg("-g");
    cmd.arg(path);
    cmd.arg("-o");
    cmd.arg(out);

    if cmd.spawn()?.wait()?.success() {
        return Ok(());
    } else {
        return Err(anyhow::Error::new(CompileError()));
    }
}

fn run(ex: &str, input: &str) -> Result<RunResult> {
    println!("**********INPUT************");
    println!("");

    print!("{}", read_to_string(input).context(format!("File not found: {}", input))?);

    println!("");
    println!("----------OUTPUT-----------");

    let mut cmd = Command::new(ex);
    cmd.stdin(File::open(input)?);

    let tic = Instant::now();

    // runs actual program
    let run_success = cmd.spawn()?.wait()?.success();

    let time = tic.elapsed();

    println!();
    println!("Time: {}", time.as_secs_f64());
    println!();

    if run_success {
        Ok(RunResult::Ran(time))
    } else {
        Ok(RunResult::Failed)
    }
}

fn debug(ex: &str, input: &str) -> Result<bool> {
    let mut cmd = Command::new("gdb");
    cmd.arg("-q");
    cmd.arg("-ex").arg(format!("r < {}", input));
    cmd.arg("-ex").arg("bt");
    cmd.arg("-ex").arg("q");
    cmd.arg(ex);

    let mut handle = cmd.spawn()?;
    let res = handle.wait()?;

    Ok(res.success())
}

fn check_code_changed(code: &str) -> Result<CodeState> {
    let mut hasher = Sha256::new();
    let mut buf = vec![];

    File::open(code)
        .context(format!("Code file not found: {}", code))?
        .read_to_end(&mut buf)?;
    hasher.update(buf);
    let code_hash = hasher.finalize();

    let mut last_hash = vec![];
    if let Ok(mut f) = File::open(".codehash") {
        f.read_to_end(&mut last_hash)?;
    } else {
        return Ok(CodeState::Changed(code_hash.to_vec()));
    }

    if last_hash.as_slice() != code_hash.as_slice() {
        return Ok(CodeState::Changed(code_hash.to_vec()));
    } else {
        return Ok(CodeState::Same);
    }
}

fn main() -> Result<()> {
    let matches = App::new("Ravi program runner")
        .version("0.1.0")
        .author("Aleksandre K. <skhokhi@gmail.com>")
        .about("Ravi gaushvebs programas da ganaxebs pasuxs.")
        .arg(
            Arg::with_name("code")
                .index(1)
                .default_value("./x.cpp")
                .help("Sets code file"),
        )
        .arg(
            Arg::with_name("output")
                .index(2)
                .default_value("./ex")
                .help("Sets executable output file"),
        )
        .arg(
            Arg::with_name("infile")
                .short("-i")
                .default_value("./in.in")
                .help("Sets file that will be programs input"),
        )
        .get_matches();

    let code = matches.value_of("code").unwrap();
    let output = matches.value_of("output").unwrap();
    let infile = matches.value_of("infile").unwrap();

    if let CodeState::Changed(code_hash) = check_code_changed(code)? {
        println!("Compiling!...");
        compile(code, output)?;
        File::create(".codehash")?.write(&code_hash)?;
    }

    let run_result = run(output, infile)?;

    match run_result {
        RunResult::Failed => {
            debug(output, infile)?;
        }
        _ => {}
    }

    Ok(())
}
