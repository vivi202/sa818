use std::fmt;

use crate::freq_conf::FreqConf;

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

    pub fn generate_command(&self) -> Result<Command, String> {
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

        Ok(Command {
            //AT+DMOSETGROUP=BW，TX_F，RX_F，Tx_subaudio，SQ，Rx_subaudio
            command: format!(
                "AT+DMOSETGROUP={},{},{},{},{},{}",
                bw_string, tx_frequency, rx_frequency, tx_group, self.squelch, rx_group
            ),
            expected_response: "+DMOSETGROUP:0".to_string(),
        })
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
