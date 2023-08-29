use eeric::prelude::*;

pub fn parse_vsetvli_format(vsetvli: &str) -> Result<format::Vsetvli, String> {
    todo!()
}

pub fn parse_vsetivli_format(vsetivli: &str) -> Result<format::Vsetivli, String> {
    todo!()
}

pub fn parse_vsetvl_format(vsetvl: &str) -> Result<format::Vsetvl, String> {
    todo!()
}

pub fn parse_vl_format(vl: &str) -> Result<format::Vl, String> {
    todo!()
}

pub fn parse_vls_format(vls: &str) -> Result<format::Vls, String> {
    todo!()
}

pub fn parse_vlx_format(vlx: &str) -> Result<format::Vlx, String> {
    todo!()
}

pub fn parse_vlr_format(vlr: &str) -> Result<format::Vlr, String> {
    todo!()
}

pub fn parse_vs_format(vs: &str) -> Result<format::Vs, String> {
    todo!()
}

pub fn parse_vss_format(vss: &str) -> Result<format::Vss, String> {
    todo!()
}

pub fn parse_vsx_format(vsx: &str) -> Result<format::Vsx, String> {
    todo!()
}

pub fn parse_vsr_format(vsr: &str) -> Result<format::Vsr, String> {
    todo!()
}

pub fn parse_opivv_format(opivv: &str) -> Result<format::Opivv, String> {
    todo!()
}

pub fn parse_opivx_format(opivx: &str) -> Result<format::Opivx, String> {
    todo!()
}

pub fn parse_opivi_format(opivi: &str) -> Result<format::Opivi, String> {
    todo!()
}

pub fn parse_opmvv_format(opmvv: &str) -> Result<format::Opmvv, String> {
    todo!()
}

pub fn parse_opmvx_format(opmvx: &str) -> Result<format::Opmvx, String> {
    todo!()
}

pub fn parse_vwxunary0_format(vwxunary0: &str) -> Result<format::Vwxunary0, String> {
    todo!()
}

pub fn parse_vrxunary0_format(vrxunary0: &str) -> Result<format::Vrxunary0, String> {
    todo!()
}

pub fn parse_vxunary0_format(vxunary0: &str) -> Result<format::Vxunary0, String> {
    todo!()
}

pub fn parse_vmunary0_format(vmunary0: &str) -> Result<format::Vmunary0, String> {
    todo!()
}

pub fn parse_opfvv_format(opfvv: &str) -> Result<format::Opfvv, String> {
    todo!()
}

pub fn parse_opfvf_format(opfvf: &str) -> Result<format::Opfvf, String> {
    todo!()
}

pub fn parse_vwfunary0_format(vwfunary0: &str) -> Result<format::Vwfunary0, String> {
    todo!()
}

pub fn parse_vrfunary0_format(vrfunary0: &str) -> Result<format::Vrfunary0, String> {
    todo!()
}

pub fn parse_vfunary0_format(vfunary0: &str) -> Result<format::Vfunary0, String> {
    todo!()
}

pub fn parse_vfunary1_format(vfunary1: &str) -> Result<format::Vfunary1, String> {
    todo!()
}

pub enum VectorOperand {
    Register(usize),
    Mask
}

pub fn parse_vector_operand(op: String) -> Result<VectorOperand, String> {
    use VectorOperand::*;
    
    let operand = match op.as_str() {
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
        "v0.t" => return Ok(Mask),
        _      => return Err(op)   
    };

    Ok(Register(operand))
}