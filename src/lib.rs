use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use chrono::Utc;

pub struct SnowflakeRust {
    epoch: i64,
    worker_id: i64,
    sequence: i64,
    time: Arc<Mutex<i64>>,
    sequence_mask: i64
}

impl SnowflakeRust {
    pub fn kubernetes() -> SnowflakeRust {
        let ip = get_ip().unwrap();
        let ip_split: Vec<&str> = ip.split(".").collect();
        let ip_low = ip_split[2].to_string().parse::<i64>().unwrap() << 8 | ip_split[3].to_string().parse::<i64>().unwrap();
        SnowflakeRust {
            epoch: 1575129600000,
            worker_id: ip_low,
            sequence: 0,
            time: Arc::new(Mutex::new(0)),
            sequence_mask: -1 ^ -1 << 12,
        }
    }

    pub fn new(worker_id: i64) -> SnowflakeRust {
        SnowflakeRust {
            epoch: 1575129600000,
            worker_id,
            sequence: 0,
            time: Arc::new(Mutex::new(0)),
            sequence_mask: -1 ^ (-1 << 12),
        }
    }

    pub fn generate(&mut self) -> Option<i64> {
        let mut last_timestamp = self.time.lock().unwrap();
        let mut timestamp = self.get_time();
        if timestamp < *last_timestamp {
            if *last_timestamp - timestamp > 150 {
                return None
            } else {
                thread::sleep(time::Duration::from_millis((*last_timestamp - timestamp + 1) as u64));
                timestamp = self.get_time();
            }
        } else if timestamp == *last_timestamp {
            self.sequence = (self.sequence + 1) & self.sequence_mask
        } else {
            self.sequence = 0
        }

        *last_timestamp = timestamp;
        println!("{}",timestamp);
        println!("{}",self.worker_id);
        println!("{}",self.sequence);
        Option::from((timestamp << 28) | (self.worker_id << 12) | self.sequence)
    }

    fn get_time(&self) -> i64 {
        Utc::now().timestamp_millis() - self.epoch
    }
}

fn get_ip() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip().to_string()),
        Err(_) => return None,
    };
}
