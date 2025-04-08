# 🦀 crabics-parser (fun implementation of Rust crates)

A Rust binary crate that defines and parses a custom binary file format called `.crb` (Crabics Binary).  
The `.crb` format stores information about liquid pours into a virtual bucket — name, amount, and timestamp.

---

## File Format Specification (`.crb`)

Each `.crb` file consists of one or more **pour records**, each with the following layout (in bytes):

| Offset | Field        | Size (bytes) | Description                        |
|--------|--------------|--------------|------------------------------------|
| 0      | Name Length  | 1            | Length of the name string (max 255)|
| 1      | Name         | N            | UTF-8 encoded string (N = Name Len)|
| N+1    | Amount       | 4            | `f32` representing liters poured   |
| N+5    | Timestamp    | 8            | UNIX timestamp as `u64`            |

Each record is packed tightly, and multiple records are stored consecutively.

---

## Features

- Define and serialize `.crb` files from a vector of `PourRecord`
- Parse `.crb` binary files safely and efficiently
- Validate format and handle corrupted files gracefully
- CLI to view or dump file contents

---

## Usage

### CLI Example

```sh
cargo run -- parse samples/test.crb
