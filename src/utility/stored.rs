use std::{
    fs::{create_dir_all, remove_file, File},
    io::{Read, Write},
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Rmp,
    Ron,
}

impl Kind {
    pub fn ext(self) -> String {
        format!("{self:?}").to_lowercase()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stored<'s, T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    dir: &'s str,
    key: &'s str,
    kind: Kind,
    value: T,
}

impl<'s, T> Stored<'s, T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub const DIR: &str = "data";

    pub fn ext(&self) -> String {
        format!("{:?}", self.kind).to_lowercase()
    }
    pub fn dir(&self) -> PathBuf {
        PathBuf::from(Self::DIR).join(self.dir)
    }
    pub fn path(&self) -> PathBuf {
        self.dir().join(self.key).with_extension(self.ext())
    }

    pub const fn new(dir: &'s str, key: &'s str, kind: Kind, value: T) -> Self {
        Self {
            dir,
            key,
            kind,
            value,
        }
    }
    pub fn read(dir: &'s str, key: &'s str, kind: Kind) -> Result<Self> {
        let path = PathBuf::from(Self::DIR)
            .join(dir)
            .join(key)
            .with_extension(kind.ext());

        let mut file = File::open(path)?;
        let value = match kind {
            Kind::Rmp => rmp_serde::from_read(file)?,
            Kind::Ron => {
                let mut string = String::new();
                file.read_to_string(&mut string)?;
                ron::from_str(&string)?
            }
        };

        Ok(Self::new(dir, key, kind, value))
    }

    pub fn storage_resync(self) -> Result<(T, Self)> {
        let old = self.value;
        let new = Self::read(self.dir, self.key, self.kind)?;

        Ok((old, new))
    }
    pub fn storage_write(&self) -> Result<()> {
        let data = &match self.kind {
            Kind::Rmp => rmp_serde::to_vec(&self.value)?,
            Kind::Ron => ron::to_string(&self.value)?.as_bytes().to_vec(),
        };

        create_dir_all(self.dir())?;

        let mut file = File::create(self.path())?;
        file.write_all(data)?;
        file.flush().map_err(Into::into)
    }
    pub fn storage_delete(self) -> Result<T> {
        remove_file(self.path())?;
        Ok(self.value)
    }
    pub fn storage_rename(self, dir: &'s str, key: &'s str, kind: Kind) -> Result<Self> {
        let value = self.storage_delete()?;
        let moved = Self::new(dir, key, kind, value);

        moved.storage_write()?;
        Ok(moved)
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn unwrap(self) -> T {
        self.value
    }
}

impl<'s, T> const Deref for Stored<'s, T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'s, T> DerefMut for Stored<'s, T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
