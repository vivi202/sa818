//To write a full Channell configuratio to sa818 two serial commands are needed

use sa818::channel::Channel;
use sa818::filter_config::FilterConfig;
use sa818::filter_config::FilterState;
use sa818::freq_conf::FreqConf;
use sa818::group_call::DcsSuffix;
use sa818::group_call::GroupSel;

fn main() {
    let channel = Channel::default()
        .tx(FreqConf::new(173.3).unwrap())
        .rx(FreqConf::with_group_sel(400.3, GroupSel::new_dcs(27, DcsSuffix::Normal)).unwrap());
    let command = channel.generate_command().unwrap();
    dbg!(command);
    let filter = FilterConfig::default().preemphasis(FilterState::Bypass);
    let command = filter.generate_command().unwrap();
    dbg!(command);
}
