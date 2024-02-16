use crate::{group_call::GroupSel, read_string};
use std::{
    fmt,
    io::{Read, Write},
};
#[derive(Debug)]
pub struct FreqConf {
    pub frequency: f32,
    pub group_sel: Option<GroupSel>,
}

impl FreqConf {
    pub fn new(frequency: f32) -> Result<Option<Self>, String> {
        if (frequency < 134.0 || frequency > 174.0) && (frequency < 400.0 || frequency > 480.0) {
            return Err(String::from("Invalid Frequency"));
        }
        Ok(Some(Self {
            frequency,
            group_sel: None,
        }))
    }
    pub fn with_group_sel(
        frequency: f32,
        group_call: Option<GroupSel>,
    ) -> Result<Option<Self>, String> {
        let mut freq = FreqConf::new(frequency)?;
        if let Some(ref mut freq) = freq {
            freq.group_sel = group_call;
        }
        Ok(freq)
    }
}

#[derive(Debug)]
pub enum FmBandwidth {
    Wide,
    Narrow,
}

impl fmt::Display for FmBandwidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FmBandwidth::Wide => write!(f, "WBFM:25k"),
            FmBandwidth::Narrow => write!(f, "NBFM:12.5k"),
        }
    }
}

#[derive(Debug)]
pub struct Command {
    pub command: String,
    pub expected_response: String,
}
#[derive(Debug)]
pub struct Channel {
    pub bandwidth: FmBandwidth,
    pub tx_conf: Option<FreqConf>,
    pub rx_conf: Option<FreqConf>,
    pub squelch: u8,
}

impl Channel {
    pub fn default() -> Self {
        Self {
            bandwidth: FmBandwidth::Narrow,
            tx_conf: None,
            rx_conf: None,
            squelch: 4,
        }
    }
    pub fn bandwidth(mut self, bandwidth: FmBandwidth) -> Self {
        self.bandwidth = bandwidth;
        self
    }
    pub fn squelch(mut self, squelch: u8) -> Result<Self, String> {
        if squelch > 8 {
            return Err(String::from("invalid Squelch"));
        }
        self.squelch = squelch;
        Ok(self)
    }

    pub fn write_config<T: Read + Write>(&self, io: &mut T) -> Result<String, String> {
        let bw_string = match self.bandwidth {
            FmBandwidth::Wide => "0",
            FmBandwidth::Narrow => "1",
        };

        let tx_frequency: String;
        let tx_group: String;
        if let Some(ref freq_conf) = self.tx_conf {
            tx_frequency = format!("{:.4}", freq_conf.frequency);
            tx_group = match &freq_conf.group_sel {
                Some(group_sel) => format!("{}", group_sel),
                None => "0000".to_string(),
            };
        } else {
            return Err(String::from("Tx frequency is not specified!"));
        }

        let rx_frequency: String;
        let rx_group: String;
        if let Some(ref freq_conf) = self.rx_conf {
            rx_frequency = format!("{:.4}", freq_conf.frequency);
            rx_group = match &freq_conf.group_sel {
                Some(group_sel) => format!("{}", group_sel),
                None => "0000".to_string(),
            };
        } else {
            return Err(String::from("Rx frequency is not specified!"));
        }
        let command = format!(
            "AT+DMOSETGROUP={},{},{},{},{},{}\r\n",
            bw_string, tx_frequency, rx_frequency, tx_group, self.squelch, rx_group
        );
        io.write_all(command.as_bytes())
            .map_err(|e| e.to_string())?;
        let mut response = read_string(io)?;
        io.read_to_string(&mut response)
            .map_err(|e| e.to_string())?;
        if response.trim() != "+DMOSETGROUP=0" {
            return Err(format!("Invalid Response: {}", response));
        }
        Ok(response)
    }

    pub fn tx(mut self, tx_conf: Option<FreqConf>) -> Self {
        self.tx_conf = tx_conf;
        self
    }
    pub fn rx(mut self, rx_conf: Option<FreqConf>) -> Self {
        self.rx_conf = rx_conf;
        self
    }
}
