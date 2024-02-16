use clap::{Parser, Subcommand, ValueEnum, Args};
use sa818::channel::Channel;
use serialport::SerialPort;
use std::time::Duration;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Specify serial port
    #[arg(short, long, value_name = "SERIAL", default_value = "/dev/ttyS1")]
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
    Version,
    /// get RSSI value
    Rssi,
    #[command(arg_required_else_help = true)]
    /// configure tx, rx frequency and group selective(CTCSS OR DCS) 
    Channel{
        #[command(subcommand)]
        mode: Option<Mode>,
        #[arg(long,short,value_enum,default_value = "narrow")]
        bandwidth: Bandwidth,
        #[command(flatten)]
        receive_group: RxGroupSel,
        #[command(flatten)]
        transmit_group: TxGroupSel
    }
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct RxGroupSel {
    #[arg(long)]
    rcts: Option<f32>,
    #[arg(long)]
    rdcs: Option<u8>
}


#[derive(Args)]
#[group(required = true, multiple = false)]
struct TxGroupSel {
    #[arg(long)]
    tcts: Option<f32>,
    #[arg(long)]
    tdcs: Option<u8>
}


#[derive(Subcommand)]
enum Mode {
    Simplex{
        #[arg(short, long, value_name = "FREQUENCY")]
        frequency: f32
    },
    Halfduplex
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Bandwidth {
    Wide,
    Narrow
}
fn main() {
    let cli = Cli::parse();
    let mut serial_io: Box<dyn SerialPort> = open_serial(cli.serial);

    match cli.command {
        Some(Commands::Version) => {
            let result = sa818::get_version(&mut serial_io);
            println!("version: {}", result.unwrap())
        }
        Some(Commands::Rssi) =>{
            let result = sa818::get_rssi(&mut serial_io);
            println!("RSSI: {}", result.unwrap())
        }
        Some(Commands::Channel { bandwidth ,mode, receive_group, transmit_group }) =>{
            match mode {
                Some(Mode::Simplex { frequency }) => {
                    
                },
                Some(Mode::Halfduplex) => todo!(),
                None => todo!(),
            }
        }
        None => {}
    }
}

fn open_serial(serial_port: String) -> Box<dyn SerialPort> {
    let port = serialport::new(serial_port, 9600)
        .timeout(Duration::from_millis(1000))
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .open()
        .expect("Failed to open port");
    return port;
}
