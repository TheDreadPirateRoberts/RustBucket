pub fn read(buf: &[u8]) -> Result<(u64, usize), &'static str>{
    let check_bit: u8 = 0b1000_0000;

    let mut value: u64 = 0;
    let mut bytes = 0;
    for (i, n) in buf.iter().enumerate() {
        if n & check_bit == 128{
            if i == 8 {println!("HERE!!!");return Err("Ninth byte of VLI must have bit 8 unset");}
            let m = n - 128;
            value = value << 7;
            value += m as u64;
        } else{
            value = value << 7;
            value += *n as u64;
            bytes = i + 1;
            break;
        }
    }
    Ok((value, bytes))
}


#[test]
fn test_single_byte_1() {
    assert_eq!(read(&[0x00]).unwrap().0, 0x00);
    assert_eq!(read(&[0x00]).unwrap().1, 1);
}

#[test]
fn test_single_byte_2() {
    assert_eq!(read(&[0x7f]).unwrap().0, 0x7f);
    assert_eq!(read(&[0x7f]).unwrap().1, 1);
}

#[test]
fn test_double_byte_1() {
    assert_eq!(read(&[0x81, 0x00]).unwrap().0, 0x80);
    assert_eq!(read(&[0x81, 0x00]).unwrap().1, 2);
}

#[test]
fn test_double_byte_2() {
    assert_eq!(read(&[0x82, 0x00]).unwrap().0, 0x100);
    assert_eq!(read(&[0x82, 0x00]).unwrap().1, 2);
}

#[test]
fn test_double_byte_3() {
    assert_eq!(read(&[0x80, 0x7f]).unwrap().0, 0x7f);
    assert_eq!(read(&[0x80, 0x7f]).unwrap().1, 2);
}

#[test]
fn test_five_byte_1() {
    assert_eq!(read(&[0x8a, 0x91, 0xd1, 0xac, 0x78]).unwrap().0, 0xa2345678);
    assert_eq!(read(&[0x8a, 0x91, 0xd1, 0xac, 0x78]).unwrap().1, 5);
}

#[test]
fn test_five_byte_2() {
    assert_eq!(read(&[0x81, 0x81, 0x81, 0x81, 0x01]).unwrap().0, 0x10204081);
    assert_eq!(read(&[0x81, 0x81, 0x81, 0x81, 0x01]).unwrap().1, 5);
}

#[test]
fn test_byte_nine_issue() {
    assert_eq!(read(&[0x81, 0x81, 0x81, 0x81, 0x81,
        0x81, 0x81, 0x81, 0xff]).err(), Some("Ninth byte of VLI must have bit 8 unset"));
}