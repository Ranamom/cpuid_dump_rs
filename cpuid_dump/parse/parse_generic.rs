use crate::*;

pub fn vendor_00_00h(cpuid: &CpuidResult) -> String {
    format!(" [{}]", Vendor::from_cpuid(cpuid).name)
}

pub fn info_00_01h(cpuid: &CpuidResult) -> String {
    let [eax, ebx] = [cpuid.eax, cpuid.ebx];

    let fms = cpuid_asm::FamModStep::from_cpuid(eax);

    let buff = [
        format!(" [F: 0x{:X}, M: 0x{:X}, S: 0x{:X}]", fms.syn_fam, fms.syn_mod, fms.step),
        padln!(),
        format!(" [Codename: {}]", fms.codename()),
        padln!(),
        format!(" [APIC ID: {}]", ebx >> 24),
        padln!(),
        format!(" [Total thread(s): {}T]", (ebx >> 16) & 0xFF),
        padln!(),
        format!(" [CLFlush: {}B]", ((ebx >> 8) & 0xFF) * 8),
    ];

    return concat_string_from_slice(&buff);
}

pub fn feature_00_01h(cpuid: &CpuidResult) -> String {
    let mut buff: Vec<String> = Vec::with_capacity(64);
    let [ecx, edx] = [cpuid.ecx, cpuid.edx];

    buff.extend(str_detect_ftr(edx, FTR_00_01_EDX_X0));
    buff.extend(str_detect_ftr(ecx, FTR_00_01_ECX_X0));

    let [ecx, edx] = [
        Reg::new(ecx).to_bool_array(),
        Reg::new(edx).to_bool_array(),
    ];

    if edx[25] {
        let v = [
            (edx[26], "2"), (ecx[0], "3"), (ecx[19], "4.1"), (ecx[20], "4.2"),
        ];
        let sse = ftr_variant_expand("SSE", &v);

        buff.push(sse.to_string());
    }
    if ecx[9] { buff.push("SSSE3".to_string()); }

    return align_mold_ftr(&buff);
}

pub fn feature_00_07h_x0(cpuid: &CpuidResult) -> String {
    let mut buff: Vec<String> = Vec::with_capacity(96);
    let [ebx, ecx, edx] = [cpuid.ebx, cpuid.ecx, cpuid.edx];

    buff.extend(str_detect_ftr(ebx, FTR_00_07_EBX_X0));
    buff.extend(str_detect_ftr(ecx, FTR_00_07_ECX_X0));
    buff.extend(str_detect_ftr(edx, FTR_00_07_EDX_X0));

    let [ebx, ecx, edx] = [
        Reg::new(ebx).to_bool_array(),
        Reg::new(ecx).to_bool_array(),
        Reg::new(edx).to_bool_array(),
    ];

    let avx512_f    = ebx[16];
    let avx512_dq   = ebx[17];
    let avx512_ifma = ebx[21];
    let avx512_cd   = ebx[28];
    let avx512_bw   = ebx[30];
    let avx512_vl   = ebx[31];

    if avx512_f || avx512_dq || avx512_ifma || avx512_cd
    || avx512_bw || avx512_vl {
        let v = [
            (avx512_f,    "F"),
            (avx512_dq,   "DQ"),
            (avx512_ifma, "IFMA"),
            (avx512_cd,   "CD"),
            (avx512_bw,   "BW"),
            (avx512_vl,   "VL")
        ];
        let avx512 = ftr_variant_expand("AVX512", &v);
        buff.push(avx512);
    }

    /* Intel Xeon Phi only */
    if ebx[26] && ebx[27] {
        buff.push("AVX512{PF,ER}".to_string());
    }

    // 0x00000007_ECX_x0
    let avx512_vbmi1     = ecx[ 1];
    let avx512_vbmi2     = ecx[ 6];
    let avx512_vnni      = ecx[11];
    let avx512_bitalg    = ecx[12];
    let avx512_vpopcntdq = ecx[14];

    if avx512_vbmi1 || avx512_vbmi2 || avx512_vnni
    || avx512_bitalg || avx512_vpopcntdq {
        let v = [
            (avx512_vbmi1,     "VBMI"),
            (avx512_vbmi2,     "VBMI2"),
            (avx512_vnni,      "VNNI"),
            (avx512_bitalg,    "BITALG"),
            (avx512_vpopcntdq, "VPOPCNTDQ"),
        ];
        let avx512 = ftr_variant_expand("AVX512", &v);
        buff.push(avx512);
    }

    // 0x00000007_EDX_x0
    /* Intel Xeon Phi Only */
    if edx[2] && edx[3] {
        buff.push("AVX512{4VNNIW,4FMAPS}".to_string());
    }

    let avx512_vp2intersect  = edx[ 8];
    let avx512_fp16          = edx[23];

    if avx512_vp2intersect || avx512_fp16 {
        let v = [
            (avx512_vp2intersect, "VP2INTERSECT"),
            (avx512_fp16,         "FP16"),
        ];
        let avx512 = ftr_variant_expand("AVX512", &v);
        buff.push(avx512);
    }

    /*
        Currently Intel Sapphire Rapids only
        Bit22 => AMX-BF16,
        Bit24 => AMX-TILE,
        Bit25 => AMX-INT8,
    */
    if edx[22] && edx[24] && edx[25] {
        /*
            let v = [
                (ebx[22], "BF16"),
                (ebx[24], "TILE"),
                (ebx[25], "INT8"),
            ];
            let amx = ftr_variant_expand("AMX", &v);
        */
        buff.push("AMX{BF16,TILE,INT8}".to_string());
    }

    return align_mold_ftr(&buff);
}

