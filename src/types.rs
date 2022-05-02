pub(crate) mod types {

    use std::collections::{HashMap};

    use std::io::{Error, ErrorKind, Read, Write};

    use bytestream::*;

    use crate::math::ipos3;

    use crate::constants::*;

    pub(crate)   struct MapData {
        data: HashMap<ipos3, Voxel>
    }

    pub struct Map {
        data: Vec<Vec<Vec<Voxel>>>
    }

    impl Map {
        pub fn set_voxel(&mut self, x:usize,y:usize,z:usize, voxel: Voxel) {
            assert!(x < HEIGHT as usize);
            assert!(y < WIDTH as usize);
            assert!(z < DEPTH as usize);
            self.data[x][y][z] = voxel;
        }
        pub fn get_voxel(&self, x:usize,y:usize,z:usize) -> Voxel {
            assert!(x < HEIGHT as usize);
            assert!(y < WIDTH as usize);
            assert!(z < DEPTH as usize);
            self.data[x][y][z]
        }
    }

    impl StreamReader for Map {
        fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> std::io::Result<Self> {
            let mut data = vec![vec![vec![Voxel { kind: VoxelType::Open, color: SKY_COLOR };DEPTH as usize];WIDTH as usize];HEIGHT as usize];
            let mut map = Map { data };

            for y in 0..WIDTH as usize {
                for x in 0..HEIGHT as usize {
                    for z in 0..DEPTH as usize {
                        map.set_voxel(x,y,z, Voxel{kind: VoxelType::Open, color: SKY_COLOR });
                    }

                    let mut it = Span::read_from(buffer, order)?;

                    while !it.is_last_span() {
                        let mut colors = &it.color_array[..];

                        for i in it.start_top_coloured .. (it.end_top_coloured + 1) {
                            let color : BGRAColor;

                            if it.color_array.len() == 1 {
                                color = it.color_array[0];
                            } else {
                                color = *(colors.take_first().unwrap_or(&DEFAULT_COLOR));
                            }

                            map.set_voxel(x,y,i as usize,Voxel { kind: VoxelType::Solid, color : color.clone()});
                        }

                        it = Span::read_from(buffer, order)?;
                    }

                    for i in it.start_top_coloured .. DEPTH as u8 {
                        map.set_voxel(x,y,i as usize,Voxel { kind: VoxelType::Solid, color : DEFAULT_COLOR });
                    }
                }
            }

            Ok(map)
        }
    }

    impl StreamReader for MapData {
        fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> std::io::Result<Self> {
            let mut data:  HashMap<ipos3, Voxel> = HashMap::new();
            for y in 0..WIDTH {
                for x in 0..HEIGHT {
                    let mut it = Span::read_from(buffer,order)?;

                    for z in 0..DEPTH {
                        let pos = ipos3 {
                            x: x as u8,
                            y: y as u8,
                            z: z as u8
                        };
                        data.entry(pos).or_insert(Voxel{ kind: VoxelType::Open, color: SKY_COLOR });
                    }

                    while !it.is_last_span() {
                        let mut slice = &it.color_array[..];

                        for i in it.start_top_coloured .. (it.end_top_coloured + 1) {
                            let pos = ipos3 {
                                x: x as u8,
                                y: y as u8,
                                z: i
                            };
                            let color : BGRAColor;
                            if it.color_array.len() == 1 {
                                color = it.color_array[0];
                            } else {
                                color = *(slice.take_first().unwrap_or(&DEFAULT_COLOR));
                            }

                            let k = data.entry(pos).or_insert(Voxel { kind: VoxelType::Solid, color : color.clone()});
                            *k = Voxel { kind: VoxelType::Solid, color : color.clone()};
                        }
                        it = Span::read_from(buffer,order)?;
                    }

                    for i in it.start_top_coloured .. DEPTH as u8 {
                        let pos = ipos3 {
                            x: x as u8,
                            y: y as u8,
                            z: i
                        } ;

                        let k = data.entry(pos).or_insert(Voxel { kind: VoxelType::Solid, color : DEFAULT_COLOR });
                        *k = Voxel { kind: VoxelType::Solid, color : DEFAULT_COLOR };
                    }
                }
            }

            Ok(Self { data })
        }
    }

    #[repr(C)]
    struct Span {
        num_4b_chunks: u8,
        start_top_coloured: u8,
        end_top_coloured: u8,
        start_air: u8,
        color_array: Vec<BGRAColor>
    }

    impl Span {
        #[inline(always)]
        fn is_last_span(&self) -> bool {
            return self.num_4b_chunks == END_OF_RUN as u8;
        }
    }

    impl StreamReader for Span {
        fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> std::io::Result<Self> {
            let num_4b_chunks = u8::read_from(buffer,order)?;
            let start_top_coloured = u8::read_from(buffer,order)?;
            let end_top_coloured = u8::read_from(buffer,order)?;
            let start_air =  u8::read_from(buffer,order)?;

            if start_top_coloured > DEPTH as u8 {
                return Err(Error::new(ErrorKind::InvalidData, format!("Unreachable voxel. y={}", start_top_coloured)));
            }

            let mut colors = vec![];

            // Case: Not last section | num_4b_chunks != 0
            if num_4b_chunks != END_OF_RUN as u8 {
                for _ in 0..(num_4b_chunks - 1) {
                    colors.push(BGRAColor::read_from(buffer,order)?)
                }
            } else {
                if start_top_coloured == end_top_coloured {
                    colors.push(BGRAColor::read_from(buffer,order)?)
                } else {
                    for _ in start_top_coloured..(end_top_coloured + 1) {
                        colors.push(BGRAColor::read_from(buffer,order)?)
                    }
                }
            }

            Ok(Self{
                num_4b_chunks,
                start_top_coloured,
                end_top_coloured,
                start_air,
                color_array: colors
            })
        }
    }

    impl StreamWriter for Span {
        fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> std::io::Result<()> {
            self.num_4b_chunks.write_to(buffer,order)?;
            self.start_top_coloured.write_to(buffer,order)?;
            self.end_top_coloured.write_to(buffer,order)?;
            self.start_air.write_to(buffer,order)?;

            for i in &self.color_array {
                i.write_to(buffer, order)?;
            }

            Ok(())
        }
    }

    #[derive(Copy, Clone)]
    struct Voxel {
        kind: VoxelType,
        color: BGRAColor
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct BGRAColor {
        pub(crate) b: u8,
        pub(crate) g: u8,
        pub(crate) r: u8,
        pub(crate) a: u8
    }

    impl StreamReader for BGRAColor {
        fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> std::io::Result<Self> {
            Ok(Self{
                b: u8::read_from(buffer,order)?,
                g: u8::read_from(buffer,order)?,
                r: u8::read_from(buffer,order)?,
                a: u8::read_from(buffer,order)?
            })
        }
    }

    impl StreamWriter for BGRAColor {
        fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> std::io::Result<()> {
            self.b.write_to(buffer,order)?;
            self.g.write_to(buffer,order)?;
            self.r.write_to(buffer,order)?;
            self.a.write_to(buffer,order)?;
            Ok(())
        }
    }

    #[derive(Copy, Clone)]
    enum VoxelType {
        Open,
        Solid
    }

}

