mod btree;
use btree::database_header::DatabaseHeader;

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("sqlite_template").unwrap();
    let mut buffer: [u8; 100] = [0;100];

    f.read(&mut buffer).unwrap();
    let header = DatabaseHeader::parse(&buffer).unwrap();

    println!("{:?}", header);
}
