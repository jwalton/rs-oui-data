# oui-data

A Rust library for looking up information from the IEEE OUI database.

## Usage

Use the `lookup` function to lookup a MAC address in the MA-L, MA-M, MA-S, CID, and IAB registries.

```rs
let record = oui::lookup("00:00:00:00:00:00").unwrap();
assert_eq!(record.organization(), "XEROX CORPORATION");
```

## Building

This library is largely generated code, derived from data hosted at [http://standards-oui.ieee.org](http://standards-oui.ieee.org).

To build with updated records, run `./update_data.sh` to update the CSV files, then just `cargo build` as usual.
