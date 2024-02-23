use core::fmt;

#[derive(Debug, Clone, Copy)]
pub enum DcsSuffix {
  Inverted,
  Normal,
}

#[derive(Debug, Clone, Copy)]
pub enum GroupSel {
  Ctcss(u8),
  Dcs(u32, DcsSuffix),
}

const CTCSS_FREQ: [&'static str; 39] = [
  "0", "67.0", "71.9", "74.4", "77.0", "79.7", "82.5", "85.4", "88.5", "91.5", "94.8", "97.4",
  "100.0", "103.5", "107.2", "110.9", "114.8", "118.8", "123.0", "127.3", "131.8", "136.5",
  "141.3", "146.2", "151.4", "156.7", "162.2", "167.9", "173.8", "179.9", "186.2", "192.8",
  "203.5", "210.7", "218.1", "225.7", "233.6", "241.8", "250.3",
];

impl GroupSel {
  pub fn new_dcs(code: u32, suffix: DcsSuffix) -> Result<Self, String> {
    if code < 23 || code > 754 {
      return Err(format!("Invalid ctcss code {}", code));
    }
    Ok(GroupSel::Dcs(code, suffix))
  }
  pub fn new_ctcss(code: u8) -> Result<Self, String> {
    if code == 0 || code > 38 {
      return Err(format!("Invalid ctcss code {}", code));
    }
    Ok(GroupSel::Ctcss(code))
  }
}

pub fn parse_dcs(mut dcs_string: String) -> Result<GroupSel, String> {
  let last = dcs_string.pop();
  match last {
    Some(char) => {
      let code = dcs_string.parse::<u32>().map_err(|e| e.to_string())?;
      match char {
        'N' => {
          return Ok(GroupSel::Dcs(code, DcsSuffix::Normal));
        }
        'I' => {
          return Ok(GroupSel::Dcs(code, DcsSuffix::Inverted));
        }
        _ => {
          return Err(format!("Invalid dcs suffix {}", char));
        }
      }
    }
    None => Err("Error Empty dcs string".to_string()),
  }
}

pub fn parse_ctcss(ctcss: &String) -> Result<GroupSel, String> {
  let code = CTCSS_FREQ.iter().position(|&f| f == ctcss);
  match code {
    Some(code) => Ok(GroupSel::Ctcss(code as u8)),
    None => Err(format!("{} is not a valid ctcss", ctcss)),
  }
}

impl fmt::Display for GroupSel {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      GroupSel::Ctcss(code) => {
        write!(f, "{}", code)
      }
      GroupSel::Dcs(code, suffix) => match suffix {
        crate::group_call::DcsSuffix::Inverted => {
          write!(f, "{}I", code)
        }
        crate::group_call::DcsSuffix::Normal => {
          write!(f, "{}N", code)
        }
      },
    }
  }
}
