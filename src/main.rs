use clap::Parser;
fn main() {
    let args = rbcforex::CliInputs::parse();

    // rust_forexcan::call_rbc(args.from_cur, args.to_cur);
    
    // Call handler
    // Because the handler is the only part where user input matters,
    // the handler takes ownership of the inputs.
    rbcforex::handle_input(args);    

    // Display result
}
