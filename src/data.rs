mod data {
    use std::collections::{LinkedList, VecDeque};
    use std::io::Read;
    use bytestream::{ByteOrder, StreamReader};

    struct VXL {
        cols: Vec<Vec<Column>>
    }

    impl StreamReader for VXL {
        fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> std::io::Result<Self> {
            let mut cols: Vec<Vec<Column>> = (0..512).map(|_| (0..512).map(|_| Column::read_from(buffer, order)).try_collect()).try_collect()?;

            Ok(Self{ cols })
        }
    }

    struct Column {
        data: VecDeque<Span>
    }

    impl StreamReader for Column {
        fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> std::io::Result<Self> {
            let mut data = vec![];
            let mut current_span = Span::read_from(buffer, order)?;

            while current_span.header.length != 0 {
                data.push(current_span);
                current_span = Span::read_from(buffer, order)?;
            }

            data.push(current_span);
            Ok(Self { data: VecDeque::from(data) })
        }
    }

    #[derive(PartialEq)]
    struct Span {
        header: SpanHeader,
        colors: Vec<BGRAColor>
    }

    impl StreamReader for Span {
        fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> std::io::Result<Self> {
            let header = SpanHeader::read_from(buffer, order)?;
            let mut vec = vec![];
            let run;

            if header.length == 0 {
                run = Run::LastSpan {header};
            } else {
                run = Run::Span {header}
            }

            for _ in 0..(run.size() - 1) {
                vec.push(BGRAColor::read_from(buffer,order)?);
            }

            Ok(Self{ header, colors: vec })
        }
    }


    #[derive(Copy, Clone, PartialEq)]
    struct SpanHeader {
        /// N
        length: u8,
        /// S
        starting_height_tcr: u8,
        /// E
        ending_height_tcr: u8,
        /// A
        starting_height_air: u8
    }

    impl SpanHeader {
        fn get_z(&self) -> u8 {
            return (self.length - 1) - self.get_k();
        }

        fn get_k(&self) -> u8 {
            return (self.ending_height_tcr - self.starting_height_tcr) + 1;
        }

        fn starting_height_solid(&self) -> u8 { self.ending_height_tcr + 1 }

        fn ending_height_air(&self) -> u8 { self.starting_height_tcr - 1 }

        fn length_air(&self) -> u8 {self.starting_height_tcr - self.starting_height_air }
        fn length_tcr (&self) -> u8 { self.get_k() }
        fn length_bcr (&self) -> u8 { self.get_z() }
    }

    impl StreamReader for SpanHeader {
        fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> std::io::Result<Self> {
            Ok(Self {
                length: u8::read_from(buffer,order)?,
                starting_height_tcr: u8::read_from(buffer,order)?,
                ending_height_tcr: u8::read_from(buffer,order)?,
                starting_height_air: u8::read_from(buffer,order)?
            })
        }
    }
    #[derive(PartialEq)]
    struct BGRAColor {
        b: u8,
        g: u8,
        r: u8,
        a: u8
    }

    impl StreamReader for BGRAColor {
        fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> std::io::Result<Self> {
            Ok(Self {
                b: u8::read_from(buffer,order)?,
                g: u8::read_from(buffer,order)?,
                r: u8::read_from(buffer,order)?,
                a: u8::read_from(buffer,order)?
            })
        }
    }

    /// Internal representation used for reading a "Run" of voxel data.
    /// Equivalent to `Span`
    enum Run {
        LastSpan {
            header: SpanHeader
        },
        Span {
            header: SpanHeader
        }
    }

    impl Run {
        fn size(&self) -> u8 {
            return match self {
                Run::LastSpan { header, .. } => { 1 + header.get_k() }
                Run::Span { header, .. } => { header.length }
            }
        }
    }

    enum Voxel {
        Open,
        Colored {color: BGRAColor},
        /// Has `color` due to **Surface Voxel Rule**
        Solid {color: BGRAColor}
    }

    impl Column {
        fn starting_height_bcr(&self, current_span: &Span) -> Option<u8> {
            let m = self.get_m(current_span)?;
            Some(m - current_span.header.get_z())
        }

        fn length_solid(&self, current_span: &Span) -> Option<u8> {
            let m = self.get_m(current_span)?;
            Some(m - current_span.header.get_z() - current_span.header.starting_height_solid())
        }

        fn ending_height_bcr(&self, current_span: &Span) -> Option<u8> {
            let m = self.get_m(current_span)?;
            Some(m + 1)
        }

        fn ending_height_solid(&self, current_span: &Span) -> Option<u8> {
            let m = self.get_m(current_span)?;
            Some(m - current_span.header.get_z() - 1)
        }

        fn get_m(&self, current_span: &Span) -> Option<u8> {
            if current_span.header.length == 0 {
                return Some(64)
            }

            let idx  = &self.data
                .iter()
                .position(|it| *it == *current_span)?;

            let next = &self.data.get(*idx + 1)?;

            Some(next.header.starting_height_air)
        }
    }
}
