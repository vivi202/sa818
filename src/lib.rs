pub mod channel;
pub mod filter_config;
pub mod group_call;
pub mod tail_tone;
pub mod volume_config;
use std::io::{BufRead, BufReader, Read, Write};
use std::string;

use crate::channel::Channel;
use crate::filter_config::FilterConfig;
use crate::volume_config::VolumeConfig;
use tail_tone::TailTone;

struct Sa818Config {
    channel_conf: Option<Channel>,
    filter_conf: Option<FilterConfig>,
    tail_conf: Option<TailTone>,
    volume_conf: Option<VolumeConfig>,
}

impl Sa818Config {
    pub fn default() -> Self {
        Self {
            channel_conf: Some(Channel::default()),
            filter_conf: Some(filter_config::FilterConfig::default()),
            tail_conf: None,
            volume_conf: None,
        }
    }
}

pub fn handshake<T: Read + Write>(io: &mut T) -> Result<String, String> {
    io.write_all("AT+DMOCONNECT\r\n".as_bytes())
        .map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    io.read_to_string(&mut buffer).map_err(|e| e.to_string())?;
    if buffer.trim() != "+DMOCONNECT:0" {
        return Err(format!("Invalid Response: {}", buffer));
    }
    Ok(buffer)
}

pub fn get_version<T: Read + Write>(io: &mut T) -> Result<String, String> {
    io.write("AT+VERSION\r\n".as_bytes())
        .map_err(|e| e.to_string())?;
    let buffer = read_string(io)?;
    let mut splitted_buffer = buffer.trim().split(':');
    if splitted_buffer.next().unwrap() != "+VERSION" {
        return Err(format!("Invalid Response: {}", buffer));
    }
    Ok(splitted_buffer.next().unwrap().to_string())
}

pub fn get_rssi<T: Read + Write>(io: &mut T) -> Result<u8, String> {
    io.write_all("RSSI?\r\n".as_bytes())
        .map_err(|e| e.to_string())?;
    let buffer = read_string(io)?;
    let mut splitted_buffer = buffer.trim().split('=');
    if splitted_buffer.next().unwrap() != "RSSI" {
        return Err(format!("Invalid Response: {}", buffer));
    }
    //Get rssi value
    let rssi = splitted_buffer
        .next()
        .unwrap()
        .parse::<u8>()
        .map_err(|e| format!("Failed to parse rssi value: {}", e.to_string()))?;
    Ok(rssi)
}

fn read_string<T: Read + Write>(io: &mut T) -> Result<String, String> {
    let mut buf_reader = BufReader::new(io);
    let mut buffer = String::new();
    buf_reader
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
