pub struct IntegerParser;

impl IntegerParser {
    pub fn parse_r_format(r: &str) -> Result<eeric::Format::R, String> {
        let tokens: Vec<&str> = r.split(", ").collect();

        if tokens.len() != 3 {
            return Err("Expected format: 'rd, rs1, rs2'".to_owned());
        }

        let rd = tokens[0];
        let rs1 = tokens[1];
        let rs2 = tokens[2];

        let rd = Self::parse_operand(rd)?;
        let rs1 = Self::parse_operand(rs1)?;
        let rs2 = Self::parse_operand(rs2)?;

        Ok(eeric::Format::R { rd, rs1, rs2 })
    }

    pub fn parse_i_format(i: &str) -> Result<eeric::Format::I, String> {
        let tokens: Vec<&str> = i.split(", ").collect();

        if tokens.len() != 3 {
            return Err("Expected format: 'rd, rs1, imm'".to_owned());
        }

        let rd = tokens[0];
        let rs1 = tokens[1];
        let imm = tokens[2];

        let rd = Self::parse_operand(rd)?;
        let rs1 = Self::parse_operand(rs1)?;
        let imm = Self::parse_immediate(imm)?;

        Ok(eeric::Format::I { rd, rs1, imm12: imm })
    }

    pub fn parse_s_format(s: &str) -> Result<eeric::Format::S, String> {
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

        let rs1 = Self::parse_operand(rs1)?;
        let rs2 = Self::parse_operand(rs2)?;
        let imm = Self::parse_immediate(imm)?;

        Ok(eeric::Format::S { rs1, rs2, imm12: imm })
    }

    pub fn parse_u_format(u: &str) -> Result<eeric::Format::U, String> {
        let tokens: Vec<&str> = u.split(", ").collect();

        if tokens.len() != 2 {
            return Err("Expected format: 'rd, imm'".to_owned());
        }

        let rd = tokens[0];
        let imm = tokens[1];

        let rd = Self::parse_operand(rd)?;
        let imm = Self::parse_immediate(imm)?;

        Ok(eeric::Format::U { rd, imm20: imm })
    }

    pub(self) fn parse_operand(op_str: &str) -> Result<usize, String> {
        match op_str {
            "x0"  | "zero"        => Ok(0),
            "x1"  | "ra"          => Ok(1),
            "x2"  | "sp"          => Ok(2),
            "x3"  | "gp"          => Ok(3),
            "x4"  | "tp"          => Ok(4),
            "x5"  | "t0"          => Ok(5),
            "x6"  | "t1"          => Ok(6),
            "x7"  | "t2"          => Ok(7),
            "x8"  | "s0"   | "fp" => Ok(8),
            "x9"  | "s1"          => Ok(9),
            "x10" | "a0"          => Ok(10),
            "x11" | "a1"          => Ok(11),
            "x12" | "a2"          => Ok(12),
            "x13" | "a3"          => Ok(13),
            "x14" | "a4"          => Ok(14),
            "x15" | "a5"          => Ok(15),
            "x16" | "a6"          => Ok(16),
            "x17" | "a7"          => Ok(17),
            "x18" | "s2"          => Ok(18),
            "x19" | "s3"          => Ok(19),
            "x20" | "s4"          => Ok(20),
            "x21" | "s5"          => Ok(21),
            "x22" | "s6"          => Ok(22),
            "x23" | "s7"          => Ok(23),
            "x24" | "s8"          => Ok(24),
            "x25" | "s9"          => Ok(25),
            "x26" | "s10"         => Ok(26),
            "x27" | "s11"         => Ok(27),
            "x28" | "t3"          => Ok(28),
            "x29" | "t4"          => Ok(29),
            "x30" | "t5"          => Ok(30),
            "x31" | "t6"          => Ok(31),
            _                     => Err(format!("Incorrect integer operand: {}", op_str))   
        }
    }

    pub(self) fn parse_immediate(imm_str: &str) -> Result<u64, String> {
        if imm_str.starts_with("0x") || imm_str.starts_with("0X") {
            u64::from_str_radix(&imm_str[2..], 16).map_err(|e| format!("Error parsing immediate: {}", e))
        } else {
            imm_str.parse::<u64>().map_err(|e| format!("Error parsing immediate: {}", e))
        }
    }
}

pub struct CsrParser;

impl CsrParser {
    pub fn parse_csrr_format(csrr: &str) -> Result<eeric::Format::Csrr, String> {
        todo!()
    }

    pub fn parse_csri_format(csri: &str) -> Result<eeric::Format::Csri, String> {
        todo!()
    }
}

pub struct FloatParser;

impl FloatParser {
    pub fn parse_r4_format(r: &str) -> Result<eeric::Format::R4, String> {
        let tokens: Vec<&str> = r.split(", ").collect();

        if tokens.len() != 4 {
            return Err("Expected format: 'rd, rs1, rs2, rs3'".to_owned());
        }

        let rd = tokens[0];
        let rs1 = tokens[1];
        let rs2 = tokens[2];
        let rs3 = tokens[3];

        let rd = Self::parse_operand(rd)?;
        let rs1 = Self::parse_operand(rs1)?;
        let rs2 = Self::parse_operand(rs2)?;
        let rs3 = Self::parse_operand(rs3)?;

        Ok(eeric::Format::R4 { rd, rs1, rs2, rs3 })
    }

