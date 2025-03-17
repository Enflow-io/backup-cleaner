<p align="center">
  <img src="cleaner-logo.jpeg" alt="bat - a cat clone with wings"><br>
  <a href="https://github.com/Enflow-io/backup-cleaner/actions?query=workflow%3ACICD"><img src="https://github.com/Enflow-io/backup-cleaner/actions/workflows/rust.yml/badge.svg" alt="Build Status"></a>
  <img src="https://img.shields.io/crates/l/bat.svg" alt="license"><br />
  Configurable command-line tool for easily deleting unnecessary backup files.
</p>

## Backup cleaner

This app removes redundant backup copies and frees up space on your disk.


cargo run -- -p 1d-5 -f /Users/constantine/Projects/Rust/Backups -r "(\d{2}).(\d{2}).(\d{4})"

### How it works

1. Create .env file to config
2. Cron it up


### Config
period + qnt