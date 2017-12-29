extern crate byteorder;

use std::io::Cursor;
use self::byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub struct DatabaseHeader {
    header_str: String,
    page_size: u16,
    write_version: u8,
    read_version: u8,
    reserved_space: u8,
    max_ambedded_payload_fraction: u8,
    min_embedded_payload_fraction: u8,
    leaf_payload_fraction: u8,
    change_counter: u32,
    in_header_database_size: u32,
    first_freelist_trunk_page: u32,
    num_freelist_pages: u32,
    schema_cookie: u32,
    schema_format_number: u32,
    default_page_cache_size: u32,
    largest_root_btree_page: u32,
    text_encoding: u32,
    user_version: u32,
    incremental_vacuum_mode: u32,
    application_id: u32,
    version_valid_for: u32,
    sqlite_version_number: u32,
}

impl DatabaseHeader {
    pub fn parse(buf: &[u8]) -> Option<DatabaseHeader> {
        if buf.len() != 100 { return None; }

        let header_str = String::from_utf8(Vec::from(&buf[0..16])).unwrap();

        let mut rdr = Cursor::new(Vec::from(&buf[16..]));
        let page_size = rdr.read_u16::<BigEndian>().unwrap();
        let write_version = rdr.read_u8().unwrap();
        let read_version = rdr.read_u8().unwrap();
        let reserved_space = rdr.read_u8().unwrap();
        let max_ambedded_payload_fraction = rdr.read_u8().unwrap();
        let min_embedded_payload_fraction = rdr.read_u8().unwrap();
        let leaf_payload_fraction = rdr.read_u8().unwrap();
        let change_counter = rdr.read_u32::<BigEndian>().unwrap();
        let in_header_database_size = rdr.read_u32::<BigEndian>().unwrap();
        let first_freelist_trunk_page = rdr.read_u32::<BigEndian>().unwrap();
        let num_freelist_pages = rdr.read_u32::<BigEndian>().unwrap();
        let schema_cookie = rdr.read_u32::<BigEndian>().unwrap();
        let schema_format_number = rdr.read_u32::<BigEndian>().unwrap();
        let default_page_cache_size = rdr.read_u32::<BigEndian>().unwrap();
        let largest_root_btree_page = rdr.read_u32::<BigEndian>().unwrap();
        let text_encoding = rdr.read_u32::<BigEndian>().unwrap();
        let user_version = rdr.read_u32::<BigEndian>().unwrap();
        let incremental_vacuum_mode = rdr.read_u32::<BigEndian>().unwrap();
        let application_id = rdr.read_u32::<BigEndian>().unwrap();
        let new_pos = rdr.position() + 20;
        rdr.set_position(new_pos);
        let version_valid_for = rdr.read_u32::<BigEndian>().unwrap();
        let sqlite_version_number = rdr.read_u32::<BigEndian>().unwrap();

        let result = DatabaseHeader {
            header_str,
            page_size,
            write_version,
            read_version,
            reserved_space,
            max_ambedded_payload_fraction,
            min_embedded_payload_fraction,
            leaf_payload_fraction,
            change_counter,
            in_header_database_size,
            first_freelist_trunk_page,
            num_freelist_pages,
            schema_cookie,
            schema_format_number,
            default_page_cache_size,
            largest_root_btree_page,
            text_encoding,
            user_version,
            incremental_vacuum_mode,
            application_id,
            version_valid_for,
            sqlite_version_number,
        };

        Some(result)
    }
}

#[test]
fn test_database_header() {
    let buf: [u8; 100] = [
        0x53, 0x51, 0x4c, 0x69,
        0x74, 0x65, 0x20, 0x66,
        0x6f, 0x72, 0x6d, 0x61,
        0x74, 0x20, 0x33, 0x00, // SQLite format 3.
        0x10, 0x00,             // DB Page Size
        0x01,                   // File format write version
        0x01,                   // File format read version
        0x00,                   // Bytres of unused "reserved" space at the end of each page
        0x40,                   // Maximum embedded payload fraction
        0x20,                   // Minimum embedded payload fraction
        0x20,                   // Leaf payload fraction
        0x00, 0x00, 0x00, 0x02, // File change counter
        0x00, 0x00, 0x00, 0x02, // Size of database file in pages (The "in-header" database size)
        0x00, 0x00, 0x00, 0x00, // Page number of the first freelist trunk page
        0x00, 0x00, 0x00, 0x00, // Total number of freelist pages
        0x00, 0x00, 0x00, 0x01, // The schema cookie
        0x00, 0x00, 0x00, 0x04, // The schema format number
        0x00, 0x00, 0x00, 0x00, // Default page cache size
        0x00, 0x00, 0x00, 0x00, // Page number of the larget root b-tree page
        0x00, 0x00, 0x00, 0x01, // Database text encoding
        0x00, 0x00, 0x00, 0x00, // The "user version"
        0x00, 0x00, 0x00, 0x00, // Incremental vacuum mode indicator
        0x00, 0x00, 0x00, 0x00, // Application ID
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, // Reserved
        0x00, 0x00, 0x00, 0x00, // Version-valid-for
        0x00, 0x2e, 0x10, 0xfb  // Sqlite version number
    ];

    assert_eq!(100, buf.len());

    let header = DatabaseHeader::parse(&buf).unwrap();

    assert_eq!("SQLite format 3\u{0}", header.header_str);
    assert_eq!(4096, header.page_size);
    assert_eq!(1, header.write_version);
    assert_eq!(1, header.read_version);
    assert_eq!(0, header.reserved_space);
    assert_eq!(64, header.max_ambedded_payload_fraction);
    assert_eq!(32, header.min_embedded_payload_fraction);
    assert_eq!(32, header.leaf_payload_fraction);
    assert_eq!(2, header.change_counter);
    assert_eq!(2, header.in_header_database_size);
    assert_eq!(0, header.first_freelist_trunk_page);
    assert_eq!(0, header.num_freelist_pages);
    assert_eq!(1, header.schema_cookie);
    assert_eq!(4, header.schema_format_number);
    assert_eq!(0, header.default_page_cache_size);
    assert_eq!(0, header.largest_root_btree_page);
    assert_eq!(1, header.text_encoding);
    assert_eq!(0, header.user_version);
    assert_eq!(0, header.incremental_vacuum_mode);
    assert_eq!(0, header.application_id);
    assert_eq!(0, header.version_valid_for);
    assert_eq!(3019003, header.sqlite_version_number);
    ;
}