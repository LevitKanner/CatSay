use assert_cmd::prelude::*;   //Add methods on commands
use std::process::Command;  //Runs programs
use predicates::prelude::*;


#[test]
fn run_with_defaults() -> Result< (), Box<dyn std::error::Error>> {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hey there!"));

        Ok(())
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>>{
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .args(&["-f" , "no/such/file.txt"])
        .assert()
        .failure();

        Ok(())
}