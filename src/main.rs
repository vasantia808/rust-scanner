enum ScanType {
    Tcp,
    Udp,
    Syn,
}

enum OutputFormat {
    Json,
    Csv,
    PlainText,
}

struct Config {
    target: String,
    start_port: u16,
    end_port: u16,
    scan_type: ScanType,
    output_format: OutputFormat,
    output_file: Option<String>,
    timeout_ms: u16,
    max_concurrent: u16,
    verbose: bool,
    notes: Option<String>,
    target_name: Option<String>,
}

fn main() {
   
}
