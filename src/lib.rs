#![feature(iterator_try_collect)]
mod data;
#[cfg(test)]

mod lib {
    use std::fs;
    use std::fs::File;
    use std::io::{Cursor, Error, Read};
    use std::iter::Map;
    use bytestream::ByteOrder::LittleEndian;
    use bytestream::StreamReader;
    use crate::data::data::VXL;

    #[test]
    fn read_vxl() -> Result<(), Error>{

        let dir = fs::read_dir("vxl")?;
        for file in dir {
            let entry = file?;
            let os_name = entry.file_name();
            let name = os_name.to_str().unwrap();
            if name.ends_with("vxl") {
                println!("Reading [{}]",name);
                read_file(format!("vxl/{}",name))?;
                println!("Read [{}] successfully", name);
            }
        }
        Ok(())
    }

    fn read_file(path: String) -> Result<(), Error> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let mut cur =  Cursor::new(buffer);
        let map_data = VXL::read_from(&mut cur, LittleEndian)?;

        Ok(())
    }

    fn read_map_data(buffer: Vec<u8>) -> Result<VXL, Error> {
        let cursor = &mut Cursor::new(buffer);
        VXL::read_from(cursor,  LittleEndian)
    }
}