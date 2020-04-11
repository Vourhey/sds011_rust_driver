use serialport::{SerialPort, SerialPortSettings, DataBits, FlowControl, Parity, StopBits};
use std::time::Duration;
use std::mem::transmute;


const HEAD: u8 = b'\xaa';
const TAIL: u8 = b'\xab';
const CMD_ID: u8 = b'\xb4';

const READ: u8 = b'\x00';
const WRITE: u8 = b'\x01';

const REPORT_MODE_CMD: u8 = b'\x02';
const ACTIVE: u8 = b'\x00';
const PASSIVE: u8 = b'\x01';

const QUERY_CMD: u8 = b'\x04';

// The sleep command ID
// TODO
//const SLEEP_CMD: u8 = b'\x06';
// Sleep and work byte
// TODO
// const SLEEP: u8 = b'\x00';
// const WORK: u8= b'\x01';

// The work period command ID
const WORK_PERIOD_CMD: u8 = b'\x08';


pub(crate) struct SDS011 {
    port: Box<dyn SerialPort>,
}

impl SDS011 {

    pub fn new(port: &str) -> SDS011 {
        let s = SerialPortSettings {
            baud_rate: 9600,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_secs(2),
        };

        let opened = serialport::open_with_settings(port, &s).unwrap();
        let mut s = SDS011 { port: opened };
        s.set_report_mode();
        s
    }

    pub fn set_report_mode(&mut self) {
        let read = false;
        let active = false;

        let mut cmd = self.cmd_begin();

        cmd.push(REPORT_MODE_CMD);
        cmd.push(if read { READ } else { WRITE });
        cmd.push(if active { ACTIVE } else { PASSIVE });
        cmd.append(vec![b'\x00'; 10].as_mut());

        println!("Before finish {:?}", cmd);
        self.finish_cmd(&mut cmd);
        println!("After finish {:?}", cmd);
        self.execute(&cmd);
        self.get_reply();
    }

    pub fn query(&mut self) -> Option<(f32, f32)> {
        let mut cmd = self.cmd_begin();

        cmd.push(QUERY_CMD);
        cmd.append(vec![b'\x00'; 12].as_mut());

        self.finish_cmd(&mut cmd);
        self.execute(&cmd);

        match self.get_reply() {
            None => return None,
            Some(raw) =>  {
                let pm25_ar = [raw[2], raw[3]];
                let pm10_ar = [raw[4], raw[5]];
                let pm25: u16 = unsafe{ transmute::<[u8; 2], u16>(pm25_ar ) }.to_le();
                let pm10: u16 = unsafe{ transmute::<[u8; 2], u16>(pm10_ar) }.to_le();

                return Some((pm25 as f32 / 10.0, pm10 as f32 / 10.0));
            }
        }
    }

    pub fn cmd_begin(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.push(HEAD);
        vec.push(CMD_ID);
        vec
    }

    pub fn set_work_period(&mut self, work_time: u8) {
        let read = false;
        let mut cmd = self.cmd_begin();

        cmd.push(WORK_PERIOD_CMD);
        cmd.push(if read { READ } else { WRITE });
        cmd.push(work_time);
        cmd.append(vec![b'\x00'; 10].as_mut());

        self.finish_cmd(&mut cmd);
        self.execute(&cmd);
        self.get_reply();
    }

    fn finish_cmd(&self, cmd: &mut Vec<u8>)  {
        let id1=b'\xff';
        let id2=b'\xff';

        cmd.push(id1);
        cmd.push(id2);

        let mut ch = cmd.clone();
        ch.truncate(cmd.len()-2);
        let mut checksum: u32 = 0;
        for i in ch {
            checksum += i as u32;
        }
        checksum = checksum % 256;

        cmd.push(checksum as u8);
        cmd.push( TAIL);
    }

    fn execute(&mut self, cmd_bytes: &Vec<u8>) {
        self.port.write_all(cmd_bytes).expect("Couldn't write");
    }

    fn get_reply(&mut self) -> Option<[u8; 10]> {
        let mut buf = [0u8; 10];
        let res = self.port.read_exact(buf.as_mut());

        println!("{:?}", String::from_utf8_lossy(&buf));

        match res {
            Err(e) =>println!("{:?}", e),
            Ok(v) => println!("{:?}", String::from_utf8_lossy(&buf)),
        }


        let data = &buf[2..8];

        if data.len() == 0 { return None; }

        let check: u8 = data.iter().sum::<u8>() & 255;
        if check != buf[8] { return None; }

        Some(buf)
    }
}
