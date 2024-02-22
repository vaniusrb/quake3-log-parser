todo:
- format report
    json / human


fast read and parser file:
https://github.com/mtb0x1/1brc

test url:
https://gist.github.com/cloudwalk-tests/704a555a0fe475ae0284ad9088e203f1


REMAINDER / NOTE:
Not all choices used here should be used always and anywhere, it's just to demonstrate knowledge.
Some implementations it's expected be considered a "over engineering" just to parse a log. 
In a real world it could be much simpler.

Example:
reduce file size
jemmaloc
    Avoid memory fragmentation (for long time of running process) and quick deallocation
memmap
from_str_unchecked
reduce/fold
    use of excessive functional sintaxe could result in a code with bad readability. 
    it's justified to use when is desired parallelism with Rayon
    in many cases the old and good for is better to understand and to maintain
in_line always
ahash
    generally the std Hash is enough
    with capacity
rayon
    when have cpu processing, for I/O should use Async with Tokio
once_cell / lazy load
strum

mem::take
static MATCH_COUNTER: Mutex<u32> = Mutex::new(0u32);


could be used different parser implementations

parser is decoupled from the summarize 
generic
mock parser
new type
    impl Into<Player>
    
