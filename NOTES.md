# Scanner Design Notes

## Config Struct Fields
- `target: String` — IP address or domain to scan
- `start_port: u16` — beginning of port range
- `end_port: u16` — end of port range
- `scan_type: ScanType` — enum, TCP/UDP/SYN
- `output_format: OutputFormat` — enum, JSON/CSV/PlainText
- `output_file: Option<String>` — optional file output
- `timeout_ms: u16` — timeout in milliseconds, default 1000
- `max_concurrent: u16` — concurrent threads, default 1000
- `verbose: bool` — detailed output, default false
- `notes: Option<String>` — free form target notes
- `target_name: Option<String>` — human readable scan label

## Design Decisions
- Using enums for ScanType and OutputFormat instead of Strings
- output_file is Option<String> because file output is optional
- timeout stored in milliseconds for precision
