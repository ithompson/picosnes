use std::io::{self, Read};

#[derive(Clone)]
pub struct NesFile {
    pub nametable_layout: NametableLayout,
    pub nvram_present: bool,
    pub mapper: MapperId,
    pub timing: TimingMode,
    pub console_type: ConsoleType,
    pub misc_rom_count: u8,
    pub default_expansion_device: u8,
    pub prg_ram_size: usize,
    pub prg_nvram_size: usize,
    pub chr_ram_size: usize,
    pub chr_nvram_size: usize,

    pub trainer: Option<[u8; 512]>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub misc_rom: Vec<u8>,
}

impl NesFile {
    pub fn from_stream(reader: &mut dyn Read) -> Result<Self, io::Error> {
        let header = {
            let mut buf = [0u8; 16];
            reader.read_exact(&mut buf)?;
            buf
        };
        if &header[0..=3] != b"NES\x1A" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid NES file",
            ));
        }

        let prg_rom_size_lsb = header[4];
        let chr_rom_size_lsb = header[5];

        let nametable_layout = NametableLayout::from_flags(
            (header[6] & 0b0000_0001) != 0,
            (header[6] & 0b0000_1000) != 0,
        );
        let nvram_present = header[6] & 0b0000_0010 != 0;
        let trainer_present = header[6] & 0b0000_0100 != 0;
        let mapper_0to3 = header[6] >> 4;

        let console_base_type = header[7] & 0x3;
        let mapper_7to4 = header[7] >> 4;

        let mapper_11to8 = header[8] & 0x0F;
        let submapper_id = header[8] >> 4;

        let prg_rom_size_msb = header[9] & 0x0F;
        let chr_rom_size_msb = header[9] >> 4;

        let prg_ram_size = decode_ram_size(header[10] & 0x0F);
        let prg_nvram_size = decode_ram_size(header[10] >> 4);

        let chr_ram_size = decode_ram_size(header[11] & 0x0F);
        let chr_nvram_size = decode_ram_size(header[11] >> 4);

        let timing = TimingMode::from_u8(header[12] & 0x03)?;

        let console_extended_type = header[13];

        let misc_rom_count = header[14] & 0x3;
        let default_expansion_device = header[15] & 0x3F;

        let prg_rom_size = decode_rom_size(prg_rom_size_lsb, prg_rom_size_msb, 16 * 1024);
        let chr_rom_size = decode_rom_size(chr_rom_size_lsb, chr_rom_size_msb, 8 * 1024);

        let mapper_id =
            (mapper_11to8 as u16) << 8 | (mapper_7to4 as u16) << 4 | (mapper_0to3 as u16);
        let mapper = MapperId {
            id: mapper_id,
            sub_id: submapper_id,
        };
        let console_type = ConsoleType::from_flags(console_base_type, console_extended_type)?;

        let trainer = if trainer_present {
            let mut buf = [0u8; 512];
            reader.read_exact(&mut buf)?;
            Some(buf)
        } else {
            None
        };
        let prg_rom = read_exact_to_vec(reader, prg_rom_size)?;
        let chr_rom = read_exact_to_vec(reader, chr_rom_size)?;
        let misc_rom = {
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;
            buf
        };

        Ok(NesFile {
            nametable_layout,
            nvram_present,
            mapper,
            timing,
            console_type,
            misc_rom_count,
            default_expansion_device,
            prg_ram_size,
            prg_nvram_size,
            chr_ram_size,
            chr_nvram_size,
            trainer,
            prg_rom,
            chr_rom,
            misc_rom,
        })
    }
}

fn decode_rom_size(size_lsb: u8, size_msb: u8, scale: usize) -> usize {
    if size_msb == 0xF {
        let multiplier = (size_lsb & 0x3) as usize;
        let exponent = (size_lsb >> 2) as usize;
        (multiplier * 2 + 1) * (1 << exponent)
    } else {
        let size_in_units = ((size_msb as usize) << 8) | (size_lsb as usize);
        size_in_units * scale
    }
}

fn decode_ram_size(shift: u8) -> usize {
    if shift == 0 {
        0
    } else {
        64 << (shift as usize)
    }
}

fn read_exact_to_vec(reader: &mut dyn Read, len: usize) -> Result<Vec<u8>, io::Error> {
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;
    Ok(buf)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NametableLayout {
    Vertical,
    Horizontal,
    AlternateVertical,
    AlternateHorizontal,
}

impl NametableLayout {
    fn from_flags(arrangement: bool, alternative: bool) -> Self {
        match (arrangement, alternative) {
            (false, false) => NametableLayout::Vertical,
            (true, false) => NametableLayout::Horizontal,
            (false, true) => NametableLayout::AlternateVertical,
            (true, true) => NametableLayout::AlternateHorizontal,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MapperId {
    pub id: u16,
    pub sub_id: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TimingMode {
    NTSC,
    PAL,
    MultiRegion,
    Dendy,
}

impl TimingMode {
    fn from_u8(value: u8) -> io::Result<Self> {
        match value {
            0 => Ok(TimingMode::NTSC),
            1 => Ok(TimingMode::PAL),
            2 => Ok(TimingMode::MultiRegion),
            3 => Ok(TimingMode::Dendy),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid TimingMode value {}", value),
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConsoleType {
    NES,
    VsSystem { ppu: u8, hardware: u8 },
    Playchoice,
    FamicloneDecimal,
    FamicomEPSM,
    VT01,
    VT02,
    VT03,
    VT09,
    VT32,
    VT369,
    UM6578,
    FamicomNetworkSystem,
}

impl ConsoleType {
    fn from_flags(base_type: u8, extended_type: u8) -> io::Result<Self> {
        match base_type {
            0 => Ok(ConsoleType::NES),
            1 => Ok(ConsoleType::VsSystem {
                ppu: extended_type & 0x0F,
                hardware: extended_type >> 4,
            }),
            2 => Ok(ConsoleType::Playchoice),
            3 => match extended_type & 0x0F {
                0x3 => Ok(ConsoleType::FamicloneDecimal),
                0x4 => Ok(ConsoleType::FamicomEPSM),
                0x5 => Ok(ConsoleType::VT01),
                0x6 => Ok(ConsoleType::VT02),
                0x7 => Ok(ConsoleType::VT03),
                0x8 => Ok(ConsoleType::VT09),
                0x9 => Ok(ConsoleType::VT32),
                0xA => Ok(ConsoleType::VT369),
                0xB => Ok(ConsoleType::UM6578),
                0xC => Ok(ConsoleType::FamicomNetworkSystem),
                _ => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid ConsoleType extended code {}", extended_type),
                )),
            },
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid ConsoleType base code {}", base_type),
            )),
        }
    }
}