    pub(self) fn parse_operand(op_str: &str) -> Result<usize, String> {
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
}

pub enum VectorOperand {
    Register(usize),
    Mask
}

pub struct VectorParser;

impl VectorParser {
    pub fn parse_vsetvli_format(vsetvli: &str) -> Result<eeric::Format::Vsetvli, String> {
        todo!()
    }

    pub fn parse_vsetivli_format(vsetivli: &str) -> Result<eeric::Format::Vsetivli, String> {
        todo!()
    }

    pub fn parse_vsetvl_format(vsetvl: &str) -> Result<eeric::Format::Vsetvl, String> {
        todo!()
    }

    pub fn parse_vl_format(vl: &str) -> Result<eeric::Format::Vl, String> {
        todo!()
    }

    pub fn parse_vls_format(vls: &str) -> Result<eeric::Format::Vls, String> {
        todo!()
    }

    pub fn parse_vlx_format(vlx: &str) -> Result<eeric::Format::Vlx, String> {
        todo!()
    }

    pub fn parse_vlr_format(vlr: &str) -> Result<eeric::Format::Vlr, String> {
        todo!()
    }

    pub fn parse_vs_format(vs: &str) -> Result<eeric::Format::Vs, String> {
        todo!()
    }

    pub fn parse_vss_format(vss: &str) -> Result<eeric::Format::Vss, String> {
        todo!()
    }

    pub fn parse_vsx_format(vsx: &str) -> Result<eeric::Format::Vsx, String> {
        todo!()
    }

    pub fn parse_vsr_format(vsr: &str) -> Result<eeric::Format::Vsr, String> {
        todo!()
    }

    pub fn parse_opivv_format(opivv: &str) -> Result<eeric::Format::Opivv, String> {
        todo!()
    }

    pub fn parse_opivx_format(opivx: &str) -> Result<eeric::Format::Opivx, String> {
        todo!()
    }

    pub fn parse_opivi_format(opivi: &str) -> Result<eeric::Format::Opivi, String> {
        todo!()
    }

    pub fn parse_opmvv_format(opmvv: &str) -> Result<eeric::Format::Opmvv, String> {
        todo!()
    }

    pub fn parse_opmvx_format(opmvx: &str) -> Result<eeric::Format::Opmvx, String> {
        todo!()
    }

    pub fn parse_vwxunary0_format(vwxunary0: &str) -> Result<eeric::Format::Vwxunary0, String> {
        todo!()
    }

    pub fn parse_vrxunary0_format(vrxunary0: &str) -> Result<eeric::Format::Vrxunary0, String> {
        todo!()
    }

    pub fn parse_vxunary0_format(vxunary0: &str) -> Result<eeric::Format::Vxunary0, String> {
        todo!()
    }

    pub fn parse_vmunary0_format(vmunary0: &str) -> Result<eeric::Format::Vmunary0, String> {
        todo!()
    }

    pub fn parse_opfvv_format(opfvv: &str) -> Result<eeric::Format::Opfvv, String> {
        todo!()
    }

    pub fn parse_opfvf_format(opfvf: &str) -> Result<eeric::Format::Opfvf, String> {
        todo!()
    }

    pub fn parse_vwfunary0_format(vwfunary0: &str) -> Result<eeric::Format::Vwfunary0, String> {
        todo!()
    }

    pub fn parse_vrfunary0_format(vrfunary0: &str) -> Result<eeric::Format::Vrfunary0, String> {
        todo!()
    }

    pub fn parse_vfunary0_format(vfunary0: &str) -> Result<eeric::Format::Vfunary0, String> {
        todo!()
    }

    pub fn parse_vfunary1_format(vfunary1: &str) -> Result<eeric::Format::Vfunary1, String> {
        todo!()
    }

    pub fn parse_vector_operand(op: String) -> Result<VectorOperand, String> {
        use VectorOperand::*;
        
        match op.as_str() {
            "v0.t" => Ok(Mask),
            "v0"   => Ok(Register(0)),
            "v1"   => Ok(Register(1)),
            "v2"   => Ok(Register(2)),
            "v3"   => Ok(Register(3)),
            "v4"   => Ok(Register(4)),
            "v5"   => Ok(Register(5)),
            "v6"   => Ok(Register(6)),
            "v7"   => Ok(Register(7)),
            "v8"   => Ok(Register(8)),
            "v9"   => Ok(Register(9)),
            "v10"  => Ok(Register(10)),
            "v11"  => Ok(Register(11)),
            "v12"  => Ok(Register(12)),
            "v13"  => Ok(Register(13)),
            "v14"  => Ok(Register(14)),
            "v15"  => Ok(Register(15)),
            "v16"  => Ok(Register(16)),
            "v17"  => Ok(Register(17)),
            "v18"  => Ok(Register(18)),
            "v19"  => Ok(Register(19)),
            "v20"  => Ok(Register(20)),
            "v21"  => Ok(Register(21)),
            "v22"  => Ok(Register(22)),
            "v23"  => Ok(Register(23)),
            "v24"  => Ok(Register(24)),
            "v25"  => Ok(Register(25)),
            "v26"  => Ok(Register(26)),
            "v27"  => Ok(Register(27)),
            "v28"  => Ok(Register(28)),
            "v29"  => Ok(Register(29)),
            "v30"  => Ok(Register(30)),
            "v31"  => Ok(Register(31)),
            _              => Err(op)   
        }
    }
}