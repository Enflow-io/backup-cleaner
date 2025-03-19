<p align="center">
  <img src="cleaner-logo.jpeg" alt="bat - a cat clone with wings"><br>
  <a href="https://github.com/Enflow-io/backup-cleaner/actions?query=workflow%3ACICD"><img src="https://github.com/Enflow-io/backup-cleaner/actions/workflows/rust.yml/badge.svg" alt="Build Status"></a>
  <img src="https://img.shields.io/crates/l/bat.svg" alt="license"><br />
  Configurable command-line tool for easily deleting unnecessary backup files.
</p>



System administrators often create backups on a daily basis and then struggle to delete unnecessary archive files. B-Cleaner solves this issue.

## Usage

```
bclean -p 1d-7 -f ./backups (1 config for the folder)
bclean -p 1d-7 -p 1w-3 -f ./backups -r "(\d{2}).(\d{2}).(\d{4})" (2 configs, folder and regexp to extract date from file)
```

