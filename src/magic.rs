use {regex::bytes::Regex, std::fmt};

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Endian {
    Big = 0,
    Little = 1,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Arch {
    PowerPc = 0,
}

#[derive(Eq, Hash, PartialEq)]
pub struct Kind {
    endian: Endian,
    arch: Arch,
}

impl fmt::Debug for Kind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?} {:?}", self.arch, self.endian)
    }
}

pub trait Magic {
    fn kind(&self) -> Kind;
    fn regex(&self) -> Regex;
    fn matches(&self, data: &[u8]) -> bool;
}

pub struct PowerPcBigEndianMflr;
pub struct PowerPcLittleEndianMflr;

impl Magic for PowerPcBigEndianMflr {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Big,
            arch: Arch::PowerPc,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u)\x7c\x08\x02\xa6").unwrap()
    }
    fn matches(&self, _data: &[u8]) -> bool {
        true
    }
}

impl Magic for PowerPcLittleEndianMflr {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Little,
            arch: Arch::PowerPc,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u)\xa6\x02\x08\x7c").unwrap()
    }
    fn matches(&self, _data: &[u8]) -> bool {
        true
    }
}

pub fn magics() -> Vec<Box<dyn Magic>> {
    vec![
        Box::new(PowerPcBigEndianMflr),
        Box::new(PowerPcLittleEndianMflr),
    ]
}
