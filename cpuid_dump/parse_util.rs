use crate::*;

#[macro_export]
macro_rules! has_ftr {
    ($ftr_bool: expr, $name_str: expr) => {
        if $ftr_bool {
            $name_str
        } else {
            ""
        }
    };
    ($ftr_bool: expr, $name_str: expr, $else_ftr: expr, $else_name: expr) => {
        if $ftr_bool {
            $name_str
        } else if $else_ftr {
            $else_name
        } else {
            ""
        }
    };
}

/*
#[macro_export]
macro_rules! push {
    ($buff: expr, $str: expr) => {
        $buff.push($str.to_string())
    };
}
*/

#[macro_export]
macro_rules! pad {
    () => { " ".repeat(crate::INPUT_WIDTH + crate::OUTPUT_WIDTH + 1) };
}

#[macro_export]
macro_rules! padln {
    () => { format!("\n{}", pad!()) };
}

pub struct Reg(u32);
impl Reg {
    pub fn new(reg: u32) -> Reg {
        Reg(reg)
    }
    pub fn to_bit_array(&self) -> [u8; 32] {
        let mut bit = [0u8; 32];
        for i in 0..32 {
            bit[i] = ((self.0 >> i) & 1) as u8;
        }
        return bit;
    }
    pub fn to_bool_array(&self) -> [bool; 32] {
        let mut array = [false; 32];
        for (idx, v) in self.to_bit_array().iter().enumerate() {
            array[idx] = *v == 1;
        }
        return array;
    }
    /*
    pub fn to_bitvec(&self) -> Vec<u8> {
        self.to_bit_array().to_vec()
    }
    pub fn to_boolvec(&self) -> Vec<bool> {
        self.to_bool_array().to_vec()
    }
    */
}

pub fn cpu_name(cpuid: &CpuidResult) -> String {
    let mut name = Vec::with_capacity(64);

    [cpuid.eax, cpuid.ebx, cpuid.ecx, cpuid.edx].iter().for_each(
        |val| name.extend(val.to_le_bytes().iter().map(
            // replace from \u0000..\u001F (<Control>) to \u0020 (space)
            |&byte| if byte <= 0x1F { 0x20 } else { byte }
        ))
    );

    return String::from_utf8(name).unwrap();
}

pub fn str_detect_ftr(reg: u32, ftr_str: &[&str]) -> Vec<String> {
    //  let len = std::cmp::min(32, ftr_str.len());
    let reg = Reg::new(reg).to_bool_array();
    let mut buff: Vec<String> = Vec::with_capacity(32);

    for (r, ftr) in reg.iter().zip(&ftr_str[..]) {
        if *r && 0 < ftr.len() {
            buff.push(ftr.to_string());
        }
    }

    return buff;
}

pub fn concat_string_from_slice(src: &[String]) -> String {
    let mut dst = String::new();

    src.iter().for_each(
        |val| dst.push_str(&val)
    );

    return dst;
}

pub fn align_mold_ftr(buff: &[String]) -> String {
    let mut rest: usize = PARSE_WIDTH;
    let mut len: usize;
    let mut mold = String::new();
    let mut _inner = String::new();

    const DECO_LEN: usize = " []".len();
    
    for v in buff {
        len = v.len() + DECO_LEN;

        if len <= rest {
            _inner = format!(" [{}]", v);
            rest -= len;
        } else {
            if PARSE_WIDTH < len {
                /*
                _inner = format!(" [{}{}  {}]",
                    &v[..rest], padln!(), &v[rest..]);
                rest = PARSE_WIDTH - (len - rest);
                */
                _inner = format!("{} [{}]", padln!(), v);
                rest = 0;
            } else {
                _inner = format!("{} [{}]", padln!(), v);
                rest = PARSE_WIDTH - len;
            };
        }
        mold.push_str(&_inner);
    }

    return mold;
}

pub fn ftr_variant_expand(base_name: &str, flag_str: &[(bool, &str)]) -> String {
    let mut base = String::from(base_name);
    base.push_str("{");

    for (flag, name) in flag_str {
        if *flag {
            base.push_str(name);
            base.push_str(",");
        }
    }
    base.pop();

    base.push_str("}");

    return base;
}
