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

impl GroupSel {
    pub fn new_dcs(code: u32, suffix: DcsSuffix) -> Option<Self> {
        if code < 23 || code > 754 {
            return None;
        }
        Some(GroupSel::Dcs(code, suffix))
    }
    pub fn new_ctcss(code: u8) -> Option<Self> {
        if code == 0 || code > 38 {
            return None;
        }
        Some(GroupSel::Ctcss(code))
    }
}

pub fn parse_dcs(mut dcs_string: String) -> Result<Option<GroupSel>, String> {
    let last = dcs_string.pop();
    match last {
        Some(char) => {
            let code = dcs_string.parse::<u32>().map_err(|e| e.to_string())?;
            match char {
                'N' => {
                    return Ok(Some(GroupSel::Dcs(code, DcsSuffix::Normal)));
                }
                'I' => {
                    return Ok(Some(GroupSel::Dcs(code, DcsSuffix::Inverted)));
                }
                _ => {
                    return Err(format!("Invalid dcs suffix {}", char));
                }
            }
        }
        None => Err("Error Empty dcs string".to_string()),
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
