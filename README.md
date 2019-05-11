# commonregex-rs

🔎 Find common expressions in a string

> Inspired by [commonregex](https://github.com/mingrammer/commonregex)

## Usage

```rust
extern crate commonregex_rs;

use commonregex_rs::commonregex;

let log = String::from("
     Started GET '/' for 127.0.0.1 at 2019-05-11 00:51:35
     Started GET '/' for 10.10.0.1 at 2019-05-11 00:52:05
");
 
let filtered_ips = commonregex::ip::v4(&log);
// #=> vec!["127.0.0.1", "10.10.0.1"]
```

## Features

### Internet

**Email**

```rust
commonregex::internet::email(&text)
```

### IP Addresses

**IPv4**

```rust
commonregex::ip::v4(&text)
```

**IPv6**

```rust
commonregex::ip::v6(&text)
```

### Phone numbers

**French phone numbers**

```rust
commonregex::phone::fr(&text)
```
