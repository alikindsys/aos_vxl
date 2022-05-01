#[derive(Hash, Eq)]
#[allow(non_camel_case_types)]
pub struct ipos8 {
    pub x: u8,
    pub y: u8,
    pub z: u8
}

#[allow(non_camel_case_types)]
pub struct ipos2 {
    pub x: u8,
    pub y: u8
}

impl PartialEq for ipos2 {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl ipos8 {
    /// Checks if another position has the same x,y coordinates, ignoring z.
    ///
    /// Its mostly used as eq for checking inside of a VoxelColumn.
    #[inline(always)]
    pub fn range_cmp(&self, other: ipos8) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl PartialEq for ipos8 {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other)
    }
}