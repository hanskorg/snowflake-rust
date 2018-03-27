# snowflake-rust
snowflake is a lib for generating unique ID 
this project is rust-lang implementation of Twitter's id generator algorithm snowflake


#Usage
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
