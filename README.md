# commonregex-rs

🔎 Find common expressions in a string

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)
  - [Internet](#internet)
  - [IP Addresses](#ip-addresses)
  - [Phone numbers](#phone-numbers)
- [Documentation](#documentation)
- [Notes](#notes)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Installation

Add this to your `Cargo.toml`

```toml
[dependencies]
commonregex_rs = "0.1.0"
```

## Usage

```rust
extern crate commonregex_rs;

use commonregex_rs::commonregex;

let logs = String::from("
     Started GET '/' for 127.0.0.1 at 2019-05-11 00:51:35
     Started GET '/' for 10.10.0.1 at 2019-05-11 00:52:05
");
 
let filtered_ips = commonregex::ip::v4(&logs);
// #=> vec!["127.0.0.1", "10.10.0.1"]
```

## Features

### Internet

**Email**

```rust
commonregex::internet::email(&text)
```

**URL**

```rust
commonregex::internet::url(&text)
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

**US phone numbers**

```rust
commonregex::phone::us(&text)
```

## Documentation

Full documentation is available [here](https://docs.rs/commonregex_rs/0.1.0/commonregex_rs/).

## Contributing

Please read [CONTRIBUTING.md](https://github.com/fakenine/commonregex-rs/blob/master/CONTRIBUTING.md) for information on how to contribute.

## Notes

This project was inspired by [commonregex](https://github.com/mingrammer/commonregex).
