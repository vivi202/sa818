mod mocked_io;
use sa818::{
  self,
  channel::{Channel, FreqConf},
  group_call::{DcsSuffix, GroupSel},
};

#[test]
fn test_handshake() {
  //Handshake success
  let mut mock = mocked_io::Mock::new().response("+DMOCONNECT:0\r\n".to_string());
  let val = sa818::handshake(&mut mock);
  assert!(val.is_ok());
  //Handshake failure
  let mut mock = mocked_io::Mock::new().response("+DMOCONNECT:1\r\n".to_string());
  let val = sa818::handshake(&mut mock);
  assert!(val.is_err());
}

#[test]
fn write_channel_conf() {
  //Test default configuration
  let channel = Channel::default()
    .tx(FreqConf::new(433.925).unwrap())
    .rx(FreqConf::new(433.95).unwrap());
  let mut mock = mocked_io::Mock::new().response("+DMOSETGROUP=0\r\n".to_string());
  let response = channel.write_config(&mut mock);
  //Default configuration is NBFM and no group selective.
  assert_eq!(
    mock.input,
    "AT+DMOSETGROUP=1,433.9250,433.9500,0000,4,0000\r\n"
  );
  assert!(response.is_ok());

  //Test ctcss setting
  let channel = Channel::default()
    .tx(FreqConf::with_group_sel(433.925, GroupSel::new_ctcss(15)).unwrap())
    .rx(FreqConf::with_group_sel(433.95, GroupSel::new_ctcss(8)).unwrap());
  let mut mock = mocked_io::Mock::new().response("+DMOSETGROUP=0\r\n".to_string());
  let response = channel.write_config(&mut mock);
  assert_eq!(mock.input, "AT+DMOSETGROUP=1,433.9250,433.9500,15,4,8\r\n");
  assert!(response.is_ok());

  //Test dcs setting
  let channel = Channel::default()
    .tx(FreqConf::with_group_sel(433.925, GroupSel::new_dcs(26, DcsSuffix::Normal)).unwrap())
    .rx(FreqConf::with_group_sel(433.950, GroupSel::new_dcs(90, DcsSuffix::Inverted)).unwrap());
  let mut mock = mocked_io::Mock::new().response("+DMOSETGROUP=0\r\n".to_string());
  let response = channel.write_config(&mut mock);
  assert_eq!(
    mock.input,
    "AT+DMOSETGROUP=1,433.9250,433.9500,26N,4,90I\r\n"
  );
  assert!(response.is_ok());

  //failure
  let mut mock = mocked_io::Mock::new().response("+DMOSETGROUP=1\r\n".to_string());
  let response = channel.write_config(&mut mock);
  assert!(response.is_err())
}

#[test]
fn test_get_version() {
  //Test success
  let mut mock = mocked_io::Mock::new().response("+VERSION:SA818_V4.0\r\n".to_string());
  let version = sa818::get_version(&mut mock);
  assert!(version.is_ok());
  assert_eq!(version.unwrap(), "SA818_V4.0");
  //Test failure
  let mut mock = mocked_io::Mock::new().response("+INVALID:SA818_V4.0\r\n".to_string());
  let version = sa818::get_version(&mut mock);
  assert!(version.is_err());
}
#[test]
fn test_get_rssi() {
  //Test success
  let mut mock = mocked_io::Mock::new().response("RSSI=128\r\n".to_string());
  let rssi = sa818::get_rssi(&mut mock);
  assert_eq!(mock.input, "RSSI?\r\n");
  assert!(rssi.is_ok());
  assert_eq!(rssi.unwrap(), 128);

  //Test failure
  let mut mock = mocked_io::Mock::new().response("INVALID=128\r\n".to_string());
  let rssi = sa818::get_rssi(&mut mock);
  assert_eq!(mock.input, "RSSI?\r\n");
  assert!(rssi.is_err());

  //Test parsing failure
  let mut mock = mocked_io::Mock::new().response("RSSI=abc\r\n".to_string());
  let rssi = sa818::get_rssi(&mut mock);
  assert_eq!(mock.input, "RSSI?\r\n");
  assert!(rssi.is_err());
}
