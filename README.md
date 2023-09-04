# MIDI2

A helper library for dealing with midi 2 byte data.

For detailed midi2 specification see [the documenation](https://midi.org/)
on which this library is based.

## **Note!**

This library is still in its early development phase and is not
recommended for production.

We would welcome contributions! 
Please refer to the [CONTRIBUTOR.md](CONTRIBUTOR.md)

## Features
    
### Ergonomic, semantic wrappers for midi2 message types.
Wrappers are "views" onto the underlying data
allowing for optimisation by avoiding copies.

#### Example

```rust
use midi2::message::midi2_channel_voice::{
    NoteOnMessage,
    NoteAttribute,
};


let mut buffer = [0x0; 2];
let message = NoteOnMessage::builder(&mut buffer)
    .note(midi2::u7::new(0x60))
    .velocity(0x4B57)
    .attribute(NoteAttribute::ManufacturerSpecific(0x63FF))
    .build();

assert_eq!(message.unwrap().data(), &[0x4090_6001, 0x4B57_63FF]);
```

### Midi2 Capability Inquiry message wrappers
Wrappers around the special midi2 Capability Inquiry.
These messages are represented by groups of either midi1 or midi2 
system exclusive messages.

### `#![no_std]`
The library is entirely no_std, which guarantees that 
it will never allocate memory under the hood.
This makes it suitable for use on realtime audio threads.
