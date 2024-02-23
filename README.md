## Quake 3 Arena Server Log Parser ##

## NOTE ###

Not all decisions used here should be used always and everywhere, consider that they are only to demonstrate knowledge, with educational purposes.
Some implementations are expected to be considered "over-engineering" just to parse a log. In a real world, it could be much simpler.

### Features, libraries and functions used ###

* Cargo `profile.release` was configured for better performance.
* Crate `memmap` for memory mapped file, resulting in fast file reading.  
* Crate `mimalloc` allocator has excellent performance and reduces memory fragmentation.
* Function `from_str_unchecked` for fast String allocation.
* Crate `ahash` for fast HashMap implementation.
* Crate `rayon` to parallelize CPU-bound processing.
* Crate `once_cell` to allow static lazy load. 
* Crate `strum` to easy enum persistence.
* Crate `regex` using named captures to parse the log file. 
* Crate `clap` for command line argument parser.
* Function `mem::take` to fast-changing ownership.   
* Use of `static Mutex` to static mutate state.
* `serde_json` with feature `preserve_order`, default behavior sort the field name of the json object.

### Design ###

* Use of very decoupled structures to facilitate maintenance and testing.
* Parser is aware just of the type of the log event.
* Analyzer invokes the parser and updates the Match Accumulator.
* Match Accumulator is decoupled from Parser and Analyzer.
* Definition of new type pattern `Player` to enforce usage of this type, where methods receive arguments as `impl Into<Player>`.
* Parser is a trait, that could be used in different parser implementations, useful for testing and maintenance.
* Report is also a Trait, so it is easy to change the report output.

### Usage ###

Generate report for players ranking: 
```terminal
cargo run --
```

Generate report for kills by means: 
```terminal
cargo run -- --kbm
```
