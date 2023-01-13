use {regex::bytes::Regex, std::fmt};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Endian {
    Big = 0,
    Little = 1,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Arch {
    PowerPc = 0,
    Mips = 1,
    Arm = 2,
    X86 = 3,
}

#[derive(Eq, Hash, PartialEq, Clone)]
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

pub fn magics() -> Vec<Box<dyn Magic>> {
    vec![
        Box::new(MipsBigEndianAdduiPrologue),
        Box::new(MipsLittleEndianAdduiPrologue),
        Box::new(MipsBigEndianJrAddui),
        Box::new(MipsLittleEndianJrAddui),
        Box::new(MipsBigEndianAdduiJr),
        Box::new(MipsLittleEndianAdduiJr),
        Box::new(PowerPcBigEndianMflrPrologue),
        Box::new(PowerPcLittleEndianMflrPrologue),
        Box::new(PowerPcBigEndianBlrEpilogue),
        Box::new(PowerPcLittleEndianBlrEpilogue),
        Box::new(ArmBigEndianStmfdPrologue),
        Box::new(ArmLittleEndianStmfdPrologue),
        Box::new(ArmBigEndianLdmfdEpilogue),
        Box::new(ArmLittleEndianLdmfdEpilogue),
        Box::new(X86LittleEndianPushEbpMovPrologue),
        Box::new(X86LittleEndianPushEbpMovPrologue2),
        Box::new(X86LittleEndianNopSlide),
        Box::new(X86LittleEndianEndBr64),
    ]
}

struct MipsBigEndianAdduiPrologue;

/// addui $sp, -XX
/// sw XX, XX($sp)
/// 27 bd ff xx
/// af bx xx xx
impl Magic for MipsBigEndianAdduiPrologue {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Big,
            arch: Arch::Mips,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u)\x27\xbd\xff.{5}").unwrap()
    }
    fn matches(&self, data: &[u8]) -> bool {
        if data[4] != 0xaf {
            return false;
        }

        if data[5] & 0xe0 != 0xa0 {
            return false;
        }

        true
    }
}

struct MipsLittleEndianAdduiPrologue;

/// addui $sp, -XX
/// sw XX, XX($sp)
/// xx ff bd 27
/// xx xx bx af
impl Magic for MipsLittleEndianAdduiPrologue {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Little,
            arch: Arch::Mips,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u).\xff\xbd\x27.{4}").unwrap()
    }
    fn matches(&self, data: &[u8]) -> bool {
        if data[7] != 0xaf {
            return false;
        }

        if data[6] & 0xe0 != 0xa0 {
            return false;
        }

        true
    }
}

struct MipsBigEndianJrAddui;

/// jr $ra
/// addui $sp, XX
impl Magic for MipsBigEndianJrAddui {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Big,
            arch: Arch::Mips,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u)\x03\xe0\x00\x08\x27\xbd.{2}").unwrap()
    }
    fn matches(&self, _data: &[u8]) -> bool {
        true
    }
}

struct MipsLittleEndianJrAddui;

/// jr $ra
/// addui $sp, XX
impl Magic for MipsLittleEndianJrAddui {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Little,
            arch: Arch::Mips,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u)\x08\x00\xe0\x03.{2}\xbd\x27").unwrap()
    }
    fn matches(&self, _data: &[u8]) -> bool {
        true
    }
}

struct MipsBigEndianAdduiJr;

/// addui $sp, XX
/// jr $ra
impl Magic for MipsBigEndianAdduiJr {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Big,
            arch: Arch::Mips,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u)\x27\xbd.{2}\x03\xe0\x00\x08").unwrap()
    }
    fn matches(&self, _data: &[u8]) -> bool {
        true
    }
}

struct MipsLittleEndianAdduiJr;

/// addui $sp, XX
/// jr $ra
impl Magic for MipsLittleEndianAdduiJr {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Little,
            arch: Arch::Mips,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u).{2}\xbd\x27\x08\x00\xe0\x03").unwrap()
    }
    fn matches(&self, _data: &[u8]) -> bool {
        true
    }
}

struct PowerPcBigEndianMflrPrologue;

