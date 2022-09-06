use crate::{_AX, cpuid, CpuidResult};

pub struct MicroArchLevel(u8);

impl MicroArchLevel {
    const BASE_LINE: u32 = {
        /* 00_01_EDX */
        const FPU: u32  = 1 <<  0;
        const CX8: u32  = 1 <<  8;
        const SCE: u32  = 1 << 11;
        const CMOV: u32 = 1 << 15;
        const MMX: u32  = 1 << 23;
        const FXSR: u32 = 1 << 24;
        const SSE: u32  = 1 << 25;
        const SSE2: u32 = 1 << 26;

        FPU | CX8 | SCE | CMOV | MMX | FXSR | SSE | SSE2
    };

    const X86_64_V2: [u32; 2] = { 
        /* 00_01_ECX */
        const SSE3: u32       = 1 << 0;
        const SSSE3: u32      = 1 << 9;
        const CMPXCHG16B: u32 = 1 << 13;
        const SSE4_1: u32     = 1 << 19;
        const SSE4_2: u32     = 1 << 20;
        const POPCNT: u32     = 1 << 23;
        /* 80_01_ECX */
        const LAHF_SAHF: u32 = 1 << 0;

        [
            SSE3 | SSSE3 | CMPXCHG16B | SSE4_1 | SSE4_2 | POPCNT,
            LAHF_SAHF,
        ]
    };

    const X86_64_V3: [u32; 3] = {
        /* 00_01_ECX */
        const FMA: u32     = 1 << 12;
        const MOVBE: u32   = 1 << 22;
        const OSXSAVE: u32 = 1 << 27;
        const AVX: u32     = 1 << 28;
        const F16C: u32    = 1 << 29;

        /* 00_07_EBX */
        const BMI1: u32 = 1 << 3;
        const AVX2: u32 = 1 << 5;
        const BMI2: u32 = 1 << 8;

        /* 80_01_ECX */
        const ABM_LZCNT: u32 = 1 << 5;

        [
            FMA | MOVBE | OSXSAVE | AVX | F16C,
            BMI1 | AVX2 | BMI2,
            ABM_LZCNT,
        ]
    };

    const X86_64_V4: u32 = {
        /* 00_07_EBX */
        const AVX512F: u32  = 1 << 16;
        const AVX512DQ: u32 = 1 << 17;
        const AVX512CD: u32 = 1 << 28;
        const AVX512BW: u32 = 1 << 30;
        const AVX512VL: u32 = 1 << 31;

        AVX512F | AVX512DQ | AVX512CD | AVX512BW | AVX512VL 
    };
    
    fn set_cpuid() -> [CpuidResult; 3] {
        [
            cpuid!(0x1, 0x0),
            cpuid!(0x7, 0x0),
            cpuid!(_AX+0x1, 0x0),
        ]
    }

    fn from_cpuid_array(cpuid_array: [CpuidResult; 3]) -> Self {
        let [cpuid_00_01, cpuid_00_07, cpuid_80_01] = cpuid_array;

        let mask = |bitmask: &[u32], cpuid: &[u32]| -> bool {
            for (bitmask, cpuid) in bitmask.iter().zip(cpuid) {
                if (bitmask & cpuid) != *bitmask {
                    return false;
                }
            }

            true
        };

        let base_line = mask(&[Self::BASE_LINE], &[cpuid_00_01.edx]);
        let x86_64_v2 = mask(&Self::X86_64_V2, &[cpuid_00_01.ecx, cpuid_80_01.ecx]);
        let x86_64_v3 = mask(&Self::X86_64_V3,
            &[cpuid_00_01.ecx, cpuid_00_07.ebx, cpuid_80_01.ecx]);
        let x86_64_v4 = mask(&[Self::X86_64_V4], &[cpuid_00_07.ebx]);

        let mut level = if base_line { 1 } else { 0 };

        if x86_64_v2 { level |= 1 << 1 }
        if x86_64_v3 { level |= 1 << 2 }
        if x86_64_v4 { level |= 1 << 3 }

        Self(level)
    }
    pub fn check() -> u8 {
        let cpuid_array = Self::set_cpuid();

        Self::from_cpuid_array(cpuid_array).0
    }
    pub fn to_u8(&self) -> u8 {
        match self.0 {
            0b0001 => 1,
            0b0011 => 2,
            0b0111 => 3,
            0b1111 => 4,
            _ => 0,
        }
    }
}

#[test]
fn test_micro_arch_level() {
    let cpuid_array = [
        /* Ryzen 5 5600G */
        /* 00_01 */
        CpuidResult { eax: 0x00A50F00, ebx: 0x0A0C0800, ecx: 0x7EF8320B, edx: 0x178BFBFF },
        /* 00_07 */
        CpuidResult { eax: 0x00000000, ebx: 0x219C97A9, ecx: 0x0040068C, edx: 0x00000010 },
        /* 80_01 */
        CpuidResult { eax: 0x00A50F00, ebx: 0x20000000, ecx: 0x75C237FF, edx: 0x2FD3FBFF },
    ];

    let level = MicroArchLevel::from_cpuid_array(cpuid_array);

    assert_eq!(0b0111, level.0);

    /*
    println!("MicroArchLevel: 0b{:04b}", level.0);
    println!("MicroArchLevel: {}", level.to_u8());
    */
}