pub fn feature_00_07h_x1(eax: &u32) -> String {
    // https://github.com/torvalds/linux/commit/b85a0425d8056f3bd8d0a94ecdddf2a39d32a801
    let mut v = [""; 32];
    v[4] = "AVX_VNNI";
    v[5] = "AVX512_BF16";
    v[22] = "HRESET";
    v[26] = "LAM";

    let buff = str_detect_ftr(*eax, &v);

    return align_mold_ftr(&buff);
}

pub fn xstate_00_0dh(cpuid: &RawCpuid) -> String {
    let x0 = |eax: u32| -> String {
        let tmp = align_mold_ftr(&str_detect_ftr(eax, XFEATURE_MASK_00_0D_EAX_X0));

        if !tmp.is_empty() {
            format!(" [XFEATURE Mask:]{}{}",
                padln!(), tmp)
        } else {
            tmp
        }
    };
    let x1 = |eax: u32| -> String {
        align_mold_ftr(&str_detect_ftr(eax, XSAVE_00_0D_EAX_X1))
    };

    let size = |eax: u32, txt: &str| -> String {
        /* 00_0D_X{SUB}:EAX is the state size, EAX = 0 indicates not supported it */
        if eax != 0x0 {
            format!(" [{}: size({})]", txt, eax)
        } else {
            "".to_string()
        }
    };

    let eax = cpuid.result.eax;

    return match cpuid.sub_leaf {
        0x0 => x0(eax),
        0x1 => x1(eax),
        0x2 => size(eax, "XSTATE"),
        0x9 => size(eax, "Protection Key"),
        0xB => size(eax, "CET User"),
        0xC => size(eax, "CET SuperVisor"),
        _ => "".to_string(),
    };
}

pub fn feature_80_01h(cpuid: &CpuidResult) -> String {
    let [ecx, edx] = [ cpuid.ecx, cpuid.edx, ];

    // 0x8000_0001_ECX_x0
    let mut buff = str_detect_ftr(ecx, FTR_80_01_ECX_X0);

    // 0x8000_0001_EDX_x0
    let edx = Reg::new(edx).to_bool_array();
    if edx[31] {
        let v = [ (edx[30], "EXT") ];
        let tdnow = ftr_variant_expand("3DNow!", &v);
        buff.push(tdnow);
    }

    return align_mold_ftr(&buff);
}

pub fn addr_size_80_08h(eax: &u32) -> String {
    let pad = format!("{}{}", padln!(), " ".repeat(" [Address size:".len()));

    format!(" [Address size: {:2}-bits physical {} {:2}-bits virtual]",
        eax & 0xFF, pad, (eax >> 8) & 0xFF)
}

pub fn cache_prop(cpuid: &CpuidResult) -> String {
    let cache = cpuid_asm::CacheProp::from_cpuid(cpuid);

    if cache.level == 0 { return "".to_string(); }

    let v = [
        format!(" [L{}{}, {:>3}-way, {:>4}-{}]",
            cache.level, &cache.cache_type_string[..1], cache.way,
            cache.size / cache.size_unit_byte, cache.size_unit_string),
        padln!(),
        format!(" [Shared {}T]", cache.share_thread),
        if cache.inclusive { " [Inclusive]" } else { "" }.to_string(),
        // has_ftr!(cache.inclusive, " [Inclusive]").to_string(),
    ];

    return concat_string_from_slice(&v);
}
