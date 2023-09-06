use eeric::prelude::*;

use super::integer;

pub fn parse_csrr_format(csrr: &str) -> Result<format::Csrr, String> {
    let tokens: Vec<&str> = csrr.split(", ").collect();

    if tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'rd, csr, rs1', got {} instead",
            csrr
        ));
    }

    let rd = tokens[0];
    let csr = tokens[1];
    let rs1 = tokens[1];

    let rd = integer::parse_operand(rd)?;
    let csr = parse_operand(csr)?;
    let rs1 = integer::parse_operand(rs1)?;

    Ok(format::Csrr { rd, csr, rs1 })
}

pub fn parse_csri_format(csri: &str) -> Result<format::Csri, String> {
    let tokens: Vec<&str> = csri.split(", ").collect();

    if tokens.len() != 3 {
        return Err(format!(
            "Expected format: 'rd, csr, imm', got {} instead",
            csri
        ));
    }

    let rd = tokens[0];
    let csr = tokens[1];
    let imm = tokens[2];

    let rd = integer::parse_operand(rd)?;
    let csr = parse_operand(csr)?;
    let uimm = integer::parse_immediate(imm)?;

    Ok(format::Csri {
        rd,
        csr,
        uimm: uimm as u32 as usize,
    })
}

fn parse_operand(op: &str) -> Result<usize, String> {
    let operand = match op {
        "instret" => alias::INSTRET,
        "instreth" => alias::INSTRETH,
        "cycle" => alias::CYCLE,
        "cycleh" => alias::CYCLEH,
        "time" => alias::TIME,
        "timeh" => alias::TIMEH,
        "marchid" => alias::MARCHID,
        "fcsr" => alias::FCSR,
        "fflags" => alias::FFLAGS,
        "frm" => alias::FRM,
        "mstatus" => alias::MSTATUS,
        "vsstatus" => alias::VSSTATUS,
        "vtype" => alias::VTYPE,
        "vl" => alias::VL,
        "vlenb" => alias::VLENB,
        "vstart" => alias::VSTART,
        "vxrm" => alias::VXRM,
        "vxsat" => alias::VXSAT,
        "vcsr" => alias::VCSR,
        _ => return Err(format!("Incorrect or unsupported CSR operand: {}", op)),
    };

    Ok(operand)
}

pub mod pseudo {
    pub fn parse_op_csr_format(op_csr: &str) -> Result<(usize, usize), String> {
        let tokens: Vec<&str> = op_csr.split(", ").collect();

        if tokens.len() != 2 {
            return Err(format!(
                "Expected format: 'xreg, csr', got {} instead",
                op_csr
            ));
        }

        let reg = tokens[0];
        let csr = tokens[1];

        let reg = super::integer::parse_operand(reg)?;
        let csr = super::parse_operand(csr)?;

        Ok((reg, csr))
    }

    pub fn parse_csr_op_format(csr_op: &str) -> Result<(usize, usize), String> {
        let tokens: Vec<&str> = csr_op.split(", ").collect();

        if tokens.len() != 2 {
            return Err(format!(
                "Expected format: 'csr, xreg', got {} instead",
                csr_op
            ));
        }

        let csr = tokens[0];
        let reg = tokens[1];

        let csr = super::parse_operand(csr)?;
        let reg = super::integer::parse_operand(reg)?;

        Ok((csr, reg))
    }
}
