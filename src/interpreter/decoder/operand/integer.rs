use eeric::prelude::*;

pub fn parse_r_format(r: &str) -> Result<format::R, String> {
    let tokens: Vec<&str> = r.split(", ").collect();

    if tokens.len() != 3 {
        return Err("Expected format: 'rd, rs1, rs2'".to_owned());
    }

    let rd = tokens[0];
    let rs1 = tokens[1];
    let rs2 = tokens[2];

    let rd = parse_operand(rd)?;
    let rs1 = parse_operand(rs1)?;
    let rs2 = parse_operand(rs2)?;

    Ok(format::R { rd, rs1, rs2 })
}

pub fn parse_i_format(i: &str) -> Result<format::I, String> {
    let tokens: Vec<&str> = i.split(", ").collect();

    if tokens.len() != 3 {
        return Err("Expected format: 'rd, rs1, imm'".to_owned());
    }

    let rd = tokens[0];
    let rs1 = tokens[1];
    let imm = tokens[2];

    let rd = parse_operand(rd)?;
    let rs1 = parse_operand(rs1)?;
    let imm = parse_immediate(imm)?;

    Ok(format::I { rd, rs1, imm12: imm })
}

pub fn parse_s_format(s: &str) -> Result<format::S, String> {
    let tokens: Vec<&str> = s.split(", ").collect();

    if tokens.len() != 2 {
        return Err("Expected format: 'rs2, immediate(rs1)'".to_string());
    }

    let rs2 = tokens[0].trim();
    let imm_rs1 = tokens[1].trim();

    if !imm_rs1.ends_with(")") {
        return Err("Malformed format: missing closing ')'".to_string());
    }

    let inner_part = &imm_rs1[..imm_rs1.len() - 1];  // Remove the closing ')'
    let parts: Vec<&str> = inner_part.split('(').collect();

    if parts.len() != 2 {
        return Err("Expected format: 'immediate(rs1)' for the second part".to_string());
    }

    let imm = parts[0].trim();
    let rs1 = parts[1].trim();

    let rs1 = parse_operand(rs1)?;
    let rs2 = parse_operand(rs2)?;
    let imm = parse_immediate(imm)?;

    Ok(format::S { rs1, rs2, imm12: imm })
}

pub fn parse_u_format(u: &str) -> Result<format::U, String> {
    let tokens: Vec<&str> = u.split(", ").collect();

    if tokens.len() != 2 {
        return Err("Expected format: 'rd, imm'".to_owned());
    }

    let rd = tokens[0];
    let imm = tokens[1];

    let rd = parse_operand(rd)?;
    let imm = parse_immediate(imm)?;

    Ok(format::U { rd, imm20: imm })
}

pub fn parse_operand(op_str: &str) -> Result<usize, String> {
    let operand = match op_str {
        "x0"  | "zero"        => 0,
        "x1"  | "ra"          => 1,
        "x2"  | "sp"          => 2,
        "x3"  | "gp"          => 3,
        "x4"  | "tp"          => 4,
        "x5"  | "t0"          => 5,
        "x6"  | "t1"          => 6,
        "x7"  | "t2"          => 7,
        "x8"  | "s0"   | "fp" => 8,
        "x9"  | "s1"          => 9,
        "x10" | "a0"          => 10,
        "x11" | "a1"          => 11,
        "x12" | "a2"          => 12,
        "x13" | "a3"          => 13,
        "x14" | "a4"          => 14,
        "x15" | "a5"          => 15,
        "x16" | "a6"          => 16,
        "x17" | "a7"          => 17,
        "x18" | "s2"          => 18,
        "x19" | "s3"          => 19,
        "x20" | "s4"          => 20,
        "x21" | "s5"          => 21,
        "x22" | "s6"          => 22,
        "x23" | "s7"          => 23,
        "x24" | "s8"          => 24,
        "x25" | "s9"          => 25,
        "x26" | "s10"         => 26,
        "x27" | "s11"         => 27,
        "x28" | "t3"          => 28,
        "x29" | "t4"          => 29,
        "x30" | "t5"          => 30,
        "x31" | "t6"          => 31,
        _                     => return Err(format!("Incorrect integer operand: {}", op_str))   
    };

    Ok(operand)
}

pub fn parse_immediate(imm_str: &str) -> Result<u64, String> {
    if imm_str.starts_with("0x") || imm_str.starts_with("0X") {
        u64::from_str_radix(&imm_str[2..], 16).map_err(|e| format!("Error parsing immediate: {}", e))
    } else {
        imm_str.parse::<u64>().map_err(|e| format!("Error parsing immediate: {}", e))
    }
}