# Ping

A toy implementation of ICMP and ICMP6 Echo request and response written while
learning Rust.

I mainly wanted to explore the following ideas:

- The memory layout of structs, specifically how to turn a struct into a byte
array and how to interpret a byte array as a struct with zero-copy. The article
[The Lost Art of Structure Packing](http://www.catb.org/esr/structure-packing/)
was really helpful in understanding the alignment requirements.

- Big endian and little endian representation of multi-byte values.

- Make the echo request sender and echo response handler asynchronous so
that multiple responses can be processed for a single request (e.g. when
sending a echo request to a multicast address).

- Communication between threads using channels instead of sharing memory.
