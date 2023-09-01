use eeric::prelude::*;

use super::{integer, float};

fn construct_vtype((sew, lmul, tail, mask): (SEW, LMUL, MaskBehavior, MaskBehavior)) -> Result<u32, String> {
    let vsew = match sew {
        SEW::E8 => 0b000,
        SEW::E16 => 0b001,
        SEW::E32 => 0b010,
        SEW::E64 => 0b011,
        SEW::E128 => return Err("Cannot construct vtype with SEW=128b".to_owned())
    };

    let vlmul = match lmul {
        LMUL::MF8 => 0b101,
        LMUL::MF4 => 0b110,
        LMUL::MF2 => 0b111,
        LMUL::M1 => 0b000,
        LMUL::M2 => 0b001,
        LMUL::M4 => 0b010,
        LMUL::M8 => 0b011
    };

    use MaskBehavior as MB;

    let (vta, vma) = match (tail, mask) {
        (MB::Undisturbed, MB::Undisturbed) => (0, 0),
        (MB::Undisturbed, MB::Agnostic) => (0, 1),
        (MB::Agnostic, MB::Undisturbed) => (1, 0),
        (MB::Agnostic, MB::Agnostic) => (1, 1),
    };

    let result = (vma << 7) | (vta << 6) | (vsew << 3) | vlmul;

    Ok(result)
}

fn parse_vtype(vtype: &[&str]) -> Result<(SEW, LMUL, MaskBehavior, MaskBehavior), String> {
    let sew = match vtype[0] {
        "e8" => SEW::E8,
        "e16" => SEW::E16,
        "e32" => SEW::E32,
        "e64" => SEW::E64,
        other => return Err(format!("Unknown SEW value: {}", other)),
    };

    let lmul = match vtype[1] {
        "mf8" => LMUL::MF8,
        "mf4" => LMUL::MF4,
        "mf2" => LMUL::MF2,
        "m1" => LMUL::M1,
        "m2" => LMUL::M2,
        "m4" => LMUL::M4,
        "m8" => LMUL::M8,
        other => return Err(format!("Unknown LMUL value: {}", other)),
    };

    let tail = match vtype[2] {
        "ta" => MaskBehavior::Agnostic,
        "tu" => MaskBehavior::Undisturbed,
        other => return Err(format!("Unknown tail value: {}", other)),
    };

    let mask = match vtype[3] {
        "ma" => MaskBehavior::Agnostic,
        "mu" => MaskBehavior::Undisturbed,
        other => return Err(format!("Unknown mask value: {}", other)),
    };

    Ok((sew, lmul, tail, mask))
}

pub fn parse_vsetvli_format(vsetvli: &str) -> Result<format::Vsetvli, String> {
    let tokens: Vec<&str> = vsetvli.split(", ").collect();
    if tokens.len() != 6 {
        return Err(format!("Expected format: 'rd, rs1, SEW, LMUL, ta/tu, ma/mu', got {} instead", vsetvli));
    }

    let rd = integer::parse_operand(tokens[0])?;
    let rs1 = integer::parse_operand(tokens[1])?;
    let vtype = parse_vtype(&tokens[2..])?;
    

    Ok(format::Vsetvli { rd, rs1, vtypei: construct_vtype(vtype)? })
}

pub fn parse_vsetivli_format(vsetivli: &str) -> Result<format::Vsetivli, String> {
    let tokens: Vec<&str> = vsetivli.split(", ").collect();
    if tokens.len() != 6 {
        return Err(format!("Expected format: 'rd, uimm5, SEW, LMUL, ta/tu, ma/mu', got {} instead", vsetivli));
    }

    let rd = integer::parse_operand(tokens[0])?;
    let uimm = integer::parse_immediate(tokens[1])? as u32;
    
    let vtype = parse_vtype(&tokens[2..])?;

    Ok(format::Vsetivli { rd, uimm, vtypei: construct_vtype(vtype)? })
}

