#![feature(slice_take)]

mod constants;
mod types;
mod math;




#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::io::{Cursor, Error, Read};
    use bytestream::ByteOrder::LittleEndian;
    use bytestream::StreamReader;
    use crate::types::types::Map;



    #[test]
    fn read_vxl() -> Result<(), Error>{
        let mut file = File::open("vxl/square.vxl")?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let mut cur =  Cursor::new(buffer);

        let m = Map::read_from(&mut cur, LittleEndian)?;
        let v = m.get_voxel(0,0,0);
        print!("#{:X}{:X}{:X} | {:?}",v.color.r, v.color.g, v.color.b, v.kind);
        Ok(())
    }
}
