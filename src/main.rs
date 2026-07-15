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

impl Config {
    fn new(target: String) -> Config {
        Config {
            target,
            start_port: 1,
            end_port: 65535,
            scan_type: ScanType::Tcp,
            output_format: OutputFormat::PlainText,
            output_file: None,
            timeout_ms: 1000,
            max_concurrent: 1000,
            verbose: false,
            notes: None,
            target_name: None,
        }
    }
}

fn main() {
   
}
