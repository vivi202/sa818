use std::{time::Duration};
use clap::{Parser, Subcommand,};
use serialport::SerialPort;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Specify serial port
    #[arg(short, long, value_name = "SERIAL", default_value = "/dev/ttyS0")]
    serial: String,
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// get version of sa818
    Version {},
}

fn main() {
    let cli = Cli::parse();
    println!("using serial port:{}",cli.serial);
    match cli.command {
        Some(Commands::Version {}) =>{
            let mut serial_io =open_serial(cli.serial);
            let result = sa818::get_version(&mut serial_io);
            println!("version: {}",result.unwrap())
        },
        None =>{},
    }
}

fn open_serial(serial_port: String) -> Box<dyn SerialPort>{
    let port = serialport::new(serial_port, 9_600)
    .timeout(Duration::from_millis(10))
    .data_bits(serialport::DataBits::Eight)
    .parity(serialport::Parity::None)
    .stop_bits(serialport::StopBits::One)
    .open().expect("Failed to open port");
    return  port;
}