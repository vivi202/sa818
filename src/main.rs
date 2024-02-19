use clap::{Args, Parser, Subcommand, ValueEnum};
use sa818::{
  channel::{Channel, FmBandwidth, FreqConf},
  group_call::GroupSel,
};
use serialport::SerialPort;
use std::{process::exit, time::Duration};

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
    /// Use the same dcs for both tx and rx
    /// Dcs format is code<N|I>
    #[arg(
            long,
            short,
            conflicts_with_all = ["rcts","rdcs","tcts","tdcs","ctcss"],
            verbatim_doc_comment
        )]
    dcs: Option<String>,
    #[arg(
            long,
            short,
            conflicts_with_all = ["rcts","rdcs","tcts","tdcs","dcs"],
        )]
    /// Use the same ctcss for both tx and rx
    ctcss: Option<String>,
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
  rcts: Option<String>,
  #[arg(long)]
  rdcs: Option<String>,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
struct TxGroupSel {
  #[arg(long)]
  tcts: Option<String>,
  #[arg(long)]
  tdcs: Option<String>,
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
      dcs,
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
          //By default without group selective
          chan.rx_conf = FreqConf::new(frequency).unwrap();
          chan.tx_conf = FreqConf::new(frequency).unwrap();

          if let Some(ctcss) = &ctcss {
            let code = sa818::group_call::parse_ctcss(&ctcss);
            if let Some(code) = code {
              setup_ctcss(&mut chan, frequency, code);
            } else {
              println!("{} ctcss is not valid", ctcss);
              exit(1);
            }
          }

          if let Some(dcs) = dcs {
            setup_dcs(dcs, &mut chan, frequency);
          }

          if let Some(receive_group) = receive_group {
            setup_rx_group(receive_group, &mut chan, frequency);
          }

          if let Some(transmit_group) = transmit_group {
            setup_tx_group(transmit_group, &mut chan, frequency);
          }

          dbg!(&chan);
          // chan.write_config(&mut serial_io).unwrap();
        }
        Some(Mode::Halfduplex {
          rxfrequency,
          txfrequency,
        }) => {
          let mut chan = Channel::default().bandwidth(bandwidth);
          //By default without group selective
          chan.rx_conf = FreqConf::new(rxfrequency).unwrap();
          chan.tx_conf = FreqConf::new(txfrequency).unwrap();
          if let Some(ctcss) = ctcss {
            let code = sa818::group_call::parse_ctcss(&ctcss);
            if let Some(code) = code {
              setup_halfduplex_ctcss(&mut chan, rxfrequency, code, txfrequency);
            } else {
              println!("{} ctcss is not valid", ctcss);
              exit(1);
            }
          }

          if let Some(dcs) = dcs {
            println!("dcs Exist");
            setup_halfduplex_dcs(dcs, &mut chan, rxfrequency, txfrequency);
          }

          if let Some(receive_group) = receive_group {
            setup_rx_group(receive_group, &mut chan, rxfrequency)
          }

          if let Some(transmit_group) = transmit_group {
            setup_tx_group(transmit_group, &mut chan, txfrequency)
          }
          // chan.write_config(&mut serial_io).unwrap();
          dbg!(chan);
        }
        None => todo!(),
      }
    }
    None => {}
  }
}

fn setup_halfduplex_dcs(dcs: String, chan: &mut Channel, rxfrequency: f32, txfrequency: f32) {
  let dcs = sa818::group_call::parse_dcs(dcs);
  match dcs {
    Ok(dcs) => {
      chan.rx_conf = FreqConf::with_group_sel(rxfrequency, dcs).unwrap();
      //Setup TX
      chan.tx_conf = FreqConf::with_group_sel(txfrequency, dcs).unwrap();
    }
    Err(e) => {
      println!("{}", e);
      exit(1);
    }
  }
}

fn setup_halfduplex_ctcss(chan: &mut Channel, rxfrequency: f32, ctcss: u8, txfrequency: f32) {
  //Setup RX
  chan.rx_conf = FreqConf::with_group_sel(rxfrequency, GroupSel::new_ctcss(ctcss)).unwrap();
  //Setup TX
  chan.tx_conf = FreqConf::with_group_sel(txfrequency, GroupSel::new_ctcss(ctcss)).unwrap();
}

fn setup_tx_group(transmit_group: TxGroupSel, chan: &mut Channel, frequency: f32) {
  if let Some(cts) = &transmit_group.tcts {
    let code = sa818::group_call::parse_ctcss(cts);
    if let Some(code) = code {
      chan.tx_conf = FreqConf::with_group_sel(frequency, GroupSel::new_ctcss(code)).unwrap();
    } else {
      println!("{} ctcss is not valid", cts);
      exit(1);
    }
  }
  if let Some(tdcs) = transmit_group.tdcs {
    let tdcs = sa818::group_call::parse_dcs(tdcs);
    match tdcs {
      Ok(tdcs) => {
        chan.tx_conf = FreqConf::with_group_sel(frequency, tdcs).unwrap();
      }
      Err(e) => println!("{}", e),
    }
  }
}

fn setup_rx_group(receive_group: RxGroupSel, chan: &mut Channel, frequency: f32) {
  if let Some(cts) = &receive_group.rcts {
    let code = sa818::group_call::parse_ctcss(cts);
    if let Some(code) = code {
      chan.rx_conf = FreqConf::with_group_sel(frequency, GroupSel::new_ctcss(code)).unwrap();
    } else {
      println!("{} ctcss is not valid", cts);
      exit(1);
    }
  }
  if let Some(rdcs) = receive_group.rdcs {
    let rdcs = sa818::group_call::parse_dcs(rdcs);
    match rdcs {
      Ok(rdcs) => {
        chan.rx_conf = FreqConf::with_group_sel(frequency, rdcs).unwrap();
      }
      Err(e) => println!("{}", e),
    }
  }
}

fn setup_dcs(dcs: String, chan: &mut Channel, frequency: f32) {
  let dcs = sa818::group_call::parse_dcs(dcs);
  match dcs {
    Ok(dcs) => {
      chan.rx_conf = FreqConf::with_group_sel(frequency, dcs).unwrap();
      //Setup TX
      chan.tx_conf = FreqConf::with_group_sel(frequency, dcs).unwrap();
    }
    Err(e) => {
      println!("{} invalid dcs", e);
      exit(1);
    }
  }
}

fn setup_ctcss(chan: &mut Channel, frequency: f32, ctcss: u8) {
  chan.rx_conf = FreqConf::with_group_sel(frequency, GroupSel::new_ctcss(ctcss)).unwrap();
  //Setup TX
  chan.tx_conf = FreqConf::with_group_sel(frequency, GroupSel::new_ctcss(ctcss)).unwrap();
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
