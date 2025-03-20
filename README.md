<p align="center">
  <img src="cleaner-logo.jpeg" alt="bat - a cat clone with wings"><br>
  <a href="https://github.com/Enflow-io/backup-cleaner/actions?query=workflow%3ACICD"><img src="https://github.com/Enflow-io/backup-cleaner/actions/workflows/rust.yml/badge.svg" alt="Build Status"></a>
  <img src="https://img.shields.io/crates/l/bat.svg" alt="license"><br />
  A configurable command-line tool for efficiently deleting unnecessary backup files.
</p>


### Overview
System administrators frequently create daily backups but often struggle to remove outdated archive files. B-Cleaner automates this process, making backup management effortless. Simply append this command to the end of your backup script.

## Configuration Format
Configurations follow this format:
```
${period}-${quantity}
```

The period is parsed using the <a href="https://docs.rs/parse_duration/latest/parse_duration/">parse_duration</a> crate.

B-Cleaner deletes all backup files older than:
```
period * quantity
```


Example:
```
1d-3
```
removes everyrthing older 3 days.

Examples:
```
1d-7 // Keep the last 7 daily backups
1w-4 // Keep the last 4 weekly backups
1y-3 // Keep the last 3 yearly backups
```

## Usage
```
bcleaner -p 1d-7 -f ./backups
```
This applies one configuration to the ./backups folder.

```
bcleaner -p 1d-7 -p 1w-3 -f ./backups -r "(\d{2}).(\d{2}).(\d{4})"
```
This applies two arguments:
* One for the folder
* One using a regex to extract dates from filenames

## Install
Download <a href="https://github.com/Enflow-io/backup-cleaner/releases/tag/0.0.1">last release</a>

Make it executable:
```
chmod +x bcleaner
```