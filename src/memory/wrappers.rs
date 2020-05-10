use byteorder::{ByteOrder, ReadBytesExt};
use serde::{de::DeserializeOwned, Serialize};
use std::io::Cursor;

pub trait CursorWrapper: From<<Self as CursorWrapper>::Item> {
    type Item: Serialize + DeserializeOwned;
    fn read<E: ByteOrder>(
        cursor: Cursor<Vec<u8>>,
    ) -> Result<Self::Item, Box<dyn std::error::Error>>;
}

impl CursorWrapper for u8 {
    type Item = u8;
    fn read<E: ByteOrder>(
        cursor: Cursor<Vec<u8>>,
    ) -> Result<Self::Item, Box<dyn std::error::Error>> {
        cursor.read_u8().map_err(|e| e.into())
    }
}

impl CursorWrapper for u16 {
    type Item = u16;
    fn read<E: ByteOrder>(
        cursor: Cursor<Vec<u8>>,
    ) -> Result<Self::Item, Box<dyn std::error::Error>> {
        cursor.read_u16::<E>().map_err(|e| e.into())
    }
}

impl CursorWrapper for u32 {
    type Item = u32;
    fn read<E: ByteOrder>(
        cursor: Cursor<Vec<u8>>,
    ) -> Result<Self::Item, Box<dyn std::error::Error>> {
        cursor.read_u32::<E>().map_err(|e| e.into())
    }
}

impl CursorWrapper for u64 {
    type Item = u64;
    fn read<E: ByteOrder>(
        cursor: Cursor<Vec<u8>>,
    ) -> Result<Self::Item, Box<dyn std::error::Error>> {
        cursor.read_u64::<E>().map_err(|e| e.into())
    }
}

impl CursorWrapper for f32 {
    type Item = f32;
    fn read<E: ByteOrder>(
        cursor: Cursor<Vec<u8>>,
    ) -> Result<Self::Item, Box<dyn std::error::Error>> {
        cursor.read_f32::<E>().map_err(|e| e.into())
    }
}

impl CursorWrapper for f64 {
    type Item = f64;
    fn read<E: ByteOrder>(
        cursor: Cursor<Vec<u8>>,
    ) -> Result<Self::Item, Box<dyn std::error::Error>> {
        cursor.read_f64::<E>().map_err(|e| e.into())
    }
}