/// mflr r0
impl Magic for PowerPcBigEndianMflrPrologue {
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

struct PowerPcLittleEndianMflrPrologue;

/// mflr r0
impl Magic for PowerPcLittleEndianMflrPrologue {
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

struct PowerPcBigEndianBlrEpilogue;

/// blr
impl Magic for PowerPcBigEndianBlrEpilogue {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Big,
            arch: Arch::PowerPc,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u)\x4e\x80\x00\x20").unwrap()
    }
    fn matches(&self, _data: &[u8]) -> bool {
        true
    }
}

struct PowerPcLittleEndianBlrEpilogue;

/// blr
impl Magic for PowerPcLittleEndianBlrEpilogue {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Little,
            arch: Arch::PowerPc,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u)\x20\x00\x80\x4e").unwrap()
    }
    fn matches(&self, _data: &[u8]) -> bool {
        true
    }
}

struct ArmBigEndianStmfdPrologue;

/// stmfd sp!, {xx}
/// any instruction with op-code beginning with 0xe
impl Magic for ArmBigEndianStmfdPrologue {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Big,
            arch: Arch::Arm,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u)\xe9\x2d.{10}").unwrap()
    }
    fn matches(&self, data: &[u8]) -> bool {
        if data[4] & 0xf0 != 0xe0 {
            return false;
        }

        if data[8] & 0xf0 != 0xe0 {
            return false;
        }
        true
    }
}

struct ArmLittleEndianStmfdPrologue;

/// stmfd sp!, {xx}
/// any instruction with op-code beginning with 0xe
impl Magic for ArmLittleEndianStmfdPrologue {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Little,
            arch: Arch::Arm,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u).{2}\xe9\x2d.{8}").unwrap()
    }
    fn matches(&self, data: &[u8]) -> bool {
        if data[7] & 0xf0 != 0xe0 {
            return false;
        }

        if data[11] & 0xf0 != 0xe0 {
            return false;
        }
        true
    }
}

struct ArmBigEndianLdmfdEpilogue;

/// mov r0, xx
/// ldmfd sp!, {xx}
impl Magic for ArmBigEndianLdmfdEpilogue {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Big,
            arch: Arch::Arm,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u)\xe1\xa0.{2}\xe8\x8d.{2}").unwrap()
    }
    fn matches(&self, _data: &[u8]) -> bool {
        true
    }
}

struct ArmLittleEndianLdmfdEpilogue;

/// mov r0, xx
/// ldmfd sp!, {xx}
impl Magic for ArmLittleEndianLdmfdEpilogue {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Little,
            arch: Arch::Arm,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u).{2}\xa0\xe1.{2}\x8d\xe8").unwrap()
    }
    fn matches(&self, _data: &[u8]) -> bool {
        true
    }
}

struct X86LittleEndianPushEbpMovPrologue;

/// push ebp
/// mov  ebp, esp
/// sub esp, xx
impl Magic for X86LittleEndianPushEbpMovPrologue {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Little,
            arch: Arch::X86,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u)\x55\x89\xe5\x83\xec.").unwrap()
    }
    fn matches(&self, _data: &[u8]) -> bool {
        true
    }
}

struct X86LittleEndianPushEbpMovPrologue2;

/// push rbp
/// mov  ebp, esp
/// push rdi
/// push rsi
impl Magic for X86LittleEndianPushEbpMovPrologue2 {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Little,
            arch: Arch::X86,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u)\x55\x89\xe5\x57\x56.").unwrap()
    }
    fn matches(&self, _data: &[u8]) -> bool {
        true
    }
}

struct X86LittleEndianNopSlide;

/// nop
/// nop
/// nop
/// nop
/// nop
/// nop
/// nop
/// nop
impl Magic for X86LittleEndianNopSlide {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Little,
            arch: Arch::X86,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u)\x90{8}").unwrap()
    }
    fn matches(&self, _data: &[u8]) -> bool {
        true
    }
}

struct X86LittleEndianEndBr64;

/// endbr64
impl Magic for X86LittleEndianEndBr64 {
    fn kind(&self) -> Kind {
        Kind {
            endian: Endian::Little,
            arch: Arch::X86,
        }
    }
    fn regex(&self) -> Regex {
        Regex::new(r"(?-u)\xf3\x0f\x1e\xfa").unwrap()
    }
    fn matches(&self, _data: &[u8]) -> bool {
        true
    }
}
