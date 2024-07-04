# CC-Downloader

This is an experimental polite downloader for Common Crawl data writter in `rust`. Currently it downloads Common Crawl data from the Cloudfront.

## Todo

- [x] Add retry support
- [ ] Add Python bindings
- [ ] Add tests
- [ ] Refactor CLI subcommands
- [ ] Simplify CLI interface

## Usage

```text
Usage: cc-downloader [COMMAND]

Commands:
  download-paths  Download paths for a given snapshot
  download        Download files from a crawl
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

------

cc-downloader download -h
Download files from a crawl

Usage: cc-downloader download [OPTIONS] <PATHS> <DESTINATION> [PROGRESS]

Arguments:
  <PATHS>        Path file
  <DESTINATION>  Destination folder
  [PROGRESS]     Print progress [possible values: true, false]

Options:
  -n, --numbered  Enumerate output files for compatibility with Ungoliant Pipeline
  -h, --help      Print help

------

cc-downloader download-paths -h
Download paths for a given snapshot

Usage: cc-downloader download-paths <SNAPSHOT> <PATHS> <DESTINATION> [PROGRESS]

Arguments:
  <SNAPSHOT>     Crawl reference, e.g. CC-MAIN-2021-04
  <PATHS>        Data type
  <DESTINATION>  Destination folder
  [PROGRESS]     Print progress [possible values: true, false]

Options:
  -h, --help  Print help
```
