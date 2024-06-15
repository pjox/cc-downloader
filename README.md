# CC-Downloader

This is an experimental polite downloader for Common Crawl data writter in `rust`. Currently it downloads Common Crawl data from the Cloudfront.

## Todo

- [ ] Add retry support
- [ ] Add Python bindings
- [ ] Add tests
- [ ] Refactor CLI subcommands
- [ ] Add support for `s3`

## Usage

```bash
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

Usage: cc-downloader download --path-file <PATHS> --output <OUTPUT> [PROGRESS]

Arguments:
  [PROGRESS]  Print progress #[arg(short, long)] [possible values: true, false]

Options:
      --path-file <PATHS>  Path file
  -o, --output <OUTPUT>    Otput folder
  -h, --help               Print help

------

cc-downloader download-paths -h                                                               
Download paths for a given snapshot

Usage: cc-downloader download-paths --snapshot <SNAPSHOT> --data-type <PATHS> --output <OUTPUT> [PROGRESS]

Arguments:
  [PROGRESS]  Print progress #[arg(short, long)] [possible values: true, false]

Options:
      --snapshot <SNAPSHOT>  Crawl reference
      --data-type <PATHS>    Data type
  -o, --output <OUTPUT>      Otput folder
  -h, --help                 Print help
```