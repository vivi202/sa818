use clap::{Args, Parser, Subcommand, ValueEnum};
use sa818::{
    channel::{Channel, FmBandwidth, FreqConf},
    group_call::GroupSel,
};
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
    Channel {
        #[command(subcommand)]
        mode: Option<Mode>,
        #[arg(long, short, value_enum, default_value = "narrow")]
        bandwidth: Bandwidth,
        #[arg(
            long,
            conflicts_with = "rcts",
            conflicts_with = "rdcs",
            conflicts_with = "tcts",
            conflicts_with = "tdcs",
            default_value = "0"
        )]
        ctcss: Option<u8>,
        #[command(flatten)]
        receive_group: Option<RxGroupSel>,
        #[command(flatten)]
        transmit_group: Option<TxGroupSel>,
    },
}

#[derive(Args)]
#[group(required = false, multiple = false)]
struct RxGroupSel {
    #[arg(long)]
    rcts: Option<u8>,
    #[arg(long)]
    rdcs: Option<u32>,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
struct TxGroupSel {
    #[arg(long)]
    tcts: Option<u8>,
    #[arg(long)]
    tdcs: Option<u32>,
}

#[derive(Subcommand)]
enum Mode {
    Simplex {
        frequency: f32,
    },
    Halfduplex {
        #[arg(short, long, value_name = "RXFREQUENCY")]
        rxfrequency: f32,
        #[arg(short, long, value_name = "TXFREQUENCY")]
        txfrequency: f32,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Bandwidth {
    Wide,
    Narrow,
}
fn main() {
    let cli = Cli::parse();
    //    let mut serial_io: Box<dyn SerialPort> = open_serial(cli.serial);

    match cli.command {
        Some(Commands::Version) => {
            // let result = sa818::get_version(&mut serial_io);
            // println!("version: {}", result.unwrap())
        }
        Some(Commands::Rssi) => {
            // let result = sa818::get_rssi(&mut serial_io);
            // println!("RSSI: {}", result.unwrap())
        }
        Some(Commands::Channel {
            bandwidth,
            mode,
            ctcss,
            receive_group,
            transmit_group,
        }) => {
            let bandwidth = match bandwidth {
                Bandwidth::Wide => FmBandwidth::Wide,
                Bandwidth::Narrow => FmBandwidth::Narrow,
            };
            match mode {
                Some(Mode::Simplex { frequency }) => {
                    let mut chan = Channel::default().bandwidth(bandwidth);
                    //Setup RX
                    chan.rx_conf = setup_rx(receive_group, ctcss, frequency);
                    //Setup TX
                    chan.tx_conf = setup_tx(transmit_group, ctcss, frequency);
                    dbg!(&chan);
                    // chan.write_config(&mut serial_io).unwrap();
                }
                Some(Mode::Halfduplex {
                    rxfrequency,
                    txfrequency,
                }) => {
                    let mut chan = Channel::default().bandwidth(bandwidth);
                    //Setup RX
                    chan.rx_conf = setup_rx(receive_group, ctcss, rxfrequency);
                    //Setup TX
                    chan.tx_conf = setup_tx(transmit_group, ctcss, txfrequency);
                    // chan.write_config(&mut serial_io).unwrap();
                    dbg!(chan);
                }
                None => todo!(),
            }
        }
        None => {}
    }
}

fn setup_rx(
    receive_group: Option<RxGroupSel>,
    ctcss: Option<u8>,
    frequency: f32,
) -> Option<FreqConf> {
    match receive_group {
        Some(rec) => {
            if let Some(dcs) = rec.rdcs {
                return FreqConf::with_group_sel(
                    frequency,
                    GroupSel::new_dcs(dcs, sa818::group_call::DcsSuffix::Normal),
                )
                .unwrap();
            }
            if let Some(code) = rec.rcts {
                //TODO CONVERT CTCSS FREQ TO CODE
                return FreqConf::with_group_sel(frequency, GroupSel::new_ctcss(code)).unwrap();
            }
        }
        None => {
            if let Some(code) = ctcss {
                return FreqConf::with_group_sel(frequency, GroupSel::new_ctcss(code)).unwrap();
            }
        }
    }
    return None;
}

fn setup_tx(
    receive_group: Option<TxGroupSel>,
    ctcss: Option<u8>,
    frequency: f32,
) -> Option<FreqConf> {
    match receive_group {
        Some(rec) => {
            if let Some(dcs) = rec.tdcs {
                return FreqConf::with_group_sel(
                    frequency,
                    GroupSel::new_dcs(dcs, sa818::group_call::DcsSuffix::Normal),
                )
                .unwrap();
            }
            if let Some(code) = rec.tcts {
                //TODO CONVERT CTCSS FREQ TO CODE
                return FreqConf::with_group_sel(frequency, GroupSel::new_ctcss(code)).unwrap();
            }
        }
        None => {
            if let Some(code) = ctcss {
                return FreqConf::with_group_sel(frequency, GroupSel::new_ctcss(code)).unwrap();
            }
        }
    }
    return None;
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
