# snowflake-rust
snowflake is a lib for generating unique ID 

this project is rust-lang implementation of Twitter's id generator algorithm snowflake

Twitter 的id生成器算法snowflake的rust-lang 实现

[![Build Status](https://travis-ci.org/hanskorg/snowflake-rust.svg?branch=master)](https://travis-ci.org/hanskorg/snowflake-rust)

# Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
snowflake-multi-threaded = "0.1.0"
```

```rust
 let workerId:i64 = 1;
 let datacenterId:i64 = 1;
 let mut id_gen = SnowFlakeId::new(workerId,datacenterId);
 println!("{}", id_gen.generate().unwrap());
```
