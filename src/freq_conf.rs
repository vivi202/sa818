use crate::group_call::GroupSel;

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
