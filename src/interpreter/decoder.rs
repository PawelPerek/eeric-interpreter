mod operand;

use std::{cmp::Ordering, collections::HashMap};

use eeric::prelude::{format::*, *};
use operand::{csr, float, integer, vector};
use Instruction::*;

pub struct Decoder;

pub enum LineClassification {
    Instruction(String),
    Label(String),
    Empty,
}

impl Decoder {
    pub fn classify(line: &str) -> LineClassification {
        let trimmed_line = line.split('#').next().unwrap_or("").trim();

        if trimmed_line.is_empty() {
            LineClassification::Empty
        } else if let Some(label) = trimmed_line.strip_suffix(':') {
            LineClassification::Label(label.to_string())
        } else {
            LineClassification::Instruction(trimmed_line.to_string())
        }
    }

    pub fn decode(
        instruction_line: &str,
        labels: &HashMap<String, usize>,
        current_address: usize,
    ) -> Result<Instruction, String> {
        let (mnemonic, operands) = Self::split_instruction(instruction_line);

        use integer::{
            parse_branch_format as b, parse_i_format as i, parse_load_format as l,
            parse_r_format as r, parse_s_format as s, parse_u_format as u,
        };

        use csr::{parse_csri_format as csri, parse_csrr_format as csrr};

        use float::parse_r4_format as r4;

        use vector::{
            parse_opfvf_fma_format as opfvf_fma, parse_opfvf_format as opfvf,
            parse_opfvv_fma_format as opfvv_fma, parse_opfvv_format as opfvv,
            parse_opivi_format as opivi, parse_opivi_vmv_format as opivi_vmv, parse_opivi_v0_format as opivi_v0, parse_opivi_maskless_format as opivi_maskless,
            parse_opivv_format as opivv, parse_opivv_vmv_format as opivv_vmv, parse_opivv_v0_format as opivv_v0, parse_opivv_maskless_format as opivv_maskless,
            parse_opivx_format as opivx, parse_opivx_vmv_format as opivx_vmv, parse_opivx_v0_format as opivx_v0, parse_opivx_maskless_format as opivx_maskless,
            parse_opmvv_fma_format as opmvv_fma, parse_opmvv_format as opmvv,
            parse_opmvv_maskless_format as opmvv_maskless,
            parse_opmvx_fma_format as opmvx_fma, parse_opmvx_format as opmvx,
            parse_vfunary0_format as vfunary0, parse_vfunary1_format as vfunary1,
            parse_vl_format as vl, parse_vlm_format as vlm, parse_vlr_format as vlr,
            parse_vls_format as vls, parse_vlx_format as vlx, parse_vmunary0_format as vmunary0,
            parse_vmunary0_vidv_format as vidv, parse_vrfunary0_format as vrfunary0,
            parse_vrxunary0_format as vrxunary0, parse_vs_format as vs,
            parse_vsetivli_format as vsetivli, parse_vsetvl_format as vsetvl,
            parse_vsetvli_format as vsetvli, parse_vsm_format as vsm, parse_vsr_format as vsr,
            parse_vss_format as vss, parse_vsx_format as vsx, parse_vwfunary0_format as vwfunary0,
            parse_vwxunary0_format as vwxunary0, parse_vwxunary0_vmvxs_format as vmvxs,
            parse_vxunary0_format as vxunary0,
        };

        let instruction = match Self::rename(mnemonic) {
            "add" => Add(r(operands)?),
            "addw" => Addw(r(operands)?),
            "sub" => Sub(r(operands)?),
            "subw" => Subw(r(operands)?),
            "addi" => Addi(i(operands)?),
            "addiw" => Addiw(i(operands)?),
            "slt" => Slt(r(operands)?),
            "slti" => Slti(i(operands)?),
            "sltu" => Sltu(r(operands)?),
            "sltiu" => Sltiu(i(operands)?),
            "lui" => Lui(u(operands)?),
            "auipc" => Auipc(u(operands)?),

            "and" => And(r(operands)?),
            "or" => Or(r(operands)?),
            "xor" => Xor(r(operands)?),
            "andi" => Andi(i(operands)?),
            "ori" => Ori(i(operands)?),
            "xori" => Xori(i(operands)?),
            "sll" => Sll(r(operands)?),
            "sllw" => Sllw(r(operands)?),
            "srl" => Srl(r(operands)?),
            "srlw" => Srlw(r(operands)?),
            "sra" => Sra(r(operands)?),
            "sraw" => Sraw(r(operands)?),
            "slli" => Slli(i(operands)?),
            "slliw" => Slliw(i(operands)?),
            "srli" => Srli(i(operands)?),
            "srliw" => Srliw(i(operands)?),
            "srai" => Srai(i(operands)?),
            "sraiw" => Sraiw(i(operands)?),

            "ld" => Ld(l(operands)?),
            "lw" => Lw(l(operands)?),
            "lh" => Lh(l(operands)?),
            "lb" => Lb(l(operands)?),
            "lwu" => Lwu(l(operands)?),
            "lhu" => Lhu(l(operands)?),
            "lbu" => Lbu(l(operands)?),
            "sd" => Sd(s(operands)?),
            "sw" => Sw(s(operands)?),
            "sh" => Sh(s(operands)?),
            "sb" => Sb(s(operands)?),

            "beq" => Beq(b(operands, labels, current_address)?),
            "bne" => Bne(b(operands, labels, current_address)?),
            "bge" => Bge(b(operands, labels, current_address)?),
            "bgeu" => Bgeu(b(operands, labels, current_address)?),
            "blt" => Blt(b(operands, labels, current_address)?),
            "bltu" => Bltu(b(operands, labels, current_address)?),
            "jal" => {
                match u(operands) {
                    Ok(instruction) => Jal(instruction),
                    Err(fst_err) => match integer::pseudo::parse_label_format(operands, labels, current_address) {
                        Ok(diff) => Jal(U {
                            rd: 1,
                            imm20: diff,
                        }),
                        Err(snd_err) => return Err(format!("{} or {}", fst_err, snd_err))
                    },
                }
            },
            "jalr" => {
                match l(operands) {
                    Ok(instruction) => Jalr(instruction),
                    Err(fst_err) => match integer::pseudo::parse_op_format(operands) {
                        Ok(rs1) => Jalr(I {
                            rd: 1,
                            rs1,
                            imm12: 0,
                        }),
                        Err(snd_err) => return Err(format!("{} or {}", fst_err, snd_err))
                    },
                }
            },

            "csrrw" => Csrrw(csrr(operands)?),
            "csrrs" => Csrrs(csrr(operands)?),
            "csrrc" => Csrrc(csrr(operands)?),
            "csrrwi" => Csrrwi(csri(operands)?),
            "csrrsi" => Csrrsi(csri(operands)?),
            "csrrci" => Csrrci(csri(operands)?),

            "mul" => Mul(r(operands)?),
            "mulh" => Mulh(r(operands)?),
            "mulhsu" => Mulhsu(r(operands)?),
            "mulhu" => Mulhu(r(operands)?),
            "div" => Div(r(operands)?),
            "divu" => Divu(r(operands)?),
            "rem" => Rem(r(operands)?),
            "remu" => Remu(r(operands)?),
            "mulw" => Mulw(r(operands)?),
            "divw" => Divw(r(operands)?),
            "divuw" => Divuw(r(operands)?),
            "remw" => Remw(r(operands)?),
            "remuw" => Remuw(r(operands)?),

            "flw" => Flw(i(operands)?),
            "fsw" => Fsw(s(operands)?),
            "fmadd.s" => Fmadds(r4(operands)?),
            "fmsub.s" => Fmsubs(r4(operands)?),
            "fnmsub.s" => Fnmsubs(r4(operands)?),
            "fnmadd.s" => Fnmadds(r4(operands)?),
            "fadd.s" => Fadds(r(operands)?),
            "fsub.s" => Fsubs(r(operands)?),
            "fmul.s" => Fmuls(r(operands)?),
            "fdiv.s" => Fdivs(r(operands)?),
            "fsqrt.s" => Fsqrts(r(operands)?),
            "fsgnj.s" => Fsgnjs(r(operands)?),
            "fsgnjn.s" => Fsgnjns(r(operands)?),
            "fsgnjx.s" => Fsgnjxs(r(operands)?),
            "fmin.s" => Fmins(r(operands)?),
            "fmax.s" => Fmaxs(r(operands)?),
            "fcvt.w.s" => Fcvtws(r(operands)?),
            "fcvt.wu.s" => Fcvtwus(r(operands)?),
            "fmv.x.w" => Fmvxw(r(operands)?),
            "feq.s" => Feqs(r(operands)?),
            "flt.s" => Flts(r(operands)?),
            "fle.s" => Fles(r(operands)?),
            "fclass.s" => Fclasss(r(operands)?),
            "fcvt.s.w" => Fcvtsw(r(operands)?),
            "fcvt.s.wu" => Fcvtswu(r(operands)?),
            "fmv.w.x" => Fmvwx(r(operands)?),
            "fcvt.l.s" => Fcvtls(r(operands)?),
            "fcvt.lu.s" => Fcvtlus(r(operands)?),
            "fcvt.s.l" => Fcvtsl(r(operands)?),
            "fcvt.s.lu" => Fcvtslu(r(operands)?),

            "fld" => Fld(i(operands)?),
            "fsd" => Fsd(s(operands)?),
            "fmadd.d" => Fmaddd(r4(operands)?),
            "fmsub.d" => Fmsubd(r4(operands)?),
            "fnmsub.d" => Fnmsubd(r4(operands)?),
            "fnmadd.d" => Fnmaddd(r4(operands)?),
            "fadd.d" => Faddd(r(operands)?),
            "fsub.d" => Fsubd(r(operands)?),
            "fmul.d" => Fmuld(r(operands)?),
            "fdiv.d" => Fdivd(r(operands)?),
            "fsqrt.d" => Fsqrtd(r(operands)?),
            "fsgnj.d" => Fsgnjd(r(operands)?),
            "fsgnjn.d" => Fsgnjnd(r(operands)?),
            "fsgnjx.d" => Fsgnjxd(r(operands)?),
            "fmin.d" => Fmind(r(operands)?),
            "fmax.d" => Fmaxd(r(operands)?),
            "fcvt.s.d" => Fcvtsd(r(operands)?),
            "fcvt.d.s" => Fcvtds(r(operands)?),
            "feq.d" => Feqd(r(operands)?),
            "flt.d" => Fltd(r(operands)?),
            "fle.d" => Fled(r(operands)?),
            "fclass.d" => Fclassd(r(operands)?),
            "fcvt.w.d" => Fcvtwd(r(operands)?),
            "fcvt.wu.d" => Fcvtwud(r(operands)?),
            "fcvt.d.w" => Fcvtdw(r(operands)?),
            "fcvt.d.wu" => Fcvtdwu(r(operands)?),
            "fcvt.l.d" => Fcvtld(r(operands)?),
            "fcvt.lu.d" => Fcvtlud(r(operands)?),
            "fmv.x.d" => Fmvxd(r(operands)?),
            "fcvt.d.l" => Fcvtdl(r(operands)?),
            "fcvt.d.lu" => Fcvtdlu(r(operands)?),
            "fmv.d.x" => Fmvdx(r(operands)?),

            "vsetvli" => Vsetvli(vsetvli(operands)?),
            "vsetivli" => Vsetivli(vsetivli(operands)?),
            "vsetvl" => Vsetvl(vsetvl(operands)?),

            "vle8.v" => Vlv {
                eew: SEW::E8,
                data: vl(operands)?,
            },
            "vle16.v" => Vlv {
                eew: SEW::E16,
                data: vl(operands)?,
            },
            "vle32.v" => Vlv {
                eew: SEW::E32,
                data: vl(operands)?,
            },
            "vle64.v" => Vlv {
                eew: SEW::E64,
                data: vl(operands)?,
            },

            "vse8.v" => Vsv {
                eew: SEW::E8,
                data: vs(operands)?,
            },
            "vse16.v" => Vsv {
                eew: SEW::E16,
                data: vs(operands)?,
            },
            "vse32.v" => Vsv {
                eew: SEW::E32,
                data: vs(operands)?,
            },
            "vse64.v" => Vsv {
                eew: SEW::E64,
                data: vs(operands)?,
            },

            "vlm.v" => Vlmv(vlm(operands)?),
            "vsm.v" => Vsmv(vsm(operands)?),

            "vlse8.v" => Vlsv {
                eew: SEW::E8,
                data: vls(operands)?,
            },
            "vlse16.v" => Vlsv {
                eew: SEW::E16,
                data: vls(operands)?,
            },
            "vlse32.v" => Vlsv {
                eew: SEW::E32,
                data: vls(operands)?,
            },
            "vlse64.v" => Vlsv {
                eew: SEW::E64,
                data: vls(operands)?,
            },

            "vsse8.v" => Vssv {
                eew: SEW::E8,
                data: vss(operands)?,
            },
            "vsse16.v" => Vssv {
                eew: SEW::E16,
                data: vss(operands)?,
            },
            "vsse32.v" => Vssv {
                eew: SEW::E32,
                data: vss(operands)?,
            },
            "vsse64.v" => Vssv {
                eew: SEW::E64,
                data: vss(operands)?,
            },

            "vluxei8.v" => Vluxv {
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vluxei16.v" => Vluxv {
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vluxei32.v" => Vluxv {
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vluxei64.v" => Vluxv {
                eew: SEW::E64,
                data: vlx(operands)?,
            },

            "vloxei8.v" => Vloxv {
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vloxei16.v" => Vloxv {
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vloxei32.v" => Vloxv {
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vloxei64.v" => Vloxv {
                eew: SEW::E64,
                data: vlx(operands)?,
            },

            "vsuxei8.v" => Vsuxv {
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsuxei16.v" => Vsuxv {
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsuxei32.v" => Vsuxv {
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsuxei64.v" => Vsuxv {
                eew: SEW::E64,
                data: vsx(operands)?,
            },

            "vsuxeix8.v" => Vsuxv {
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsuxeix16.v" => Vsuxv {
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsuxeix32.v" => Vsuxv {
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsuxeix64.v" => Vsuxv {
                eew: SEW::E64,
                data: vsx(operands)?,
            },

            "vle8ff.v" => Vlffv {
                eew: SEW::E8,
                data: vl(operands)?,
            },
            "vle16ff.v" => Vlffv {
                eew: SEW::E16,
                data: vl(operands)?,
            },
            "vle32ff.v" => Vlffv {
                eew: SEW::E32,
                data: vl(operands)?,
            },
            "vle64ff.v" => Vlffv {
                eew: SEW::E64,
                data: vl(operands)?,
            },

            // Note: I need to list all combinations so that I can research const-generification segmented load/stores in the future
            "vlseg1e8.v" => Vlsegv {
                nf: 1,
                eew: SEW::E8,
                data: vl(operands)?,
            },
            "vlseg1e16.v" => Vlsegv {
                nf: 1,
                eew: SEW::E16,
                data: vl(operands)?,
            },
            "vlseg1e32.v" => Vlsegv {
                nf: 1,
                eew: SEW::E32,
                data: vl(operands)?,
            },
            "vlseg1e64.v" => Vlsegv {
                nf: 1,
                eew: SEW::E64,
                data: vl(operands)?,
            },
            "vlseg2e8.v" => Vlsegv {
                nf: 2,
                eew: SEW::E8,
                data: vl(operands)?,
            },
            "vlseg2e16.v" => Vlsegv {
                nf: 2,
                eew: SEW::E16,
                data: vl(operands)?,
            },
            "vlseg2e32.v" => Vlsegv {
                nf: 2,
                eew: SEW::E32,
                data: vl(operands)?,
            },
            "vlseg2e64.v" => Vlsegv {
                nf: 2,
                eew: SEW::E64,
                data: vl(operands)?,
            },
            "vlseg3e8.v" => Vlsegv {
                nf: 3,
                eew: SEW::E8,
                data: vl(operands)?,
            },
            "vlseg3e16.v" => Vlsegv {
                nf: 3,
                eew: SEW::E16,
                data: vl(operands)?,
            },
            "vlseg3e32.v" => Vlsegv {
                nf: 3,
                eew: SEW::E32,
                data: vl(operands)?,
            },
            "vlseg3e64.v" => Vlsegv {
                nf: 3,
                eew: SEW::E64,
                data: vl(operands)?,
            },
            "vlseg4e8.v" => Vlsegv {
                nf: 4,
                eew: SEW::E8,
                data: vl(operands)?,
            },
            "vlseg4e16.v" => Vlsegv {
                nf: 4,
                eew: SEW::E16,
                data: vl(operands)?,
            },
            "vlseg4e32.v" => Vlsegv {
                nf: 4,
                eew: SEW::E32,
                data: vl(operands)?,
            },
            "vlseg4e64.v" => Vlsegv {
                nf: 4,
                eew: SEW::E64,
                data: vl(operands)?,
            },
            "vlseg5e8.v" => Vlsegv {
                nf: 5,
                eew: SEW::E8,
                data: vl(operands)?,
            },
            "vlseg5e16.v" => Vlsegv {
                nf: 5,
                eew: SEW::E16,
                data: vl(operands)?,
            },
            "vlseg5e32.v" => Vlsegv {
                nf: 5,
                eew: SEW::E32,
                data: vl(operands)?,
            },
            "vlseg5e64.v" => Vlsegv {
                nf: 5,
                eew: SEW::E64,
                data: vl(operands)?,
            },
            "vlseg6e8.v" => Vlsegv {
                nf: 6,
                eew: SEW::E8,
                data: vl(operands)?,
            },
            "vlseg6e16.v" => Vlsegv {
                nf: 6,
                eew: SEW::E16,
                data: vl(operands)?,
            },
            "vlseg6e32.v" => Vlsegv {
                nf: 6,
                eew: SEW::E32,
                data: vl(operands)?,
            },
            "vlseg6e64.v" => Vlsegv {
                nf: 6,
                eew: SEW::E64,
                data: vl(operands)?,
            },
            "vlseg7e8.v" => Vlsegv {
                nf: 7,
                eew: SEW::E8,
                data: vl(operands)?,
            },
            "vlseg7e16.v" => Vlsegv {
                nf: 7,
                eew: SEW::E16,
                data: vl(operands)?,
            },
            "vlseg7e32.v" => Vlsegv {
                nf: 7,
                eew: SEW::E32,
                data: vl(operands)?,
            },
            "vlseg7e64.v" => Vlsegv {
                nf: 7,
                eew: SEW::E64,
                data: vl(operands)?,
            },
            "vlseg8e8.v" => Vlsegv {
                nf: 8,
                eew: SEW::E8,
                data: vl(operands)?,
            },
            "vlseg8e16.v" => Vlsegv {
                nf: 8,
                eew: SEW::E16,
                data: vl(operands)?,
            },
            "vlseg8e32.v" => Vlsegv {
                nf: 8,
                eew: SEW::E32,
                data: vl(operands)?,
            },
            "vlseg8e64.v" => Vlsegv {
                nf: 8,
                eew: SEW::E64,
                data: vl(operands)?,
            },

            "vsseg1e8.v" => Vssegv {
                nf: 1,
                eew: SEW::E8,
                data: vs(operands)?,
            },
            "vsseg1e16.v" => Vssegv {
                nf: 1,
                eew: SEW::E16,
                data: vs(operands)?,
            },
            "vsseg1e32.v" => Vssegv {
                nf: 1,
                eew: SEW::E32,
                data: vs(operands)?,
            },
            "vsseg1e64.v" => Vssegv {
                nf: 1,
                eew: SEW::E64,
                data: vs(operands)?,
            },
            "vsseg2e8.v" => Vssegv {
                nf: 2,
                eew: SEW::E8,
                data: vs(operands)?,
            },
            "vsseg2e16.v" => Vssegv {
                nf: 2,
                eew: SEW::E16,
                data: vs(operands)?,
            },
            "vsseg2e32.v" => Vssegv {
                nf: 2,
                eew: SEW::E32,
                data: vs(operands)?,
            },
            "vsseg2e64.v" => Vssegv {
                nf: 2,
                eew: SEW::E64,
                data: vs(operands)?,
            },
            "vsseg3e8.v" => Vssegv {
                nf: 3,
                eew: SEW::E8,
                data: vs(operands)?,
            },
            "vsseg3e16.v" => Vssegv {
                nf: 3,
                eew: SEW::E16,
                data: vs(operands)?,
            },
            "vsseg3e32.v" => Vssegv {
                nf: 3,
                eew: SEW::E32,
                data: vs(operands)?,
            },
            "vsseg3e64.v" => Vssegv {
                nf: 3,
                eew: SEW::E64,
                data: vs(operands)?,
            },
            "vsseg4e8.v" => Vssegv {
                nf: 4,
                eew: SEW::E8,
                data: vs(operands)?,
            },
            "vsseg4e16.v" => Vssegv {
                nf: 4,
                eew: SEW::E16,
                data: vs(operands)?,
            },
            "vsseg4e32.v" => Vssegv {
                nf: 4,
                eew: SEW::E32,
                data: vs(operands)?,
            },
            "vsseg4e64.v" => Vssegv {
                nf: 4,
                eew: SEW::E64,
                data: vs(operands)?,
            },
            "vsseg5e8.v" => Vssegv {
                nf: 5,
                eew: SEW::E8,
                data: vs(operands)?,
            },
            "vsseg5e16.v" => Vssegv {
                nf: 5,
                eew: SEW::E16,
                data: vs(operands)?,
            },
            "vsseg5e32.v" => Vssegv {
                nf: 5,
                eew: SEW::E32,
                data: vs(operands)?,
            },
            "vsseg5e64.v" => Vssegv {
                nf: 5,
                eew: SEW::E64,
                data: vs(operands)?,
            },
            "vsseg6e8.v" => Vssegv {
                nf: 6,
                eew: SEW::E8,
                data: vs(operands)?,
            },
            "vsseg6e16.v" => Vssegv {
                nf: 6,
                eew: SEW::E16,
                data: vs(operands)?,
            },
            "vsseg6e32.v" => Vssegv {
                nf: 6,
                eew: SEW::E32,
                data: vs(operands)?,
            },
            "vsseg6e64.v" => Vssegv {
                nf: 6,
                eew: SEW::E64,
                data: vs(operands)?,
            },
            "vsseg7e8.v" => Vssegv {
                nf: 7,
                eew: SEW::E8,
                data: vs(operands)?,
            },
            "vsseg7e16.v" => Vssegv {
                nf: 7,
                eew: SEW::E16,
                data: vs(operands)?,
            },
            "vsseg7e32.v" => Vssegv {
                nf: 7,
                eew: SEW::E32,
                data: vs(operands)?,
            },
            "vsseg7e64.v" => Vssegv {
                nf: 7,
                eew: SEW::E64,
                data: vs(operands)?,
            },
            "vsseg8e8.v" => Vssegv {
                nf: 8,
                eew: SEW::E8,
                data: vs(operands)?,
            },
            "vsseg8e16.v" => Vssegv {
                nf: 8,
                eew: SEW::E16,
                data: vs(operands)?,
            },
            "vsseg8e32.v" => Vssegv {
                nf: 8,
                eew: SEW::E32,
                data: vs(operands)?,
            },
            "vsseg8e64.v" => Vssegv {
                nf: 8,
                eew: SEW::E64,
                data: vs(operands)?,
            },

            "vlsseg1e8.v" => Vlssegv {
                nf: 1,
                eew: SEW::E8,
                data: vls(operands)?,
            },
            "vlsseg1e16.v" => Vlssegv {
                nf: 1,
                eew: SEW::E16,
                data: vls(operands)?,
            },
            "vlsseg1e32.v" => Vlssegv {
                nf: 1,
                eew: SEW::E32,
                data: vls(operands)?,
            },
            "vlsseg1e64.v" => Vlssegv {
                nf: 1,
                eew: SEW::E64,
                data: vls(operands)?,
            },
            "vlsseg2e8.v" => Vlssegv {
                nf: 2,
                eew: SEW::E8,
                data: vls(operands)?,
            },
            "vlsseg2e16.v" => Vlssegv {
                nf: 2,
                eew: SEW::E16,
                data: vls(operands)?,
            },
            "vlsseg2e32.v" => Vlssegv {
                nf: 2,
                eew: SEW::E32,
                data: vls(operands)?,
            },
            "vlsseg2e64.v" => Vlssegv {
                nf: 2,
                eew: SEW::E64,
                data: vls(operands)?,
            },
            "vlsseg3e8.v" => Vlssegv {
                nf: 3,
                eew: SEW::E8,
                data: vls(operands)?,
            },
            "vlsseg3e16.v" => Vlssegv {
                nf: 3,
                eew: SEW::E16,
                data: vls(operands)?,
            },
            "vlsseg3e32.v" => Vlssegv {
                nf: 3,
                eew: SEW::E32,
                data: vls(operands)?,
            },
            "vlsseg3e64.v" => Vlssegv {
                nf: 3,
                eew: SEW::E64,
                data: vls(operands)?,
            },
            "vlsseg4e8.v" => Vlssegv {
                nf: 4,
                eew: SEW::E8,
                data: vls(operands)?,
            },
            "vlsseg4e16.v" => Vlssegv {
                nf: 4,
                eew: SEW::E16,
                data: vls(operands)?,
            },
            "vlsseg4e32.v" => Vlssegv {
                nf: 4,
                eew: SEW::E32,
                data: vls(operands)?,
            },
            "vlsseg4e64.v" => Vlssegv {
                nf: 4,
                eew: SEW::E64,
                data: vls(operands)?,
            },
            "vlsseg5e8.v" => Vlssegv {
                nf: 5,
                eew: SEW::E8,
                data: vls(operands)?,
            },
            "vlsseg5e16.v" => Vlssegv {
                nf: 5,
                eew: SEW::E16,
                data: vls(operands)?,
            },
            "vlsseg5e32.v" => Vlssegv {
                nf: 5,
                eew: SEW::E32,
                data: vls(operands)?,
            },
            "vlsseg5e64.v" => Vlssegv {
                nf: 5,
                eew: SEW::E64,
                data: vls(operands)?,
            },
            "vlsseg6e8.v" => Vlssegv {
                nf: 6,
                eew: SEW::E8,
                data: vls(operands)?,
            },
            "vlsseg6e16.v" => Vlssegv {
                nf: 6,
                eew: SEW::E16,
                data: vls(operands)?,
            },
            "vlsseg6e32.v" => Vlssegv {
                nf: 6,
                eew: SEW::E32,
                data: vls(operands)?,
            },
            "vlsseg6e64.v" => Vlssegv {
                nf: 6,
                eew: SEW::E64,
                data: vls(operands)?,
            },
            "vlsseg7e8.v" => Vlssegv {
                nf: 7,
                eew: SEW::E8,
                data: vls(operands)?,
            },
            "vlsseg7e16.v" => Vlssegv {
                nf: 7,
                eew: SEW::E16,
                data: vls(operands)?,
            },
            "vlsseg7e32.v" => Vlssegv {
                nf: 7,
                eew: SEW::E32,
                data: vls(operands)?,
            },
            "vlsseg7e64.v" => Vlssegv {
                nf: 7,
                eew: SEW::E64,
                data: vls(operands)?,
            },
            "vlsseg8e8.v" => Vlssegv {
                nf: 8,
                eew: SEW::E8,
                data: vls(operands)?,
            },
            "vlsseg8e16.v" => Vlssegv {
                nf: 8,
                eew: SEW::E16,
                data: vls(operands)?,
            },
            "vlsseg8e32.v" => Vlssegv {
                nf: 8,
                eew: SEW::E32,
                data: vls(operands)?,
            },
            "vlsseg8e64.v" => Vlssegv {
                nf: 8,
                eew: SEW::E64,
                data: vls(operands)?,
            },

            "vssseg1e8.v" => Vsssegv {
                nf: 1,
                eew: SEW::E8,
                data: vss(operands)?,
            },
            "vssseg1e16.v" => Vsssegv {
                nf: 1,
                eew: SEW::E16,
                data: vss(operands)?,
            },
            "vssseg1e32.v" => Vsssegv {
                nf: 1,
                eew: SEW::E32,
                data: vss(operands)?,
            },
            "vssseg1e64.v" => Vsssegv {
                nf: 1,
                eew: SEW::E64,
                data: vss(operands)?,
            },
            "vssseg2e8.v" => Vsssegv {
                nf: 2,
                eew: SEW::E8,
                data: vss(operands)?,
            },
            "vssseg2e16.v" => Vsssegv {
                nf: 2,
                eew: SEW::E16,
                data: vss(operands)?,
            },
            "vssseg2e32.v" => Vsssegv {
                nf: 2,
                eew: SEW::E32,
                data: vss(operands)?,
            },
            "vssseg2e64.v" => Vsssegv {
                nf: 2,
                eew: SEW::E64,
                data: vss(operands)?,
            },
            "vssseg3e8.v" => Vsssegv {
                nf: 3,
                eew: SEW::E8,
                data: vss(operands)?,
            },
            "vssseg3e16.v" => Vsssegv {
                nf: 3,
                eew: SEW::E16,
                data: vss(operands)?,
            },
            "vssseg3e32.v" => Vsssegv {
                nf: 3,
                eew: SEW::E32,
                data: vss(operands)?,
            },
            "vssseg3e64.v" => Vsssegv {
                nf: 3,
                eew: SEW::E64,
                data: vss(operands)?,
            },
            "vssseg4e8.v" => Vsssegv {
                nf: 4,
                eew: SEW::E8,
                data: vss(operands)?,
            },
            "vssseg4e16.v" => Vsssegv {
                nf: 4,
                eew: SEW::E16,
                data: vss(operands)?,
            },
            "vssseg4e32.v" => Vsssegv {
                nf: 4,
                eew: SEW::E32,
                data: vss(operands)?,
            },
            "vssseg4e64.v" => Vsssegv {
                nf: 4,
                eew: SEW::E64,
                data: vss(operands)?,
            },
            "vssseg5e8.v" => Vsssegv {
                nf: 5,
                eew: SEW::E8,
                data: vss(operands)?,
            },
            "vssseg5e16.v" => Vsssegv {
                nf: 5,
                eew: SEW::E16,
                data: vss(operands)?,
            },
            "vssseg5e32.v" => Vsssegv {
                nf: 5,
                eew: SEW::E32,
                data: vss(operands)?,
            },
            "vssseg5e64.v" => Vsssegv {
                nf: 5,
                eew: SEW::E64,
                data: vss(operands)?,
            },
            "vssseg6e8.v" => Vsssegv {
                nf: 6,
                eew: SEW::E8,
                data: vss(operands)?,
            },
            "vssseg6e16.v" => Vsssegv {
                nf: 6,
                eew: SEW::E16,
                data: vss(operands)?,
            },
            "vssseg6e32.v" => Vsssegv {
                nf: 6,
                eew: SEW::E32,
                data: vss(operands)?,
            },
            "vssseg6e64.v" => Vsssegv {
                nf: 6,
                eew: SEW::E64,
                data: vss(operands)?,
            },
            "vssseg7e8.v" => Vsssegv {
                nf: 7,
                eew: SEW::E8,
                data: vss(operands)?,
            },
            "vssseg7e16.v" => Vsssegv {
                nf: 7,
                eew: SEW::E16,
                data: vss(operands)?,
            },
            "vssseg7e32.v" => Vsssegv {
                nf: 7,
                eew: SEW::E32,
                data: vss(operands)?,
            },
            "vssseg7e64.v" => Vsssegv {
                nf: 7,
                eew: SEW::E64,
                data: vss(operands)?,
            },
            "vssseg8e8.v" => Vsssegv {
                nf: 8,
                eew: SEW::E8,
                data: vss(operands)?,
            },
            "vssseg8e16.v" => Vsssegv {
                nf: 8,
                eew: SEW::E16,
                data: vss(operands)?,
            },
            "vssseg8e32.v" => Vsssegv {
                nf: 8,
                eew: SEW::E32,
                data: vss(operands)?,
            },
            "vssseg8e64.v" => Vsssegv {
                nf: 8,
                eew: SEW::E64,
                data: vss(operands)?,
            },

            "vluxseg1ei8.v" => Vluxsegv {
                nf: 1,
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vluxseg1ei16.v" => Vluxsegv {
                nf: 1,
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vluxseg1ei32.v" => Vluxsegv {
                nf: 1,
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vluxseg1ei64.v" => Vluxsegv {
                nf: 1,
                eew: SEW::E64,
                data: vlx(operands)?,
            },
            "vluxseg2ei8.v" => Vluxsegv {
                nf: 2,
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vluxseg2ei16.v" => Vluxsegv {
                nf: 2,
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vluxseg2ei32.v" => Vluxsegv {
                nf: 2,
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vluxseg2ei64.v" => Vluxsegv {
                nf: 2,
                eew: SEW::E64,
                data: vlx(operands)?,
            },
            "vluxseg3ei8.v" => Vluxsegv {
                nf: 3,
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vluxseg3ei16.v" => Vluxsegv {
                nf: 3,
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vluxseg3ei32.v" => Vluxsegv {
                nf: 3,
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vluxseg3ei64.v" => Vluxsegv {
                nf: 3,
                eew: SEW::E64,
                data: vlx(operands)?,
            },
            "vluxseg4ei8.v" => Vluxsegv {
                nf: 4,
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vluxseg4ei16.v" => Vluxsegv {
                nf: 4,
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vluxseg4ei32.v" => Vluxsegv {
                nf: 4,
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vluxseg4ei64.v" => Vluxsegv {
                nf: 4,
                eew: SEW::E64,
                data: vlx(operands)?,
            },
            "vluxseg5ei8.v" => Vluxsegv {
                nf: 5,
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vluxseg5ei16.v" => Vluxsegv {
                nf: 5,
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vluxseg5ei32.v" => Vluxsegv {
                nf: 5,
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vluxseg5ei64.v" => Vluxsegv {
                nf: 5,
                eew: SEW::E64,
                data: vlx(operands)?,
            },
            "vluxseg6ei8.v" => Vluxsegv {
                nf: 6,
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vluxseg6ei16.v" => Vluxsegv {
                nf: 6,
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vluxseg6ei32.v" => Vluxsegv {
                nf: 6,
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vluxseg6ei64.v" => Vluxsegv {
                nf: 6,
                eew: SEW::E64,
                data: vlx(operands)?,
            },
            "vluxseg7ei8.v" => Vluxsegv {
                nf: 7,
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vluxseg7ei16.v" => Vluxsegv {
                nf: 7,
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vluxseg7ei32.v" => Vluxsegv {
                nf: 7,
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vluxseg7ei64.v" => Vluxsegv {
                nf: 7,
                eew: SEW::E64,
                data: vlx(operands)?,
            },
            "vluxseg8ei8.v" => Vluxsegv {
                nf: 8,
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vluxseg8ei16.v" => Vluxsegv {
                nf: 8,
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vluxseg8ei32.v" => Vluxsegv {
                nf: 8,
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vluxseg8ei64.v" => Vluxsegv {
                nf: 8,
                eew: SEW::E64,
                data: vlx(operands)?,
            },

            "vloxseg1ei8.v" => Vloxsegv {
                nf: 1,
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vloxseg1ei16.v" => Vloxsegv {
                nf: 1,
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vloxseg1ei32.v" => Vloxsegv {
                nf: 1,
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vloxseg1ei64.v" => Vloxsegv {
                nf: 1,
                eew: SEW::E64,
                data: vlx(operands)?,
            },
            "vloxseg2ei8.v" => Vloxsegv {
                nf: 2,
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vloxseg2ei16.v" => Vloxsegv {
                nf: 2,
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vloxseg2ei32.v" => Vloxsegv {
                nf: 2,
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vloxseg2ei64.v" => Vloxsegv {
                nf: 2,
                eew: SEW::E64,
                data: vlx(operands)?,
            },
            "vloxseg3ei8.v" => Vloxsegv {
                nf: 3,
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vloxseg3ei16.v" => Vloxsegv {
                nf: 3,
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vloxseg3ei32.v" => Vloxsegv {
                nf: 3,
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vloxseg3ei64.v" => Vloxsegv {
                nf: 3,
                eew: SEW::E64,
                data: vlx(operands)?,
            },
            "vloxseg4ei8.v" => Vloxsegv {
                nf: 4,
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vloxseg4ei16.v" => Vloxsegv {
                nf: 4,
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vloxseg4ei32.v" => Vloxsegv {
                nf: 4,
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vloxseg4ei64.v" => Vloxsegv {
                nf: 4,
                eew: SEW::E64,
                data: vlx(operands)?,
            },
            "vloxseg5ei8.v" => Vloxsegv {
                nf: 5,
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vloxseg5ei16.v" => Vloxsegv {
                nf: 5,
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vloxseg5ei32.v" => Vloxsegv {
                nf: 5,
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vloxseg5ei64.v" => Vloxsegv {
                nf: 5,
                eew: SEW::E64,
                data: vlx(operands)?,
            },
            "vloxseg6ei8.v" => Vloxsegv {
                nf: 6,
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vloxseg6ei16.v" => Vloxsegv {
                nf: 6,
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vloxseg6ei32.v" => Vloxsegv {
                nf: 6,
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vloxseg6ei64.v" => Vloxsegv {
                nf: 6,
                eew: SEW::E64,
                data: vlx(operands)?,
            },
            "vloxseg7ei8.v" => Vloxsegv {
                nf: 7,
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vloxseg7ei16.v" => Vloxsegv {
                nf: 7,
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vloxseg7ei32.v" => Vloxsegv {
                nf: 7,
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vloxseg7ei64.v" => Vloxsegv {
                nf: 7,
                eew: SEW::E64,
                data: vlx(operands)?,
            },
            "vloxseg8ei8.v" => Vloxsegv {
                nf: 8,
                eew: SEW::E8,
                data: vlx(operands)?,
            },
            "vloxseg8ei16.v" => Vloxsegv {
                nf: 8,
                eew: SEW::E16,
                data: vlx(operands)?,
            },
            "vloxseg8ei32.v" => Vloxsegv {
                nf: 8,
                eew: SEW::E32,
                data: vlx(operands)?,
            },
            "vloxseg8ei64.v" => Vloxsegv {
                nf: 8,
                eew: SEW::E64,
                data: vlx(operands)?,
            },

            "vsuxseg1ei8.v" => Vsuxsegv {
                nf: 1,
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsuxseg1ei16.v" => Vsuxsegv {
                nf: 1,
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsuxseg1ei32.v" => Vsuxsegv {
                nf: 1,
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsuxseg1ei64.v" => Vsuxsegv {
                nf: 1,
                eew: SEW::E64,
                data: vsx(operands)?,
            },
            "vsuxseg2ei8.v" => Vsuxsegv {
                nf: 2,
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsuxseg2ei16.v" => Vsuxsegv {
                nf: 2,
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsuxseg2ei32.v" => Vsuxsegv {
                nf: 2,
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsuxseg2ei64.v" => Vsuxsegv {
                nf: 2,
                eew: SEW::E64,
                data: vsx(operands)?,
            },
            "vsuxseg3ei8.v" => Vsuxsegv {
                nf: 3,
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsuxseg3ei16.v" => Vsuxsegv {
                nf: 3,
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsuxseg3ei32.v" => Vsuxsegv {
                nf: 3,
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsuxseg3ei64.v" => Vsuxsegv {
                nf: 3,
                eew: SEW::E64,
                data: vsx(operands)?,
            },
            "vsuxseg4ei8.v" => Vsuxsegv {
                nf: 4,
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsuxseg4ei16.v" => Vsuxsegv {
                nf: 4,
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsuxseg4ei32.v" => Vsuxsegv {
                nf: 4,
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsuxseg4ei64.v" => Vsuxsegv {
                nf: 4,
                eew: SEW::E64,
                data: vsx(operands)?,
            },
            "vsuxseg5ei8.v" => Vsuxsegv {
                nf: 5,
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsuxseg5ei16.v" => Vsuxsegv {
                nf: 5,
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsuxseg5ei32.v" => Vsuxsegv {
                nf: 5,
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsuxseg5ei64.v" => Vsuxsegv {
                nf: 5,
                eew: SEW::E64,
                data: vsx(operands)?,
            },
            "vsuxseg6ei8.v" => Vsuxsegv {
                nf: 6,
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsuxseg6ei16.v" => Vsuxsegv {
                nf: 6,
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsuxseg6ei32.v" => Vsuxsegv {
                nf: 6,
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsuxseg6ei64.v" => Vsuxsegv {
                nf: 6,
                eew: SEW::E64,
                data: vsx(operands)?,
            },
            "vsuxseg7ei8.v" => Vsuxsegv {
                nf: 7,
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsuxseg7ei16.v" => Vsuxsegv {
                nf: 7,
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsuxseg7ei32.v" => Vsuxsegv {
                nf: 7,
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsuxseg7ei64.v" => Vsuxsegv {
                nf: 7,
                eew: SEW::E64,
                data: vsx(operands)?,
            },
            "vsuxseg8ei8.v" => Vsuxsegv {
                nf: 8,
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsuxseg8ei16.v" => Vsuxsegv {
                nf: 8,
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsuxseg8ei32.v" => Vsuxsegv {
                nf: 8,
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsuxseg8ei64.v" => Vsuxsegv {
                nf: 8,
                eew: SEW::E64,
                data: vsx(operands)?,
            },

            "vsoxseg1ei8.v" => Vsoxsegv {
                nf: 1,
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsoxseg1ei16.v" => Vsoxsegv {
                nf: 1,
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsoxseg1ei32.v" => Vsoxsegv {
                nf: 1,
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsoxseg1ei64.v" => Vsoxsegv {
                nf: 1,
                eew: SEW::E64,
                data: vsx(operands)?,
            },
            "vsoxseg2ei8.v" => Vsoxsegv {
                nf: 2,
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsoxseg2ei16.v" => Vsoxsegv {
                nf: 2,
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsoxseg2ei32.v" => Vsoxsegv {
                nf: 2,
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsoxseg2ei64.v" => Vsoxsegv {
                nf: 2,
                eew: SEW::E64,
                data: vsx(operands)?,
            },
            "vsoxseg3ei8.v" => Vsoxsegv {
                nf: 3,
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsoxseg3ei16.v" => Vsoxsegv {
                nf: 3,
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsoxseg3ei32.v" => Vsoxsegv {
                nf: 3,
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsoxseg3ei64.v" => Vsoxsegv {
                nf: 3,
                eew: SEW::E64,
                data: vsx(operands)?,
            },
            "vsoxseg4ei8.v" => Vsoxsegv {
                nf: 4,
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsoxseg4ei16.v" => Vsoxsegv {
                nf: 4,
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsoxseg4ei32.v" => Vsoxsegv {
                nf: 4,
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsoxseg4ei64.v" => Vsoxsegv {
                nf: 4,
                eew: SEW::E64,
                data: vsx(operands)?,
            },
            "vsoxseg5ei8.v" => Vsoxsegv {
                nf: 5,
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsoxseg5ei16.v" => Vsoxsegv {
                nf: 5,
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsoxseg5ei32.v" => Vsoxsegv {
                nf: 5,
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsoxseg5ei64.v" => Vsoxsegv {
                nf: 5,
                eew: SEW::E64,
                data: vsx(operands)?,
            },
            "vsoxseg6ei8.v" => Vsoxsegv {
                nf: 6,
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsoxseg6ei16.v" => Vsoxsegv {
                nf: 6,
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsoxseg6ei32.v" => Vsoxsegv {
                nf: 6,
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsoxseg6ei64.v" => Vsoxsegv {
                nf: 6,
                eew: SEW::E64,
                data: vsx(operands)?,
            },
            "vsoxseg7ei8.v" => Vsoxsegv {
                nf: 7,
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsoxseg7ei16.v" => Vsoxsegv {
                nf: 7,
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsoxseg7ei32.v" => Vsoxsegv {
                nf: 7,
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsoxseg7ei64.v" => Vsoxsegv {
                nf: 7,
                eew: SEW::E64,
                data: vsx(operands)?,
            },
            "vsoxseg8ei8.v" => Vsoxsegv {
                nf: 8,
                eew: SEW::E8,
                data: vsx(operands)?,
            },
            "vsoxseg8ei16.v" => Vsoxsegv {
                nf: 8,
                eew: SEW::E16,
                data: vsx(operands)?,
            },
            "vsoxseg8ei32.v" => Vsoxsegv {
                nf: 8,
                eew: SEW::E32,
                data: vsx(operands)?,
            },
            "vsoxseg8ei64.v" => Vsoxsegv {
                nf: 8,
                eew: SEW::E64,
                data: vsx(operands)?,
            },

            "vl1re8.v" => Vlrv {
                nf: 1,
                eew: SEW::E8,
                data: vlr(operands)?,
            },
            "vl1re16.v" => Vlrv {
                nf: 1,
                eew: SEW::E16,
                data: vlr(operands)?,
            },
            "vl1re32.v" => Vlrv {
                nf: 1,
                eew: SEW::E32,
                data: vlr(operands)?,
            },
            "vl1re64.v" => Vlrv {
                nf: 1,
                eew: SEW::E64,
                data: vlr(operands)?,
            },
            "vl2re8.v" => Vlrv {
                nf: 2,
                eew: SEW::E8,
                data: vlr(operands)?,
            },
            "vl2re16.v" => Vlrv {
                nf: 2,
                eew: SEW::E16,
                data: vlr(operands)?,
            },
            "vl2re32.v" => Vlrv {
                nf: 2,
                eew: SEW::E32,
                data: vlr(operands)?,
            },
            "vl2re64.v" => Vlrv {
                nf: 2,
                eew: SEW::E64,
                data: vlr(operands)?,
            },
            "vl4re8.v" => Vlrv {
                nf: 4,
                eew: SEW::E8,
                data: vlr(operands)?,
            },
            "vl4re16.v" => Vlrv {
                nf: 4,
                eew: SEW::E16,
                data: vlr(operands)?,
            },
            "vl4re32.v" => Vlrv {
                nf: 4,
                eew: SEW::E32,
                data: vlr(operands)?,
            },
            "vl4re64.v" => Vlrv {
                nf: 4,
                eew: SEW::E64,
                data: vlr(operands)?,
            },
            "vl8re8.v" => Vlrv {
                nf: 8,
                eew: SEW::E8,
                data: vlr(operands)?,
            },
            "vl8re16.v" => Vlrv {
                nf: 8,
                eew: SEW::E16,
                data: vlr(operands)?,
            },
            "vl8re32.v" => Vlrv {
                nf: 8,
                eew: SEW::E32,
                data: vlr(operands)?,
            },
            "vl8re64.v" => Vlrv {
                nf: 8,
                eew: SEW::E64,
                data: vlr(operands)?,
            },

            "vs1r.v" => Vsrv {
                nf: 1,
                data: vsr(operands)?,
            },
            "vs2r.v" => Vsrv {
                nf: 2,
                data: vsr(operands)?,
            },
            "vs4r.v" => Vsrv {
                nf: 4,
                data: vsr(operands)?,
            },
            "vs8r.v" => Vsrv {
                nf: 8,
                data: vsr(operands)?,
            },

            "vadd.vv" => Vaddvv(opivv(operands)?),
            "vadd.vx" => Vaddvx(opivx(operands)?),
            "vadd.vi" => Vaddvi(opivi(operands)?),

            "vsub.vv" => Vsubvv(opivv(operands)?),
            "vsub.vx" => Vsubvx(opivx(operands)?),

            "vrsub.vx" => Vrsubvx(opivx(operands)?),
            "vrsub.vi" => Vrsubvi(opivi(operands)?),

            "vminu.vv" => Vminuvv(opivv(operands)?),
            "vminu.vx" => Vminuvx(opivx(operands)?),

            "vmin.vv" => Vminvv(opivv(operands)?),
            "vmin.vx" => Vminvx(opivx(operands)?),

            "vmaxu.vv" => Vmaxuvv(opivv(operands)?),
            "vmaxu.vx" => Vmaxuvx(opivx(operands)?),

            "vmax.vv" => Vmaxvv(opivv(operands)?),
            "vmax.vx" => Vmaxvx(opivx(operands)?),

            "vand.vv" => Vandvv(opivv(operands)?),
            "vand.vx" => Vandvx(opivx(operands)?),
            "vand.vi" => Vandvi(opivi(operands)?),

            "vor.vv" => Vorvv(opivv(operands)?),
            "vor.vx" => Vorvx(opivx(operands)?),
            "vor.vi" => Vorvi(opivi(operands)?),

            "vxor.vv" => Vxorvv(opivv(operands)?),
            "vxor.vx" => Vxorvx(opivx(operands)?),
            "vxor.vi" => Vxorvi(opivi(operands)?),

            "vrgather.vv" => Vrgathervv(opivv(operands)?),
            "vrgather.vx" => Vrgathervx(opivx(operands)?),
            "vrgather.vi" => Vrgathervi(opivi(operands)?),

            "vrgatherei16.v" => Vrgatherei16vv(opivv(operands)?),

            "vslideup.vx" => Vslideupvx(opivx(operands)?),
            "vslideup.vi" => Vslideupvi(opivi(operands)?),

            "vslidedown.vx" => Vslidedownvx(opivx(operands)?),
            "vslidedown.vi" => Vslidedownvi(opivi(operands)?),

            "vadc.vvm" => Vadcvvm(opivv_v0(operands)?),
            "vadc.vxm" => Vadcvxm(opivx_v0(operands)?),
            "vadc.vim" => Vadcvim(opivi_v0(operands)?),

            "vmadc.vvm" => Vmadcvvm(opivv_v0(operands)?),
            "vmadc.vxm" => Vmadcvxm(opivx_v0(operands)?),
            "vmadc.vim" => Vmadcvim(opivi_v0(operands)?),
            "vmadc.vv" => Vmadcvv(opivv_maskless(operands)?),
            "vmadc.vx" => Vmadcvx(opivx_maskless(operands)?),
            "vmadc.vi" => Vmadcvi(opivi_maskless(operands)?),

            "vsbc.vvm" => Vsbcvvm(opivv(operands)?),
            "vsbc.vxm" => Vsbcvxm(opivx(operands)?),

            "vmsbc.vv" => Vmsbcvv(opivv(operands)?),
            "vmsbc.vx" => Vmsbcvx(opivx(operands)?),

            "vmerge.vvm" => Vmergevvm(opivv(operands)?),
            "vmerge.vxm" => Vmergevxm(opivx(operands)?),
            "vmerge.vim" => Vmergevim(opivi(operands)?),

            "vmv.v.v" => Vmvvv(opivv_vmv(operands)?),
            "vmv.v.x" => Vmvvx(opivx_vmv(operands)?),
            "vmv.v.i" => Vmvvi(opivi_vmv(operands)?),

            "vmseq.vv" => Vmseqvv(opivv(operands)?),
            "vmseq.vx" => Vmseqvx(opivx(operands)?),
            "vmseq.vi" => Vmseqvi(opivi(operands)?),

            "vmsne.vv" => Vmsnevv(opivv(operands)?),
            "vmsne.vx" => Vmsnevx(opivx(operands)?),
            "vmsne.vi" => Vmsnevi(opivi(operands)?),

            "vmsltu.vv" => Vmsltuvv(opivv(operands)?),
            "vmsltu.vx" => Vmsltuvx(opivx(operands)?),

            "vmslt.vv" => Vmsltvv(opivv(operands)?),
            "vmslt.vx" => Vmsltvx(opivx(operands)?),

            "vmsleu.vv" => Vmsleuvv(opivv(operands)?),
            "vmsleu.vx" => Vmsleuvx(opivx(operands)?),
            "vmsleu.vi" => Vmsleuvi(opivi(operands)?),

            "vmsle.vv" => Vmslevv(opivv(operands)?),
            "vmsle.vx" => Vmslevx(opivx(operands)?),
            "vmsle.vi" => Vmslevi(opivi(operands)?),

            "vmsgtu.vx" => Vmsgtuvx(opivx(operands)?),
            "vmsgtu.vi" => Vmsgtuvi(opivi(operands)?),

            "vmsgt.vx" => Vmsgtvx(opivx(operands)?),
            "vmsgt.vi" => Vmsgtvi(opivi(operands)?),

            "vsaddu.vv" => Vsadduvv(opivv(operands)?),
            "vsaddu.vx" => Vsadduvx(opivx(operands)?),
            "vsaddu.vi" => Vsadduvi(opivi(operands)?),

            "vsadd.vv" => Vsaddvv(opivv(operands)?),
            "vsadd.vx" => Vsaddvx(opivx(operands)?),
            "vsadd.vi" => Vsaddvi(opivi(operands)?),

            "vssubu.vv" => Vssubuvv(opivv(operands)?),
            "vssubu.vx" => Vssubuvx(opivx(operands)?),

            "vssub.vv" => Vssubvv(opivv(operands)?),
            "vssub.vx" => Vssubvx(opivx(operands)?),

            "vsll.vv" => Vsllvv(opivv(operands)?),
            "vsll.vx" => Vsllvx(opivx(operands)?),
            "vsll.vi" => Vsllvi(opivi(operands)?),

            "vsmul.vv" => Vsmulvv(opivv(operands)?),
            "vsmul.vx" => Vsmulvx(opivx(operands)?),

            "vmv1r.v" => Vmv1rv(opivi(operands)?),
            "vmv2r.v" => Vmv2rv(opivi(operands)?),
            "vmv4r.v" => Vmv4rv(opivi(operands)?),
            "vmv8r.v" => Vmv8rv(opivi(operands)?),

            "vsrl.vv" => Vsrlvv(opivv(operands)?),
            "vsrl.vx" => Vsrlvx(opivx(operands)?),
            "vsrl.vi" => Vsrlvi(opivi(operands)?),

            "vsra.vv" => Vsravv(opivv(operands)?),
            "vsra.vx" => Vsravx(opivx(operands)?),
            "vsra.vi" => Vsravi(opivi(operands)?),

            "vssrl.vv" => Vssrlvv(opivv(operands)?),
            "vssrl.vx" => Vssrlvx(opivx(operands)?),
            "vssrl.vi" => Vssrlvi(opivi(operands)?),

            "vssra.vv" => Vssravv(opivv(operands)?),
            "vssra.vx" => Vssravx(opivx(operands)?),
            "vssra.vi" => Vssravi(opivi(operands)?),

            "vnsrl.wv" => Vnsrlwv(opivv(operands)?),
            "vnsrl.wx" => Vnsrlwx(opivx(operands)?),
            "vnsrl.wi" => Vnsrlwi(opivi(operands)?),

            "vnsra.wv" => Vnsrawv(opivv(operands)?),
            "vnsra.wx" => Vnsrawx(opivx(operands)?),
            "vnsra.wi" => Vnsrawi(opivi(operands)?),

            "vnclipu.wv" => Vnclipuwv(opivv(operands)?),
            "vnclipu.wx" => Vnclipuwx(opivx(operands)?),
            "vnclipu.wi" => Vnclipuwi(opivi(operands)?),

            "vnclip.wv" => Vnclipwv(opivv(operands)?),
            "vnclip.wx" => Vnclipwx(opivx(operands)?),
            "vnclip.wi" => Vnclipwi(opivi(operands)?),

            "vwredsumu.vs" => Vwredsumuvs(opivv(operands)?),
            "vwredsum.vs" => Vwredsumvs(opivv(operands)?),

            "vredsum.vs" => Vredsumvs(opmvv(operands)?),
            "vredand.vs" => Vredandvs(opmvv(operands)?),
            "vredor.vs" => Vredorvs(opmvv(operands)?),
            "vredxor.vs" => Vredxorvs(opmvv(operands)?),
            "vredminu.vs" => Vredminuvs(opmvv(operands)?),
            "vredmin.vs" => Vredminvs(opmvv(operands)?),
            "vredmaxu.vs" => Vredmaxuvs(opmvv(operands)?),
            "vredmax.vs" => Vredmaxvs(opmvv(operands)?),

            "vaaddu.vv" => Vaadduvv(opmvv(operands)?),
            "vaaddu.vx" => Vaadduvx(opmvx(operands)?),

            "vaadd.vv" => Vaaddvv(opmvv(operands)?),
            "vaadd.vx" => Vaaddvx(opmvx(operands)?),

            "vasubu.vv" => Vasubuvv(opmvv(operands)?),
            "vasubu.vx" => Vasubuvx(opmvx(operands)?),

            "vasub.vv" => Vasubvv(opmvv(operands)?),
            "vasub.vx" => Vasubvx(opmvx(operands)?),

            "vslide1up.vx" => Vslide1upvx(opmvx(operands)?),

            "vslide1down.vx" => Vslide1downvx(opmvx(operands)?),

            "vmv.x.s" => Vmvxs(vmvxs(operands)?),
            "vcpop.m" => Vcpopm(vwxunary0(operands)?),
            "vfirst.m" => Vfirstm(vwxunary0(operands)?),

            "vmv.s.x" => Vmvsx(vrxunary0(operands)?),

            "vsext.vf2" => Vsextvf2(vxunary0(operands)?),
            "vsext.vf4" => Vsextvf4(vxunary0(operands)?),
            "vsext.vf8" => Vsextvf8(vxunary0(operands)?),

            "vzext.vf2" => Vzextvf2(vxunary0(operands)?),
            "vzext.vf4" => Vzextvf4(vxunary0(operands)?),
            "vzext.vf8" => Vzextvf8(vxunary0(operands)?),

            "vmsbf.m" => Vmsbfm(vmunary0(operands)?),
            "vmsof.m" => Vmsofm(vmunary0(operands)?),
            "vmsif.m" => Vmsifm(vmunary0(operands)?),
            "viota.m" => Viotam(vmunary0(operands)?),
            "vid.v" => Vidv(vidv(operands)?),

            "vcompress.vm" => Vcompressvm(opmvv_maskless(operands)?),

            "vmandn.mm" => Vmandnmm(opmvv_maskless(operands)?),
            "vmand.mm" => Vmandmm(opmvv_maskless(operands)?),
            "vmor.mm" => Vmormm(opmvv_maskless(operands)?),
            "vmxor.mm" => Vmxormm(opmvv_maskless(operands)?),
            "vmorn.mm" => Vmornmm(opmvv_maskless(operands)?),
            "vmnand.mm" => Vmnandmm(opmvv_maskless(operands)?),
            "vmnor.mm" => Vmnormm(opmvv_maskless(operands)?),
            "vmxnor.mm" => Vmxnormm(opmvv_maskless(operands)?),

            "vdivu.vv" => Vdivuvv(opmvv(operands)?),
            "vdivu.vx" => Vdivuvx(opmvx(operands)?),

            "vdiv.vv" => Vdivvv(opmvv(operands)?),
            "vdiv.vx" => Vdivvx(opmvx(operands)?),

            "vremu.vv" => Vremuvv(opmvv(operands)?),
            "vremu.vx" => Vremuvx(opmvx(operands)?),

            "vrem.vv" => Vremvv(opmvv(operands)?),
            "vrem.vx" => Vremvx(opmvx(operands)?),

            "vmulhu.vv" => Vmulhuvv(opmvv(operands)?),
            "vmulhu.vx" => Vmulhuvx(opmvx(operands)?),

            "vmul.vv" => Vmulvv(opmvv(operands)?),
            "vmul.vx" => Vmulvx(opmvx(operands)?),

            "vmulhsu.vv" => Vmulhsuvv(opmvv(operands)?),
            "vmulhsu.vx" => Vmulhsuvx(opmvx(operands)?),

            "vmulh.vv" => Vmulhvv(opmvv(operands)?),
            "vmulh.vx" => Vmulhvx(opmvx(operands)?),

            "vmadd.vv" => Vmaddvv(opmvv_fma(operands)?),
            "vmadd.vx" => Vmaddvx(opmvx_fma(operands)?),

            "vnmsub.vv" => Vnmsubvv(opmvv_fma(operands)?),
            "vnmsub.vx" => Vnmsubvx(opmvx_fma(operands)?),

            "vmacc.vv" => Vmaccvv(opmvv_fma(operands)?),
            "vmacc.vx" => Vmaccvx(opmvx_fma(operands)?),

            "vnmsac.vv" => Vnmsacvv(opmvv_fma(operands)?),
            "vnmsac.vx" => Vnmsacvx(opmvx_fma(operands)?),

            "vwaddu.vv" => Vwadduvv(opmvv(operands)?),
            "vwaddu.vx" => Vwadduvx(opmvx(operands)?),

            "vwadd.vv" => Vwaddvv(opmvv(operands)?),
            "vwadd.vx" => Vwaddvx(opmvx(operands)?),

            "vwsubu.vv" => Vwsubuvv(opmvv(operands)?),
            "vwsubu.vx" => Vwsubuvx(opmvx(operands)?),

            "vwsub.vv" => Vwsubvv(opmvv(operands)?),
            "vwsub.vx" => Vwsubvx(opmvx(operands)?),

            "vwaddu.wv" => Vwadduwv(opmvv(operands)?),
            "vwaddu.wx" => Vwadduwx(opmvx(operands)?),

            "vwadd.wv" => Vwaddwv(opmvv(operands)?),
            "vwadd.wx" => Vwaddwx(opmvx(operands)?),

            "vwsubu.wv" => Vwsubuwv(opmvv(operands)?),
            "vwsubu.wx" => Vwsubuwx(opmvx(operands)?),

            "vwsub.wv" => Vwsubwv(opmvv(operands)?),
            "vwsub.wx" => Vwsubwx(opmvx(operands)?),

            "vwmulu.vv" => Vwmuluvv(opmvv(operands)?),
            "vwmulu.vx" => Vwmuluvx(opmvx(operands)?),

            "vwmulsu.vv" => Vwmulsuvv(opmvv(operands)?),
            "vwmulsu.vx" => Vwmulsuvx(opmvx(operands)?),

            "vwmul.vv" => Vwmulvv(opmvv(operands)?),
            "vwmul.vx" => Vwmulvx(opmvx(operands)?),

            "vwmaccu.vv" => Vwmaccuvv(opmvv_fma(operands)?),
            "vwmaccu.vx" => Vwmaccuvx(opmvx_fma(operands)?),

            "vwmacc.vv" => Vwmaccvv(opmvv_fma(operands)?),
            "vwmacc.vx" => Vwmaccvx(opmvx_fma(operands)?),

            "vwmaccus.vx" => Vwmaccusvx(opmvx_fma(operands)?),

            "vwmaccsu.vv" => Vwmaccsuvv(opmvv_fma(operands)?),
            "vwmaccsu.vx" => Vwmaccsuvx(opmvx_fma(operands)?),

            "vfadd.vv" => Vfaddvv(opfvv(operands)?),
            "vfadd.vf" => Vfaddvf(opfvf(operands)?),

            "vfredusum.vs" => Vfredusumvs(opfvv(operands)?),

            "vfsub.vv" => Vfsubvv(opfvv(operands)?),
            "vfsub.vf" => Vfsubvf(opfvf(operands)?),

            "vfredosum.vs" => Vfredosumvs(opfvv(operands)?),

            "vfmin.vv" => Vfminvv(opfvv(operands)?),
            "vfmin.vf" => Vfminvf(opfvf(operands)?),

            "vfredmin.vs" => Vfredminvs(opfvv(operands)?),

            "vfmax.vv" => Vfmaxvv(opfvv(operands)?),
            "vfmax.vf" => Vfmaxvf(opfvf(operands)?),

            "vfredmax.vs" => Vfredmaxvs(opfvv(operands)?),

            "vfsgnj.vv" => Vfsgnjvv(opfvv(operands)?),
            "vfsgnj.vf" => Vfsgnjvf(opfvf(operands)?),

            "vfsgnjn.vv" => Vfsgnjnvv(opfvv(operands)?),
            "vfsgnjn.vf" => Vfsgnjnvf(opfvf(operands)?),

            "vfsgnjx.vv" => Vfsgnjxvv(opfvv(operands)?),
            "vfsgnjx.vf" => Vfsgnjxvf(opfvf(operands)?),

            "vfslide1up.vf" => Vfslide1upvf(opfvf(operands)?),

            "vfslide1down.vf" => Vfslide1downvf(opfvf(operands)?),

            "vfmv.f.s" => Vfmvfs(vwfunary0(operands)?),

            "vfmv.s.f" => Vfmvsf(vrfunary0(operands)?),

            "vfcvt.xu.f.v" => Vfcvtxufv(vfunary0(operands)?),
            "vfcvt.x.f.v" => Vfcvtxfv(vfunary0(operands)?),
            "vfcvt.f.xu.v" => Vfcvtfxuv(vfunary0(operands)?),
            "vfcvt.f.x.v" => Vfcvtfxv(vfunary0(operands)?),
            "vfcvt.rtz.xu.f.v" => VfcvtRtzxufv(vfunary0(operands)?),
            "vfcvt.rtz.x.f.v" => VfcvtRtzxfv(vfunary0(operands)?),

            "vfwcvt.xu.f.v" => Vfwcvtxufv(vfunary0(operands)?),
            "vfwcvt.x.f.v" => Vfwcvtxfv(vfunary0(operands)?),
            "vfwcvt.f.xu.v" => Vfwcvtfxuv(vfunary0(operands)?),
            "vfwcvt.f.x.v" => Vfwcvtfxv(vfunary0(operands)?),
            "vfwcvt.f.f.v" => Vfwcvtffv(vfunary0(operands)?),
            "vfwcvt.rtz.xu.f.v" => VfwcvtRtzxufv(vfunary0(operands)?),
            "vfwcvt.rtz.x.f.v" => VfwcvtRtzxfv(vfunary0(operands)?),

            "vfncvt.xu.f.w" => Vfncvtxufw(vfunary0(operands)?),
            "vfncvt.x.f.w" => Vfncvtxfw(vfunary0(operands)?),
            "vfncvt.f.xu.w" => Vfncvtfxuw(vfunary0(operands)?),
            "vfncvt.f.x.w" => Vfncvtfxw(vfunary0(operands)?),
            "vfncvt.f.f.w" => Vfncvtffw(vfunary0(operands)?),
            "vfncvt.rod.f.f.w" => VfncvtRodffw(vfunary0(operands)?),
            "vfncvt.rtz.xu.f.w" => VfncvtRtzxufw(vfunary0(operands)?),
            "vfncvt.rtz.x.f.w" => VfncvtRtzxfw(vfunary0(operands)?),

            "vfsqrt.v" => Vfsqrtv(vfunary1(operands)?),
            "vfrsqrt7.v" => Vfrsqrt7v(vfunary1(operands)?),
            "vfrec7.v" => Vfrec7v(vfunary1(operands)?),
            "vfclass.v" => Vfclassv(vfunary1(operands)?),

            "vfmerge.vfm" => Vfmergevfm(opfvf(operands)?),
            "vfmv.v.f" => Vfmvvf(opfvf(operands)?),

            "vmfeq.vv" => Vmfeqvv(opfvv(operands)?),
            "vmfeq.vf" => Vmfeqvf(opfvf(operands)?),

            "vmfle.vv" => Vmflevv(opfvv(operands)?),
            "vmfle.vf" => Vmflevf(opfvf(operands)?),

            "vmflt.vv" => Vmfltvv(opfvv(operands)?),
            "vmflt.vf" => Vmfltvf(opfvf(operands)?),

            "vmfne.vv" => Vmfnevv(opfvv(operands)?),
            "vmfne.vf" => Vmfnevf(opfvf(operands)?),

            "vmfgt.vf" => Vmfgtvf(opfvf(operands)?),

            "vmfge.vf" => Vmfgevf(opfvf(operands)?),

            "vfdiv.vv" => Vfdivvv(opfvv(operands)?),
            "vfdiv.vf" => Vfdivvf(opfvf(operands)?),

            "vfrdiv.vf" => Vfrdivvf(opfvf(operands)?),

            "vfmul.vv" => Vfmulvv(opfvv(operands)?),
            "vfmul.vf" => Vfmulvf(opfvf(operands)?),

            "vfrsub.vf" => Vfrsubvf(opfvf(operands)?),

            "vfmadd.vv" => Vfmaddvv(opfvv_fma(operands)?),
            "vfmadd.vf" => Vfmaddvf(opfvf_fma(operands)?),

            "vfnmadd.vv" => Vfnmaddvv(opfvv_fma(operands)?),
            "vfnmadd.vf" => Vfnmaddvf(opfvf_fma(operands)?),

            "vfmsub.vv" => Vfmsubvv(opfvv_fma(operands)?),
            "vfmsub.vf" => Vfmsubvf(opfvf_fma(operands)?),

            "vfnmsub.vv" => Vfnmsubvv(opfvv_fma(operands)?),
            "vfnmsub.vf" => Vfnmsubvf(opfvf_fma(operands)?),

            "vfmacc.vv" => Vfmaccvv(opfvv_fma(operands)?),
            "vfmacc.vf" => Vfmaccvf(opfvf_fma(operands)?),

            "vfnmacc.vv" => Vfnmaccvv(opfvv_fma(operands)?),
            "vfnmacc.vf" => Vfnmaccvf(opfvf_fma(operands)?),

            "vfmsac.vv" => Vfmsacvv(opfvv_fma(operands)?),
            "vfmsac.vf" => Vfmsacvf(opfvf_fma(operands)?),

            "vfnmsac.vv" => Vfnmsacvv(opfvv_fma(operands)?),
            "vfnmsac.vf" => Vfnmsacvf(opfvf_fma(operands)?),

            "vfwadd.vv" => Vfwaddvv(opfvv(operands)?),
            "vfwadd.vf" => Vfwaddvf(opfvf(operands)?),

            "vfwredusum.vs" => Vfwredusumvs(opfvv(operands)?),

            "vfwsub.vv" => Vfwsubvv(opfvv(operands)?),
            "vfwsub.vf" => Vfwsubvf(opfvf(operands)?),

            "vfwredosum.vs" => Vfwredosumvs(opfvv(operands)?),

            "vfwadd.wv" => Vfwaddwv(opfvv(operands)?),
            "vfwadd.wf" => Vfwaddwf(opfvf(operands)?),

            "vfwsub.wv" => Vfwsubwv(opfvv(operands)?),
            "vfwsub.wf" => Vfwsubwf(opfvf(operands)?),

            "vfwmul.vv" => Vfwmulvv(opfvv(operands)?),
            "vfwmul.vf" => Vfwmulvf(opfvf(operands)?),

            "vfwmacc.vv" => Vfwmaccvv(opfvv_fma(operands)?),
            "vfwmacc.vf" => Vfwmaccvf(opfvf_fma(operands)?),

            "vfwnmacc.vv" => Vfwnmaccvv(opfvv_fma(operands)?),
            "vfwnmacc.vf" => Vfwnmaccvf(opfvf_fma(operands)?),

            "vfwmsac.vv" => Vfwmsacvv(opfvv_fma(operands)?),
            "vfwmsac.vf" => Vfwmsacvf(opfvf_fma(operands)?),

            "vfwnmsac.vv" => Vfwnmsacvv(opfvv_fma(operands)?),
            "vfwnmsac.vf" => Vfwnmsacvf(opfvf_fma(operands)?),

            // Pseudoinstructions
            "nop" => Addi(I {
                rd: 0,
                rs1: 0,
                imm12: 0,
            }),
            "li" => {
                let (reg, imm) = integer::pseudo::parse_op_imm_format(operands)?;

                match imm.cmp(&4096) {
                    Ordering::Less => Addi(I {
                        rd: reg,
                        rs1: 0,
                        imm12: imm,
                    }),
                    Ordering::Equal => Lui(U {
                        rd: reg,
                        imm20: imm,
                    }),
                    Ordering::Greater => Fusion(
                        Box::new(Lui(U {
                            rd: reg,
                            imm20: imm >> 12,
                        })),
                        Box::new(Addi(I {
                            rd: reg,
                            rs1: reg,
                            imm12: imm & 0xfff,
                        })),
                    ),
                }
            }
            "mv" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(operands)?;
                Addi(I { rd, rs1, imm12: 0 })
            }
            "not" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(operands)?;
                Xori(I { rd, rs1, imm12: -1 })
            }
            "neg" => {
                let (rd, rs2) = integer::pseudo::parse_op_op_format(operands)?;
                Sub(R { rd, rs1: 0, rs2 })
            }
            "negw" => {
                let (rd, rs2) = integer::pseudo::parse_op_op_format(operands)?;
                Subw(R { rd, rs1: 0, rs2 })
            }
            "sext.b" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(operands)?;
                Fusion(
                    Box::new(Slli(I {
                        rd,
                        rs1,
                        imm12: 64 - 8,
                    })),
                    Box::new(Srai(I {
                        rd,
                        rs1,
                        imm12: 64 - 8,
                    })),
                )
            }
            "sext.h" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(operands)?;
                Fusion(
                    Box::new(Slli(I {
                        rd,
                        rs1,
                        imm12: 64 - 16,
                    })),
                    Box::new(Srai(I {
                        rd,
                        rs1,
                        imm12: 64 - 16,
                    })),
                )
            }
            "sext.w" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(operands)?;
                Addiw(I { rd, rs1, imm12: 0 })
            }
            "zext.b" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(operands)?;
                Andi(I {
                    rd,
                    rs1,
                    imm12: 0xff,
                })
            }
            "zext.h" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(operands)?;
                Fusion(
                    Box::new(Slli(I {
                        rd,
                        rs1,
                        imm12: 64 - 16,
                    })),
                    Box::new(Srli(I {
                        rd,
                        rs1: rd,
                        imm12: 64 - 16,
                    })),
                )
            }
            "zext.w" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(operands)?;
                Fusion(
                    Box::new(Slli(I {
                        rd,
                        rs1,
                        imm12: 64 - 32,
                    })),
                    Box::new(Srli(I {
                        rd,
                        rs1: rd,
                        imm12: 64 - 32,
                    })),
                )
            }
            "seqz" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(operands)?;
                Sltiu(I { rd, rs1, imm12: 1 })
            }
            "snez" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(operands)?;
                Sltu(R { rd, rs1, rs2: 0 })
            }
            "sltz" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(operands)?;
                Slt(R { rd, rs1, rs2: 0 })
            }
            "sgtz" => {
                let (rd, rs1) = integer::pseudo::parse_op_op_format(operands)?;
                Slt(R {
                    rd,
                    rs1: 0,
                    rs2: rs1,
                })
            }
            "fmv.s" => {
                let (rd, rs1) = float::pseudo::parse_op_op_format(operands)?;
                Fsgnjs(R { rd, rs1, rs2: rs1 })
            }
            "fabs.s" => {
                let (rd, rs1) = float::pseudo::parse_op_op_format(operands)?;
                Fsgnjxs(R { rd, rs1, rs2: rs1 })
            }
            "fneg.s" => {
                let (rd, rs1) = float::pseudo::parse_op_op_format(operands)?;
                Fsgnjns(R { rd, rs1, rs2: rs1 })
            }
            "fmv.d" => {
                let (rd, rs1) = float::pseudo::parse_op_op_format(operands)?;
                Fsgnjd(R { rd, rs1, rs2: rs1 })
            }
            "fabs.d" => {
                let (rd, rs1) = float::pseudo::parse_op_op_format(operands)?;
                Fsgnjxd(R { rd, rs1, rs2: rs1 })
            }
            "fneg.d" => {
                let (rd, rs1) = float::pseudo::parse_op_op_format(operands)?;
                Fsgnjnd(R { rd, rs1, rs2: rs1 })
            }
            "beqz" => {
                let (rs1, diff) =
                    integer::pseudo::parse_op_label_format(operands, labels, current_address)?;
                Beq(S {
                    rs1,
                    rs2: 0,
                    imm12: diff,
                })
            }
            "bnez" => {
                let (rs1, diff) =
                    integer::pseudo::parse_op_label_format(operands, labels, current_address)?;
                Bne(S {
                    rs1,
                    rs2: 0,
                    imm12: diff,
                })
            }
            "blez" => {
                let (rs1, diff) =
                    integer::pseudo::parse_op_label_format(operands, labels, current_address)?;
                Bge(S {
                    rs1,
                    rs2: 0,
                    imm12: diff,
                })
            }
            "bgez" => {
                let (rs1, diff) =
                    integer::pseudo::parse_op_label_format(operands, labels, current_address)?;
                Bge(S {
                    rs1,
                    rs2: 0,
                    imm12: diff,
                })
            }
            "bltz" => {
                let (rs1, diff) =
                    integer::pseudo::parse_op_label_format(operands, labels, current_address)?;
                Blt(S {
                    rs1,
                    rs2: 0,
                    imm12: diff,
                })
            }
            "bgtz" => {
                let (rs1, diff) =
                    integer::pseudo::parse_op_label_format(operands, labels, current_address)?;
                Blt(S {
                    rs1,
                    rs2: 0,
                    imm12: diff,
                })
            }
            "bgt" => {
                let (rs1, rs2, diff) =
                    integer::pseudo::parse_op_op_label_format(operands, labels, current_address)?;
                Blt(S {
                    rs1: rs2,
                    rs2: rs1,
                    imm12: diff,
                })
            }
            "ble" => {
                let (rs1, rs2, diff) =
                    integer::pseudo::parse_op_op_label_format(operands, labels, current_address)?;
                Bge(S {
                    rs1: rs2,
                    rs2: rs1,
                    imm12: diff,
                })
            }
            "bgtu" => {
                let (rs1, rs2, diff) =
                    integer::pseudo::parse_op_op_label_format(operands, labels, current_address)?;
                Bltu(S {
                    rs1: rs2,
                    rs2: rs1,
                    imm12: diff,
                })
            }
            "bleu" => {
                let (rs1, rs2, diff) =
                    integer::pseudo::parse_op_op_label_format(operands, labels, current_address)?;
                Bgeu(S {
                    rs1: rs2,
                    rs2: rs1,
                    imm12: diff,
                })
            }
            "j" => {
                let diff = integer::pseudo::parse_label_format(operands, labels, current_address)?;
                Jal(U { rd: 0, imm20: diff })
            }
            "jr" => {
                let rs1 = integer::pseudo::parse_op_format(operands)?;
                Jalr(I {
                    rd: 0,
                    rs1,
                    imm12: 0,
                })
            }
            "ret" => Jalr(I {
                rd: 0,
                rs1: 1,
                imm12: 0,
            }),
            "call" => {
                let diff = integer::pseudo::parse_label_format(operands, labels, current_address)?;
                Fusion(
                    Box::new(Auipc(U {
                        rd: 1,
                        imm20: diff >> 12,
                    })),
                    Box::new(Jalr(I {
                        rd: 1,
                        rs1: 1,
                        imm12: (diff - 4) & 0xfff,
                    })),
                )
            }
            "tail" => {
                let diff = integer::pseudo::parse_label_format(operands, labels, current_address)?;
                Fusion(
                    Box::new(Auipc(U {
                        rd: 6,
                        imm20: diff >> 12,
                    })),
                    Box::new(Jalr(I {
                        rd: 0,
                        rs1: 6,
                        imm12: (diff - 4) & 0xfff,
                    })),
                )
            }
            "rdinstret" => {
                let rd = integer::pseudo::parse_op_format(operands)?;
                Csrrs(Csrr {
                    rd,
                    csr: alias::INSTRET,
                    rs1: 0,
                })
            }
            "rdinstreth" => {
                let rd = integer::pseudo::parse_op_format(operands)?;
                Csrrs(Csrr {
                    rd,
                    csr: alias::INSTRETH,
                    rs1: 0,
                })
            }
            "rdcycle" => {
                let rd = integer::pseudo::parse_op_format(operands)?;
                Csrrs(Csrr {
                    rd,
                    csr: alias::CYCLE,
                    rs1: 0,
                })
            }
            "rdcycleh" => {
                let rd = integer::pseudo::parse_op_format(operands)?;
                Csrrs(Csrr {
                    rd,
                    csr: alias::CYCLEH,
                    rs1: 0,
                })
            }
            "csrr" => {
                let (rd, csr) = csr::pseudo::parse_op_csr_format(operands)?;
                Csrrs(Csrr { rd, csr, rs1: 0 })
            }
            "csrw" => {
                let (csr, rs1) = csr::pseudo::parse_csr_op_format(operands)?;
                Csrrw(Csrr { rd: 0, csr, rs1 })
            }
            "csrs" => {
                let (csr, rs1) = csr::pseudo::parse_csr_op_format(operands)?;
                Csrrs(Csrr { rd: 0, csr, rs1 })
            }
            "csrc" => {
                let (csr, rs1) = csr::pseudo::parse_csr_op_format(operands)?;
                Csrrc(Csrr { rd: 0, csr, rs1 })
            }
            "frcsr" => {
                let rd = integer::pseudo::parse_op_format(operands)?;
                Csrrs(Csrr {
                    rd,
                    csr: alias::FCSR,
                    rs1: 0,
                })
            }
            "fscsr" => {
                match integer::pseudo::parse_op_op_format(operands) {
                    Ok((rd, rs1)) => Csrrw(Csrr {
                        rd,
                        csr: alias::FCSR,
                        rs1,
                    }),
                    Err(fst_err) => match integer::pseudo::parse_op_format(operands) {
                        Ok(rs) => Csrrw(Csrr {
                            rd: 0,
                            csr: alias::FCSR,
                            rs1: rs,
                        }),
                        Err(snd_err) => return Err(format!("{} or {}", fst_err, snd_err))
                    }
                }
            }
            "frrm" => {
                let rd = integer::pseudo::parse_op_format(operands)?;
                Csrrs(Csrr {
                    rd,
                    csr: alias::FRM,
                    rs1: 0,
                })
            }
            "fsrm" => {
                match integer::pseudo::parse_op_op_format(operands) {
                    Ok((rd, rs1)) => Csrrw(Csrr {
                        rd,
                        csr: alias::FRM,
                        rs1,
                    }),
                    Err(fst_err) => match integer::pseudo::parse_op_format(operands) {
                        Ok(rs) => Csrrw(Csrr {
                            rd: 0,
                            csr: alias::FRM,
                            rs1: rs,
                        }),
                        Err(snd_err) => return Err(format!("{} or {}", fst_err, snd_err))
                    }
                }
            }
            "fsrmi" => {
                match integer::pseudo::parse_op_imm_format(operands) {
                    Ok((rd, imm)) => Csrrwi(Csri {
                        rd,
                        csr: alias::FRM,
                        uimm: imm as u32 as usize,
                    }),
                    Err(fst_err) => match integer::pseudo::parse_imm_format(operands) {
                        Ok(imm) => Csrrwi(Csri {
                            rd: 0,
                            csr: alias::FRM,
                            uimm: imm as u32 as usize,
                        }),
                        Err(snd_err) => return Err(format!("{} or {}", fst_err, snd_err))
                    }
                }
            }
            "frflags" => {
                let rd = integer::pseudo::parse_op_format(operands)?;
                Csrrs(Csrr {
                    rd,
                    csr: alias::FFLAGS,
                    rs1: 0,
                })
            }
            "fsflags" => {
                match integer::pseudo::parse_op_op_format(operands) {
                    Ok((rd, rs1)) => Csrrw(Csrr {
                        rd,
                        csr: alias::FFLAGS,
                        rs1,
                    }),
                    Err(fst_err) => match integer::pseudo::parse_op_format(operands) {
                        Ok(rs1) => Csrrw(Csrr {
                            rd: 0,
                            csr: alias::FFLAGS,
                            rs1,
                        }),
                        Err(snd_err) => return Err(format!("{} or {}", fst_err, snd_err))
                    }
                }
            }
            "fsflagsi" => {
                match integer::pseudo::parse_op_imm_format(operands) {
                    Ok((rd, imm)) => Csrrwi(Csri {
                        rd,
                        csr: alias::FFLAGS,
                        uimm: imm as u32 as usize,
                    }),
                    Err(fst_err) => match integer::pseudo::parse_imm_format(operands) {
                        Ok(imm) => Csrrwi(Csri {
                            rd: 0,
                            csr: alias::FFLAGS,
                            uimm: imm as u32 as usize,
                        }),
                        Err(snd_err) => return Err(format!("{} or {}", fst_err, snd_err))
                    }
                }
            }

            "vneg.v" => {
                let (vd, vs2, vm) = vector::pseudo::parse_op_op_mask_format(operands)?;
                Vrsubvx(Opivx { vd, rs1: alias::ZERO, vs2, vm })
            }
            "vwcvt.x.x.v" => {
                let (vd, vs2, vm) = vector::pseudo::parse_op_op_mask_format(operands)?;
                Vwaddvx(Opmvx { dest: vd, rs1: alias::ZERO, vs2, vm })
            }
            "vwcvtu.x.x.v" => {
                let (vd, vs2, vm) = vector::pseudo::parse_op_op_mask_format(operands)?;
                Vwadduvx(Opmvx { dest: vd, rs1: alias::ZERO, vs2, vm })
            }
            "vnot.v" => {
                let (vd, vs2, vm) = vector::pseudo::parse_op_op_mask_format(operands)?;
                Vxorvi(Opivi { vd, imm5: -1, vs2, vm })
            }
            "vncvt.x.x.w" => {
                let (vd, vs2, vm) = vector::pseudo::parse_op_op_mask_format(operands)?;
                Vnsrlwx(Opivx { vd, rs1: alias::ZERO, vs2, vm })
            }
            "vmsgt.vv" => {
                let (vd, vs2, vs1, vm) = vector::pseudo::parse_op_op_op_mask_format(operands)?;
                Vmsltvv(Opivv { vd, vs1: vs2, vs2: vs1, vm })
            }
            "vmsgtu.vv" => {
                let (vd, vs2, vs1, vm) = vector::pseudo::parse_op_op_op_mask_format(operands)?;
                Vmsltuvv(Opivv { vd, vs1: vs2, vs2: vs1, vm })
            }
            "vmsge.vv" => {
                let (vd, vs2, vs1, vm) = vector::pseudo::parse_op_op_op_mask_format(operands)?;
                Vmslevv(Opivv { vd, vs1: vs2, vs2: vs1, vm })
            }
            "vmsgeu.vv" => {
                let (vd, vs2, vs1, vm) = vector::pseudo::parse_op_op_op_mask_format(operands)?;
                Vmsleuvv(Opivv { vd, vs1: vs2, vs2: vs1, vm })
            }
            "vmslt.vi" => {
                let (vd, vs2, imm, vm) = vector::pseudo::parse_op_op_imm_mask_format(operands)?;
                Vmslevi(Opivi { vd, imm5: imm - 1, vs2, vm })
            }
            "vmsltu.vi" => {
                let (vd, vs2, imm, vm) = vector::pseudo::parse_op_op_imm_mask_format(operands)?;
                Vmsleuvi(Opivi { vd, imm5: imm - 1, vs2, vm })
            }
            "vmsge.vi" => {
                let (vd, vs2, imm, vm) = vector::pseudo::parse_op_op_imm_mask_format(operands)?;
                Vmsgtvi(Opivi { vd, imm5: imm - 1, vs2, vm })
            }
            "vmsgeu.vi" => {
                let (vd, vs2, imm, vm) = vector::pseudo::parse_op_op_imm_mask_format(operands)?;
                Vmsgtuvi(Opivi { vd, imm5: imm - 1, vs2, vm })
            }
            "vmsge.vx" => {
                match vector::pseudo::parse_op_op_xreg_format(operands) {
                    Ok((vd, vs2, rs1)) => Fusion(
                        Box::new(Vmsltvx(Opivx { vd, rs1, vs2, vm: false })), 
                        Box::new(Vmnandmm(Opmvv { dest: vd, vs1: vd, vs2: vd, vm: false }))
                    ),
                    Err(fst_err) => match vector::pseudo::parse_op_op_xreg_mask_vd_nonzero_format(operands) {
                        Ok((vd, vs2, rs1)) => Fusion(
                            Box::new(Vmsltvx(Opivx { vd, rs1, vs2, vm: true })), 
                            Box::new(Vmxormm(Opmvv { dest: vd, vs1: 0, vs2: vd, vm: false }))
                        ),
                        Err(snd_err) => match vector::pseudo::parse_op_op_xreg_mask_temp_format(operands) {
                            Ok((vd, vs2, rs1, vt)) => if vd == 0 {
                                Fusion(
                                    Box::new(Vmsltvx(Opivx { vd: vt, rs1, vs2, vm: false })),
                                    Box::new(Vmandnmm(Opmvv { dest: vd, vs1: vt, vs2: vd, vm: false })), 
                                )
                            } else {
                                Fusion(
                                    Box::new(Vmsltvx(Opivx { vd: vt, rs1, vs2, vm: false })),
                                    Box::new(Fusion(
                                        Box::new(Vmandnmm(Opmvv { dest: vt, vs1: vt, vs2: 0, vm: false })), 
                                        Box::new(Fusion(
                                            Box::new(Vmandnmm(Opmvv { dest: vd, vs1: 0, vs2: vd, vm: false })),
                                            Box::new(Vmormm(Opmvv { dest: vd, vs1: vd, vs2: vt, vm: false }))
                                        ))
                                    )), 
                                )
                            }
                            Err(trd_err) => return Err(format!("{}, {} or {}", fst_err, snd_err, trd_err))
                        }
                    }
                }
            }
            "vmsgeu.vx" => {
                match vector::pseudo::parse_op_op_xreg_format(operands) {
                    Ok((vd, vs2, rs1)) => Fusion(
                        Box::new(Vmsltuvx(Opivx { vd, rs1, vs2, vm: false })), 
                        Box::new(Vmnandmm(Opmvv { dest: vd, vs1: vd, vs2: vd, vm: false }))
                    ),
                    Err(fst_err) => match vector::pseudo::parse_op_op_xreg_mask_vd_nonzero_format(operands) {
                        Ok((vd, vs2, rs1)) => Fusion(
                            Box::new(Vmsltuvx(Opivx { vd, rs1, vs2, vm: true })), 
                            Box::new(Vmxormm(Opmvv { dest: vd, vs1: 0, vs2: vd, vm: false }))
                        ),
                        Err(snd_err) => match vector::pseudo::parse_op_op_xreg_mask_temp_format(operands) {
                            Ok((vd, vs2, rs1, vt)) => if vd == 0 {
                                Fusion(
                                    Box::new(Vmsltuvx(Opivx { vd: vt, rs1, vs2, vm: false })),
                                    Box::new(Vmandnmm(Opmvv { dest: vd, vs1: vt, vs2: vd, vm: false })), 
                                )
                            } else {
                                Fusion(
                                    Box::new(Vmsltuvx(Opivx { vd: vt, rs1, vs2, vm: false })),
                                    Box::new(Fusion(
                                        Box::new(Vmandnmm(Opmvv { dest: vt, vs1: vt, vs2: 0, vm: false })), 
                                        Box::new(Fusion(
                                            Box::new(Vmandnmm(Opmvv { dest: vd, vs1: 0, vs2: vd, vm: false })),
                                            Box::new(Vmormm(Opmvv { dest: vd, vs1: vd, vs2: vt, vm: false }))
                                        ))
                                    )), 
                                )
                            }
                            Err(trd_err) => return Err(format!("{}, {} or {}", fst_err, snd_err, trd_err))
                        }
                    }
                }
            }
            "vfneg.v" => {
                let (vd, vs2, vm) = vector::pseudo::parse_op_op_mask_format(operands)?;
                Vfsgnjnvv(Opfvv { dest: vd, vs1: vs2, vs2, vm })
            }
            "vfabs.v" => {
                let (vd, vs2, vm) = vector::pseudo::parse_op_op_mask_format(operands)?;
                Vfsgnjxvv(Opfvv { dest: vd, vs1: vs2, vs2, vm })
            }
            "vmfgt.vv" => {
                let (vd, vs2, vs1, vm) = vector::pseudo::parse_op_op_op_mask_format(operands)?;
                Vmfltvv(Opfvv { dest: vd, vs1: vs2, vs2: vs1, vm })
            }
            "vmfge.vv" => {
                let (vd, vs2, vs1, vm) = vector::pseudo::parse_op_op_op_mask_format(operands)?;
                Vmflevv(Opfvv { dest: vd, vs1: vs2, vs2: vs1, vm })
            }
            "vmmv.m" => {
                let (vd, vs2) = vector::pseudo::parse_op_op_format(operands)?;
                Vmandmm(Opmvv { dest: vd, vs1: vs2, vs2, vm: false })
            }
            "vmclr.m" => {
                let vd = vector::pseudo::parse_op_format(operands)?;
                Vmxormm(Opmvv { dest: vd, vs1: vd, vs2: vd , vm: false })
            }
            "vmset.m" => {
                let vd = vector::pseudo::parse_op_format(operands)?;
                Vmxnormm(Opmvv { dest: vd, vs1: vd, vs2: vd , vm: false })
            }
            "vmnot.m" => {
                let (vd, vs2) = vector::pseudo::parse_op_op_format(operands)?;
                Vmnandmm(Opmvv { dest: vd, vs1: vs2, vs2, vm: false })
            }
            _ => return Err(format!("Unknown mnemonic: {}", mnemonic)),
        };

        Ok(instruction)
    }

    fn rename(old: &str) -> &str {
        match old {
            "vle1.v" => "vlm.v",
            "vse1.v" => "vsm.v",
            "vfredsum.vs" => "vfredusum.vs",
            "vfwredsum.vs" => "vfwredusum.vs",
            "vmandnot.mm" => "vmandn.mm",
            "vmornot.mm" => "vmorn.mm",
            "vpopc.m" => "vcpop.m",
            "vfrsqrte7.v" => "vfrsqrt7.v",
            "vfrece7.v" => "vfrec7.v",
            "vmcpy.m" => "vmmv.m",

            // Technically pseudoinstructions, but without custom parsers
            "vl1r.v" => "vl1re8.v",
            "vl2r.v" => "vl2re8.v",
            "vl4r.v" => "vl4re8.v",
            "vl8r.v" => "vl8re8.v",


            _ => old,
        }
    }

    fn split_instruction(instruction_line: &str) -> (&str, &str) {
        let mut lane = instruction_line.splitn(2, char::is_whitespace);
        let mnemonic = lane.next().unwrap_or_default().trim();
        let operands = lane.next().unwrap_or_default().trim();
        (mnemonic, operands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rename_correctly() {
        let old_mnemonics = [
            "vle1.v",
            "vse1.v",
            "vfredsum.vs",
            "vfwredsum.vs",
            "vmandnot.mm",
            "vmornot.mm",
            "vpopc.m",
            "vfrsqrte7.v",
            "vfrece7.v",
            "vmcpy.m",
        ];

        old_mnemonics
            .map(|mnemonic| (mnemonic, Decoder::rename(mnemonic)))
            .iter()
            .for_each(|(old, new)| assert_ne!(old, new));
    }
}