pub fn parse_vsetvl_format(vsetvl: &str) -> Result<format::Vsetvl, String> {
    let tokens: Vec<&str> = vsetvl.split(", ").collect();
    if tokens.len() != 3 {
        return Err(format!("Expected format: 'rd, rs1, rs2', got {} instead", vsetvl));
    }

    let rd = integer::parse_operand(tokens[0])?;
    let rs1 = integer::parse_operand(tokens[1])?;
    let rs2 = integer::parse_operand(tokens[2])?;

    Ok(format::Vsetvl { rd, rs1, rs2 })
}

pub fn parse_vl_format(vl: &str) -> Result<format::Vl, String> {
    let tokens: Vec<&str> = vl.split(", ").collect();
    if tokens.len() != 2 && tokens.len() != 3 {
        return Err(format!("Expected format: 'vd, (rs1), [vm]', got {} instead", vl));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(&tokens[1])?;
    let vm = tokens.len() == 3 && parse_operand(&tokens[2])?.as_mask()? == ();

    Ok(format::Vl { vd, rs1, vm })
}

pub fn parse_vlm_format(vlm: &str) -> Result<format::Vl, String> {
    let tokens: Vec<&str> = vlm.split(", ").collect();
    if tokens.len() != 2 {
        return Err(format!("Expected format: 'vd, (rs1)', got {} instead", vlm));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(&tokens[1])?;

    Ok(format::Vl { vd, rs1, vm: false })
}

pub fn parse_vls_format(vls: &str) -> Result<format::Vls, String> {
    let tokens: Vec<&str> = vls.split(", ").collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!("Expected format: 'vd, (rs1), rs2, [vm]', got {} instead", vls));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(&tokens[1])?;
    let rs2 = integer::parse_operand(tokens[2])?;
    let vm = tokens.len() == 4 && parse_operand(tokens[3])?.as_mask()? == ();

    Ok(format::Vls { vd, rs1, rs2, vm })
}

pub fn parse_vlx_format(vlx: &str) -> Result<format::Vlx, String> {
    let tokens: Vec<&str> = vlx.split(", ").collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!("Expected format: 'vd, (rs1), vs2, [vm]', got {} instead", vlx));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(&tokens[1])?;
    let vs2 = parse_operand(tokens[2])?.as_register()?;
    let vm = tokens.len() == 4 && parse_operand(tokens[3])?.as_mask()? == ();

    Ok(format::Vlx { vd, rs1, vs2, vm })
}

pub fn parse_vlr_format(vlr: &str) -> Result<format::Vlr, String> {
    let tokens: Vec<&str> = vlr.split(", ").collect();
    if tokens.len() != 2 {
        return Err(format!("Expected format: 'vd, (rs1)', got {} instead", vlr));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(&tokens[1])?;

    Ok(format::Vlr { vd, rs1 })
}

pub fn parse_vs_format(vs: &str) -> Result<format::Vs, String> {
    let tokens: Vec<&str> = vs.split(", ").collect();
    if tokens.len() != 2 && tokens.len() != 3 {
        return Err(format!("Expected format: 'vs3, (rs1), [vm]', got {} instead", vs));
    }

    let vs3 = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(&tokens[1])?;
    let vm = tokens.len() == 3 && parse_operand(&tokens[2])?.as_mask()? == ();

    Ok(format::Vs { vs3, rs1, vm })
}

pub fn parse_vsm_format(vsm: &str) -> Result<format::Vs, String> {
    let tokens: Vec<&str> = vsm.split(", ").collect();
    if tokens.len() != 2 {
        return Err(format!("Expected format: 'vs3, (rs1)', got {} instead", vsm));
    }

    let vs3 = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(&tokens[1])?;

    Ok(format::Vs { vs3, rs1, vm: false })
}

pub fn parse_vss_format(vss: &str) -> Result<format::Vss, String> {
    let tokens: Vec<&str> = vss.split(", ").collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!("Expected format: 'vs3, (rs1), rs2, [vm]', got {} instead", vss));
    }

    let vs3 = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(&tokens[1])?;
    let rs2 = integer::parse_operand(&tokens[2])?;
    let vm = tokens.len() == 4 && parse_operand(&tokens[3])?.as_mask()? == ();

    Ok(format::Vss { vs3, rs1, rs2, vm })
}

