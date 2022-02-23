mod align;
pub mod types;

#[macro_export]
macro_rules! osc_format {
    ($addr:expr, $( $x:expr ),* ) => {
        {
            use types::*;
            use align::align_up;

            use std::io::Write;

            debug_assert!($addr.len() > 0, "Empty address!");
            debug_assert!($addr.chars().nth(0) == Some('/'), "Path has to string with '/'");

            let addr_block_size = align_up($addr.len() + 1, 4);
            let tag_block_size = align_up( $( $x.tag_width()+ )* 1, 4);
            let arg_block_size = align_up( $( $x.width()+ )* 0, 4);

            let size = addr_block_size + tag_block_size + arg_block_size;

            let mut buf = vec![0u8; size];

            (&mut buf[..]).write_all($addr.as_bytes()).unwrap();

            let mut offset = addr_block_size;

            buf[offset] = ',' as u8;
            offset += 1;

            $(
                buf[offset] = $x.tag();
                offset += 1;
            )*

            debug_assert!(align_up(offset, 4) == addr_block_size + tag_block_size);
            let mut offset = addr_block_size + tag_block_size;

            $(
                $x.encode(&mut buf[offset..]);
                offset += $x.width();
            )*

            debug_assert!(align_up(offset, 4) == size);

            buf
        }
    };
}

#[test]
fn osc_freq() {
    let packet = osc_format!("/oscillator/4/frequency", 440f32);
    let raw: Vec<u8> = vec![
        0x2f, 0x6f, 0x73, 0x63, 0x69, 0x6c, 0x6c, 0x61, 0x74, 0x6f, 0x72, 0x2f, 0x34, 0x2f, 0x66,
        0x72, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x79, 0x00, 0x2c, 0x66, 0x00, 0x00, 0x43, 0xdc,
        0x00, 0x00,
    ];
    assert_eq!(packet, raw);
}

#[test]
fn osc_foo() {
    let packet = osc_format!("/foo", 1000, -1, "hello", 1.234, 5.678);
    let raw: Vec<u8> = vec![
        0x2f, 0x66, 0x6f, 0x6f, 0x00, 0x00, 0x00, 0x00, 0x2c, 0x69, 0x69, 0x73, 0x66, 0x66, 0x00,
        0x00, 0x00, 0x00, 0x03, 0xe8, 0xff, 0xff, 0xff, 0xff, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x00,
        0x00, 0x00, 0x3f, 0x9d, 0xf3, 0xb6, 0x40, 0xb5, 0xb2, 0x2d,
    ];
    assert_eq!(packet, raw);
}

#[test]
fn ytterbium() {
    let packet = osc_format!("/OSCILLATORS/OSC2/ADSR/z", 0.0, 0.0, 0.0, 0.0);
    let raw: Vec<u8> = vec![
        47, 79, 83, 67, 73, 76, 76, 65, 84, 79, 82, 83, 47, 79, 83, 67, 50, 47, 65, 68, 83, 82, 47,
        122, 0, 0, 0, 0, 44, 102, 102, 102, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    assert_eq!(packet, raw);
}
