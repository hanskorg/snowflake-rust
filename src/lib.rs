// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//!
//!Generate id with the  type i64
//!
//!
//!
//! # Examples
//!
//! ```
//!use snowflake_multi_threaded::SnowFlakeId;
//!
//! let workerId:i64 = 1;
//! let datacenterId:i64 = 1;
//! let mut id_gen = SnowFlakeId::new(workerId,datacenterId);
//! assert!(id_gen.generate_id().is_ok());
//! ```
//!
//!

use std::sync::{Arc, Mutex};
use std::time::{SystemTime,SystemTimeError,UNIX_EPOCH};

#[derive(Default, Debug)]
pub struct SnowFlakeId{

    twepoch: i64,

    worker_id_bits: u8,
    datacenter_id_bits: u8,
    sequence_bits: u8,

    worker_id: i64,
    datacenter_id: i64,
    sequence: i64,

    worker_id_shift: u8,
    datacenter_id_shift: u8,
    timestamp_left_shift: u8,
    sequence_mask: i64,

    last_timestamp:Arc<Mutex<i64>>

}

impl SnowFlakeId{

    /// Returns the instance of SnowFlakeId
    ///
    /// # Examples
    ///
    /// ```
    /// use snowflake_multi_threaded::SnowFlakeId;
    ///
    /// let var = SnowFlakeId::new(1,1);
    ///
    /// ```
    pub fn new(workerId:i64, datacenterId:i64)->SnowFlakeId{
        let instance: SnowFlakeId = Default::default();
        SnowFlakeId{
            twepoch:1514736000_000i64,

            worker_id_bits:5,
            datacenter_id_bits:5,
            sequence_bits :12,

            worker_id      : workerId,
            datacenter_id : datacenterId,
            sequence       : 0i64,

            worker_id_shift : 12,
            datacenter_id_shift: 17,
            timestamp_left_shift: 22,

            sequence_mask : -1i64 ^ (-1i64 << 12),

            last_timestamp:Arc::new(Mutex::new(0)),
        }
    }

    /// Return a number type of i64, it will always grow
    /// `When Clock  moved backwards ,it will be panic`
    /// # Examples
    ///
    /// ```
    /// use snowflake_multi_threaded::SnowFlakeId;
    ///
    /// let var = SnowFlakeId::new(1,1).generate_id();
    ///
    /// println!("the new id is {}", var.unwrap());
    /// ```
    pub fn generate_id(&mut self) -> Result<i64,SystemTimeError> {
        let mut last_timestamp = self.last_timestamp.lock().unwrap();
        let mut timestamp = SnowFlakeId::curr_time()?;
        if timestamp < *last_timestamp {
            panic!("Clock moved backwards.  Refusing to generate id for {} milliseconds", *last_timestamp);
        }
        if timestamp == *last_timestamp {
            self.sequence = (self.sequence + 1) & self.sequence_mask;
            if self.sequence == 0 {
                //milliseconds overflow
                if timestamp == *last_timestamp {
                    timestamp = self.til_next_millis(*last_timestamp)?;
                }
            }
        } else {
            self.sequence = 0i64;
        }
//        println!("{}-<<<{}-{}-{}-{}-{}", self.sequence_mask, self.timestamp_left_shift, (*last_timestamp), self.datacenter_id, self.worker_id, self.sequence);
        *last_timestamp = timestamp;
        Ok(((timestamp - self.twepoch) << self.timestamp_left_shift)
                | (self.datacenter_id << self.datacenter_id_shift)
                | (self.worker_id << self.worker_id_shift)
                | self.sequence)
    }

    fn til_next_millis(&self, last_timestamp:i64) -> Result<i64,SystemTimeError>{
        let mut timestamp = SnowFlakeId::curr_time()?;
        while timestamp <= last_timestamp {
            timestamp = SnowFlakeId::curr_time()?
        }
        Ok(timestamp)
    }

    fn curr_time() -> Result<i64, SystemTimeError>{
        let elapsed    = SystemTime::now().duration_since(UNIX_EPOCH)? ;
        let unix_timestamp = elapsed.as_secs();
        Ok((unix_timestamp * 1_000 | (elapsed.subsec_nanos() / 1_000_000) as u64) as i64)
    }
}

#[cfg(test)]
mod test {
    use SnowFlakeId;
    use std::sync::{Arc};
    use std::thread;
    use std::time::Duration;
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::time::Instant;

    #[test]
    fn loop_test(){
        let mut id_gen = SnowFlakeId::new(1,1);
        println!("{:?}",&id_gen);
        for _ in 1..1000 {
            let t  = &mut id_gen;
            assert!(t.generate_id().is_ok());
        }
    }
    #[test]
    fn multi_thread(){

        let now = Instant::now();
        for i in 1 .. 10{
            let mut id_gen = SnowFlakeId::new(i,1);
            thread::spawn(move || {
                for _ in 1..1000 {
                    let t  = &mut id_gen;
                    let new_id = t.generate_id().unwrap();
                    let id = t.generate_id();
                    assert!(id.is_ok());
//                    println!("{:?}",id.unwrap());
                }
            });
        }
        let elapsed = now.elapsed();
        println!("{}.{}",elapsed.as_secs(),elapsed.subsec_nanos());
    }
}