pub fn parse_vsx_format(vsx: &str) -> Result<format::Vsx, String> {
    let tokens: Vec<&str> = vsx.split(", ").collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!("Expected format: 'vs3, (rs1), vs2, [vm]', got {} instead", vsx));
    }

    let vs3 = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(&tokens[1])?;
    let vs2 = parse_operand(&tokens[2])?.as_register()?;
    let vm = tokens.len() == 4 && parse_operand(&tokens[3])?.as_mask()? == ();

    Ok(format::Vsx { vs3, rs1, vs2, vm })
}

pub fn parse_vsr_format(vsr: &str) -> Result<format::Vsr, String> {
    let tokens: Vec<&str> = vsr.split(", ").collect();
    if tokens.len() != 2 {
        return Err(format!("Expected format: 'vs3, (rs1)', got {} instead", vsr));
    }

    let vs3 = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_addr_operand(&tokens[1])?;

    Ok(format::Vsr { vs3, rs1 })
}

pub fn parse_opivv_format(opivv: &str) -> Result<format::Opivv, String> {
    let tokens: Vec<&str> = opivv.split(", ").collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!("Expected format: 'vd, vs2, vs1, [vm]', got {} instead", opivv));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vs1 = parse_operand(tokens[2])?.as_register()?;
    let vm = tokens.len() == 4 && parse_operand(tokens[3])?.as_mask()? == ();

    Ok(format::Opivv { vd, vs2, vs1, vm })
}

pub fn parse_opivx_format(opivx: &str) -> Result<format::Opivx, String> {
    let tokens: Vec<&str> = opivx.split(", ").collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!("Expected format: 'vd, vs2, rs1, [vm]', got {} instead", opivx));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let rs1 = integer::parse_operand(tokens[2])?;
    let vm = tokens.len() == 4 && parse_operand(tokens[3])?.as_mask()? == ();

    Ok(format::Opivx { vd, vs2, rs1, vm })
}

pub fn parse_opivi_format(opivi: &str) -> Result<format::Opivi, String> {
    let tokens: Vec<&str> = opivi.split(", ").collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!("Expected format: 'vd, vs2, rs1, [vm]', got {} instead", opivi));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let imm = integer::parse_immediate(tokens[2])?;
    let vm = tokens.len() == 4 && parse_operand(tokens[3])?.as_mask()? == ();

    Ok(format::Opivi { vd, vs2, imm5: imm, vm })
}

pub fn parse_opmvv_format(opmvv: &str) -> Result<format::Opmvv, String> {
    let tokens: Vec<&str> = opmvv.split(", ").collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!("Expected format: 'vd, vs2, vs1, [vm]', got {} instead", opmvv));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vs1 = parse_operand(tokens[2])?.as_register()?;
    let vm = tokens.len() == 4 && parse_operand(tokens[3])?.as_mask()? == ();

    Ok(format::Opmvv { dest: vd, vs2, vs1, vm })
}

pub fn parse_opmvx_format(opmvx: &str) -> Result<format::Opmvx, String> {
    let tokens: Vec<&str> = opmvx.split(", ").collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!("Expected format: 'vd, vs2, rs1, [vm]', got {} instead", opmvx));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let rs1 = integer::parse_operand(tokens[2])?;
    let vm = tokens.len() == 4 && parse_operand(tokens[3])?.as_mask()? == ();

    Ok(format::Opmvx { dest: vd, vs2, rs1, vm })
}

pub fn parse_vwxunary0_vmvxs_format(vwxunary0: &str) -> Result<format::Vwxunary0, String> {
    let tokens: Vec<&str> = vwxunary0.split(", ").collect();
    if tokens.len() != 2 {
        return Err(format!("Expected format: 'rd, vs2', got {} instead", vwxunary0));
    }

    let rd = integer::parse_operand(tokens[0])?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;

    Ok(format::Vwxunary0 { dest: rd, vs2, vs1: 0, vm: false })
}

