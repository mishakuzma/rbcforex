use clap::Parser;
use rust_forexcan;
fn main() {
    // users will submit two arguments telling us what currencies are involved
    let args = rust_forexcan::CliInputs::parse();

    rust_forexcan::call_rbc(args.from_cur, args.to_cur);
}
