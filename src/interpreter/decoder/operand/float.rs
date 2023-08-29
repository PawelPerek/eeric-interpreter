use eeric::prelude::*;

pub fn parse_r4_format(r: &str) -> Result<format::R4, String> {
    let tokens: Vec<&str> = r.split(", ").collect();

    if tokens.len() != 4 {
        return Err("Expected format: 'rd, rs1, rs2, rs3'".to_owned());
    }

    let rd = tokens[0];
    let rs1 = tokens[1];
    let rs2 = tokens[2];
    let rs3 = tokens[3];

    let rd = parse_operand(rd)?;
    let rs1 = parse_operand(rs1)?;
    let rs2 = parse_operand(rs2)?;
    let rs3 = parse_operand(rs3)?;

    Ok(format::R4 { rd, rs1, rs2, rs3 })
}

pub fn parse_operand(op_str: &str) -> Result<usize, String> {
    match op_str {
        "f0"  | "ft0"  => Ok(0),
        "f1"  | "ft1"  => Ok(1),
        "f2"  | "ft2"  => Ok(2),
        "f3"  | "ft3"  => Ok(3),
        "f4"  | "ft4"  => Ok(4),
        "f5"  | "ft5"  => Ok(5),
        "f6"  | "ft6"  => Ok(6),
        "f7"  | "ft7"  => Ok(7),
        "f8"  | "fs0"  => Ok(8),
        "f9"  | "fs1"  => Ok(9),
        "f10" | "fa0"  => Ok(10),
        "f11" | "fa1"  => Ok(11),
        "f12" | "fa2"  => Ok(12),
        "f13" | "fa3"  => Ok(13),
        "f14" | "fa4"  => Ok(14),
        "f15" | "fa5"  => Ok(15),
        "f16" | "fa6"  => Ok(16),
        "f17" | "fa7"  => Ok(17),
        "f18" | "fs2"  => Ok(18),
        "f19" | "fs3"  => Ok(19),
        "f20" | "fs4"  => Ok(20),
        "f21" | "fs5"  => Ok(21),
        "f22" | "fs6"  => Ok(22),
        "f23" | "fs7"  => Ok(23),
        "f24" | "fs8"  => Ok(24),
        "f25" | "fs9"  => Ok(25),
        "f26" | "fs10" => Ok(26),
        "f27" | "fs11" => Ok(27),
        "f28" | "ft8"  => Ok(28),
        "f29" | "ft9"  => Ok(29),
        "f30" | "ft10" => Ok(30),
        "f31" | "ft11" => Ok(31),
        _              => Err(format!("Incorrect float operand: {}", op_str))  
    }
}