pub fn parse_vwxunary0_format(vwxunary0: &str) -> Result<format::Vwxunary0, String> {
    let tokens: Vec<&str> = vwxunary0.split(", ").collect();
    if tokens.len() != 2 && tokens.len() != 3 {
        return Err(format!("Expected format: 'rd, vs2, [vm]', got {} instead", vwxunary0));
    }

    let rd = integer::parse_operand(tokens[0])?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vm = tokens.len() == 3 && parse_operand(tokens[2])?.as_mask()? == ();

    Ok(format::Vwxunary0 { dest: rd, vs2, vs1: 0, vm })
}

pub fn parse_vrxunary0_format(vrxunary0: &str) -> Result<format::Vrxunary0, String> {
    let tokens: Vec<&str> = vrxunary0.split(", ").collect();
    if tokens.len() != 2 {
        return Err(format!("Expected format: 'vd, rs1', got {} instead", vrxunary0));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let rs1 = integer::parse_operand(tokens[1])?;

    Ok(format::Vrxunary0 { dest: vd, vs2: 0, rs1, vm: false })
}

pub fn parse_vxunary0_format(vxunary0: &str) -> Result<format::Vxunary0, String> {
    let tokens: Vec<&str> = vxunary0.split(", ").collect();
    if tokens.len() != 2 && tokens.len() != 3 {
        return Err(format!("Expected format: 'vd, vs2, [vm]', got {} instead", vxunary0));
    }

    let rd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vm = tokens.len() == 3 && parse_operand(tokens[2])?.as_mask()? == ();

    Ok(format::Vxunary0 { dest: rd, vs2, vs1: 0, vm })
}

pub fn parse_vmunary0_vidv_format(vmunary0: &str) -> Result<format::Vmunary0, String> {
    let tokens: Vec<&str> = vmunary0.split(", ").collect();
    if tokens.len() != 1 && tokens.len() != 2 {
        return Err(format!("Expected format: 'vd, [vm]', got {} instead", vmunary0));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vm = tokens.len() == 2 && parse_operand(tokens[1])?.as_mask()? == ();

    Ok(format::Vmunary0 { dest: vd, vs2: 0, vs1: 0, vm })
}

pub fn parse_vmunary0_format(vmunary0: &str) -> Result<format::Vmunary0, String> {
    let tokens: Vec<&str> = vmunary0.split(", ").collect();
    if tokens.len() != 2 && tokens.len() != 3 {
        return Err(format!("Expected format: 'vd, vs2, [vm]', got {} instead", vmunary0));
    }

    let rd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vm = tokens.len() == 3 && parse_operand(tokens[2])?.as_mask()? == ();

    Ok(format::Vmunary0 { dest: rd, vs2, vs1: 0, vm })
}

pub fn parse_opfvv_format(opfvv: &str) -> Result<format::Opfvv, String> {
    let tokens: Vec<&str> = opfvv.split(", ").collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!("Expected format: 'vd, vs2, vs1, [vm]', got {} instead", opfvv));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vs1 = parse_operand(tokens[2])?.as_register()?;
    let vm = tokens.len() == 4 && parse_operand(tokens[3])?.as_mask()? == ();

    Ok(format::Opfvv { dest: vd, vs2, vs1, vm })
}

pub fn parse_opfvf_format(opfvf: &str) -> Result<format::Opfvf, String> {
    let tokens: Vec<&str> = opfvf.split(", ").collect();
    if tokens.len() != 3 && tokens.len() != 4 {
        return Err(format!("Expected format: 'vd, vs2, rs1, [vm]', got {} instead", opfvf));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let rs1 = float::parse_operand(tokens[2])?;
    let vm = tokens.len() == 4 && parse_operand(tokens[3])?.as_mask()? == ();

    Ok(format::Opfvf { vd, vs2, rs1, vm })
}

pub fn parse_vwfunary0_format(vwfunary0: &str) -> Result<format::Vwfunary0, String> {
    let tokens: Vec<&str> = vwfunary0.split(", ").collect();
    if tokens.len() != 2 {
        return Err(format!("Expected format: 'rd, vs2', got {} instead", vwfunary0));
    }

    let rd = float::parse_operand(tokens[0])?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;

    Ok(format::Vwfunary0 { dest: rd, vs2, vs1: 0, vm: false })
}

pub fn parse_vrfunary0_format(vrfunary0: &str) -> Result<format::Vrfunary0, String> {
    let tokens: Vec<&str> = vrfunary0.split(", ").collect();
    if tokens.len() != 2 {
        return Err(format!("Expected format: 'vd, rs1', got {} instead", vrfunary0));
    }

    let vd = parse_operand(tokens[0])?.as_register()?;
    let rs1 = float::parse_operand(tokens[1])?;

    Ok(format::Vrfunary0 { vd, vs2: 0, rs1, vm: false })
}

pub fn parse_vfunary0_format(vfunary0: &str) -> Result<format::Vfunary0, String> {
    let tokens: Vec<&str> = vfunary0.split(", ").collect();
    if tokens.len() != 2 && tokens.len() != 3 {
        return Err(format!("Expected format: 'vd, vs2, [vm]', got {} instead", vfunary0));
    }

    let rd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vm = tokens.len() == 3 && parse_operand(tokens[2])?.as_mask()? == ();

    Ok(format::Vfunary0 { dest: rd, vs2, vs1: 0, vm })
}

pub fn parse_vfunary1_format(vfunary1: &str) -> Result<format::Vfunary1, String> {
    let tokens: Vec<&str> = vfunary1.split(", ").collect();
    if tokens.len() != 2 && tokens.len() != 3 {
        return Err(format!("Expected format: 'vd, vs2, [vm]', got {} instead", vfunary1));
    }

    let rd = parse_operand(tokens[0])?.as_register()?;
    let vs2 = parse_operand(tokens[1])?.as_register()?;
    let vm = tokens.len() == 3 && parse_operand(tokens[2])?.as_mask()? == ();

    Ok(format::Vfunary1 { dest: rd, vs2, vs1: 0, vm })
}

#[derive(PartialEq)]
pub enum VectorOperand {
    Register(usize),
    Mask
}

impl VectorOperand {
    fn as_register(self) -> Result<usize, String> {
        match self {
            Self::Register(nth) => Ok(nth),
            Self::Mask => Err("Expected register, parsed mask instead".to_owned())
        }
    }

    fn as_mask(self) -> Result<(), String> {
        match self {
            Self::Register(nth) => Err(format!("Expected mask, parsed vector register {} instead", nth)),
            Self::Mask => Ok(())
        }
    }
}

fn parse_operand(op: &str) -> Result<VectorOperand, String> {
    let operand = match op {
        "v0"   => 0,
        "v1"   => 1,
        "v2"   => 2,
        "v3"   => 3,
        "v4"   => 4,
        "v5"   => 5,
        "v6"   => 6,
        "v7"   => 7,
        "v8"   => 8,
        "v9"   => 9,
        "v10"  => 10,
        "v11"  => 11,
        "v12"  => 12,
        "v13"  => 13,
        "v14"  => 14,
        "v15"  => 15,
        "v16"  => 16,
        "v17"  => 17,
        "v18"  => 18,
        "v19"  => 19,
        "v20"  => 20,
        "v21"  => 21,
        "v22"  => 22,
        "v23"  => 23,
        "v24"  => 24,
        "v25"  => 25,
        "v26"  => 26,
        "v27"  => 27,
        "v28"  => 28,
        "v29"  => 29,
        "v30"  => 30,
        "v31"  => 31,
        "v0.t" => return Ok(VectorOperand::Mask),
        _      => return Err(format!("Unknown vector operand {}", op))   
    };

    Ok(VectorOperand::Register(operand))
}