## Parser to Quake 3 Arena server log ##

test url:
https://gist.github.com/cloudwalk-tests/704a555a0fe475ae0284ad9088e203f1

## NOTE ###
Not all choices used here should be used always and anywhere, it's just to demonstrate knowledge, intended to educational propose.
Some implementations it's expected be considered a "over engineering" just to parse a log. 
In a real world it could be much simpler.

### Features, libraries and functions used ###
* Cargo `profile.release` was defined to high performance.
* Crate `memmap` for memory mapped file, result in high performance file reading.  
* Crate `mimalloc` allocator, that has with excellent performance and reduces fragmentation.
* Function `from_str_unchecked` for fast String allocation.
* Crate `ahash` for fast HashMap implementation.
* Crate `rayon` to parallelize cpu bound processing.
* Crate `once_cell` to allow static lazy load. 
* Crate `strum` to easy enum persistence.
* Crate `regex` using named captures to parse the log file. 
* Function `mem::take` to fast changing ownership.   
* Use of `static Mutex` to static mutate state.
* `serde_json` with feature `preserve_order`, default behavior sort the field name of the json object.

### Design ###

* Use of very decoupled structs to facilitate maintenance and testing.
* Parser is aware just of the type of the event.
* Analyzer invoke the parser and update the Match Accumulator.
* Match Accumulator is decoupled from Parser and Analyzer.
* Definition of new type pattern `Player` to enforce usage of this type, where methods receive arguments as `impl Into<Player>`.
* Parser is trait, where could be used different parser implementations, useful to test and maintenance.
* Report is also a Trait, so is easy to change report output.
