use clap::Parser;

mod day1;
mod day2;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	input_file: String,

	/// The day to do
	#[arg(short, long, default_value_t=1)]
	day: u8,
}

fn main() {
	let cli = Cli::parse();
	match cli.day {
	    1 => day1::process_input(cli.input_file),
		2 => day2::process_input(cli.input_file),
	    _ => panic!("Day not implemented yet!"),
	}
}
