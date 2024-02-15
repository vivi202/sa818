use core::fmt;

#[derive(Debug)]
pub enum DcsSuffix {
    Inverted,
    Normal,
}

#[derive(Debug)]
pub enum GroupSel {
    Ctcss(u32),
    Dcs(u32, DcsSuffix),
}

impl GroupSel {
    pub fn new_dcs(code: u32, suffix: DcsSuffix) -> Option<Self> {
        if code < 23 || code > 754 {
            return None;
        }
        Some(GroupSel::Dcs(code, suffix))
    }
    pub fn new_ctcss(code: u32) -> Option<Self> {
        if code == 0 || code > 38 {
            return None;
        }
        Some(GroupSel::Ctcss(code))
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
