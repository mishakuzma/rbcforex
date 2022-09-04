use clap::Parser;
fn main() {
    let args = rust_forexcan::CliInputs::parse();

    // rust_forexcan::call_rbc(args.from_cur, args.to_cur);
    
    // Call handler
    // Because the handler is the only part where user input matters,
    // the handler takes ownership of the inputs.
    rust_forexcan::handleInput(args);    

    // Display result
}
