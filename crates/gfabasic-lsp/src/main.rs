use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    gfabasic_lsp::run_stdio(&mut BufReader::new(stdin.lock()), &mut stdout.lock())
}
