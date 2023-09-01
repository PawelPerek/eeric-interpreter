mod operand;

use std::collections::HashMap;

use eeric::prelude::*;
use Instruction::*;
use operand::{integer, float, csr, vector};

pub struct Decoder;

pub enum LineClassification {
    Instruction(String),
    Label(String),
    Empty
}

impl Decoder {
    // TODO: proper numeric label support
    // See: https://docs.oracle.com/cd/E19120-01/open.solaris/817-5477/esqat/index.html
    pub fn classify(line: &str) -> LineClassification {
        let trimmed_line = line
            .split("#")
            .next()
            .unwrap_or("")
            .trim();

        if trimmed_line.is_empty() {
            LineClassification::Empty
        } else if trimmed_line.ends_with(":") {
            LineClassification::Label(trimmed_line[..trimmed_line.len() - 1].to_string())
        } else {
            LineClassification::Instruction(trimmed_line.to_string())
        }
    }

    pub fn decode(instruction_line: &str, labels: &HashMap<String, usize>, current_address: usize) -> Result<Instruction, String> {
        let (mnemonic, operands) = Self::split_instruction(instruction_line);

        use integer::{
            parse_r_format as r,
            parse_i_format as i,
            parse_load_format as l,
            parse_s_format as s,
            parse_branch_format as b,
            parse_u_format as u
        };

        use csr::{
            parse_csrr_format as csrr,
            parse_csri_format as csri
        };

        use float::parse_r4_format as r4;

        use vector:: {
            parse_vsetvli_format as vsetvli,
            parse_vsetivli_format as vsetivli,
            parse_vsetvl_format as vsetvl,
            
            parse_vl_format as vl,
            parse_vlm_format as vlm,
            parse_vs_format as vs,
            parse_vsm_format as vsm,
            parse_vls_format as vls,
            parse_vss_format as vss,
            parse_vlx_format as vlx,
            parse_vsx_format as vsx,
            parse_vlr_format as vlr,
            parse_vsr_format as vsr,

            parse_opivv_format as opivv,
            parse_opivx_format as opivx,
            parse_opivi_format as opivi,

            parse_opmvv_format as opmvv,
            parse_opmvx_format as opmvx,
            
            parse_vwxunary0_vmvxs_format as vmvxs,
            parse_vwxunary0_format as vwxunary0,
            parse_vrxunary0_format as vrxunary0,
            parse_vxunary0_format as vxunary0,
            parse_vmunary0_vidv_format as vidv,
            parse_vmunary0_format as vmunary0,

            parse_opfvv_format as opfvv,
            parse_opfvf_format as opfvf,

            parse_vwfunary0_format as vwfunary0,
            parse_vrfunary0_format as vrfunary0,
            parse_vfunary0_format as vfunary0,
            parse_vfunary1_format as vfunary1
        };

        // TODO: pseudo-instructions support

        let instruction = match mnemonic {
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

            "beq" => Beq(b(operands, &labels, current_address)?),
            "bne" => Bne(b(operands, &labels, current_address)?),
            "bge" => Bge(b(operands, &labels, current_address)?),
            "bgeu" => Bgeu(b(operands, &labels, current_address)?),
            "blt" => Blt(b(operands, &labels, current_address)?),
            "bltu" => Bltu(b(operands, &labels, current_address)?),
            "jal" => Jal(u(operands)?),
            "jalr" => Jalr(i(operands)?),

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

            "vle8.v" => Vlv { eew: 8, data: vl(operands)?},
            "vle16.v" => Vlv { eew: 16, data: vl(operands)?},
            "vle32.v" => Vlv { eew: 32, data: vl(operands)?},
            "vle64.v" => Vlv { eew: 64, data: vl(operands)?},

            "vse8.v" => Vsv { eew: 8, data: vs(operands)?},
            "vse16.v" => Vsv { eew: 16, data: vs(operands)?},
            "vse32.v" => Vsv { eew: 32, data: vs(operands)?},
            "vse64.v" => Vsv { eew: 64, data: vs(operands)?},

            "vlm.v" => Vlmv(vlm(operands)?),
            "vsm.v" => Vsmv(vsm(operands)?),

            "vlse8.v" => Vlsv { eew: 8, data: vls(operands)?},
            "vlse16.v" => Vlsv { eew: 16, data: vls(operands)?},
            "vlse32.v" => Vlsv { eew: 32, data: vls(operands)?},
            "vlse64.v" => Vlsv { eew: 64, data: vls(operands)?},

            "vsse8.v" => Vssv { eew: 8, data: vss(operands)?},
            "vsse16.v" => Vssv { eew: 16, data: vss(operands)?},
            "vsse32.v" => Vssv { eew: 32, data: vss(operands)?},
            "vsse64.v" => Vssv { eew: 64, data: vss(operands)?},

            "vluxei8.v" => Vluxv { eew: 8, data: vlx(operands)?},
            "vluxei16.v" => Vluxv { eew: 16, data: vlx(operands)?},
            "vluxei32.v" => Vluxv { eew: 32, data: vlx(operands)?},
            "vluxei64.v" => Vluxv { eew: 64, data: vlx(operands)?},

            "vloxei8.v" => Vloxv { eew: 8, data: vlx(operands)?},
            "vloxei16.v" => Vloxv { eew: 16, data: vlx(operands)?},
            "vloxei32.v" => Vloxv { eew: 32, data: vlx(operands)?},
            "vloxei64.v" => Vloxv { eew: 64, data: vlx(operands)?},

            "vsuxei8.v" => Vsuxv { eew: 8, data: vsx(operands)?},
            "vsuxei16.v" => Vsuxv { eew: 16, data: vsx(operands)?},
            "vsuxei32.v" => Vsuxv { eew: 32, data: vsx(operands)?},
            "vsuxei64.v" => Vsuxv { eew: 64, data: vsx(operands)?},

            "vsuxeix8.v" => Vsuxv { eew: 8, data: vsx(operands)?},
            "vsuxeix16.v" => Vsuxv { eew: 16, data: vsx(operands)?},
            "vsuxeix32.v" => Vsuxv { eew: 32, data: vsx(operands)?},
            "vsuxeix64.v" => Vsuxv { eew: 64, data: vsx(operands)?},

            "vle8ff.v" => Vlffv { eew: 8, data: vl(operands)?},
            "vle16ff.v" => Vlffv { eew: 16, data: vl(operands)?},
            "vle32ff.v" => Vlffv { eew: 32, data: vl(operands)?},
            "vle64ff.v" => Vlffv { eew: 64, data: vl(operands)?},

            // Note: I need to list all combinations so that I can research const-generification segmented load/stores in the future

            "vlseg1e8.v"  => Vlsegv { nf: 1, eew: 8,  data: vl(operands)?},
            "vlseg1e16.v" => Vlsegv { nf: 1, eew: 16, data: vl(operands)?},
            "vlseg1e32.v" => Vlsegv { nf: 1, eew: 32, data: vl(operands)?},
            "vlseg1e64.v" => Vlsegv { nf: 1, eew: 64, data: vl(operands)?},
            "vlseg2e8.v"  => Vlsegv { nf: 2, eew: 8,  data: vl(operands)?},
            "vlseg2e16.v" => Vlsegv { nf: 2, eew: 16, data: vl(operands)?},
            "vlseg2e32.v" => Vlsegv { nf: 2, eew: 32, data: vl(operands)?},
            "vlseg2e64.v" => Vlsegv { nf: 2, eew: 64, data: vl(operands)?},
            "vlseg3e8.v"  => Vlsegv { nf: 3, eew: 8,  data: vl(operands)?},
            "vlseg3e16.v" => Vlsegv { nf: 3, eew: 16, data: vl(operands)?},
            "vlseg3e32.v" => Vlsegv { nf: 3, eew: 32, data: vl(operands)?},
            "vlseg3e64.v" => Vlsegv { nf: 3, eew: 64, data: vl(operands)?},
            "vlseg4e8.v"  => Vlsegv { nf: 4, eew: 8,  data: vl(operands)?},
            "vlseg4e16.v" => Vlsegv { nf: 4, eew: 16, data: vl(operands)?},
            "vlseg4e32.v" => Vlsegv { nf: 4, eew: 32, data: vl(operands)?},
            "vlseg4e64.v" => Vlsegv { nf: 4, eew: 64, data: vl(operands)?},
            "vlseg5e8.v"  => Vlsegv { nf: 5, eew: 8,  data: vl(operands)?},
            "vlseg5e16.v" => Vlsegv { nf: 5, eew: 16, data: vl(operands)?},
            "vlseg5e32.v" => Vlsegv { nf: 5, eew: 32, data: vl(operands)?},
            "vlseg5e64.v" => Vlsegv { nf: 5, eew: 64, data: vl(operands)?},
            "vlseg6e8.v"  => Vlsegv { nf: 6, eew: 8,  data: vl(operands)?},
            "vlseg6e16.v" => Vlsegv { nf: 6, eew: 16, data: vl(operands)?},
            "vlseg6e32.v" => Vlsegv { nf: 6, eew: 32, data: vl(operands)?},
            "vlseg6e64.v" => Vlsegv { nf: 6, eew: 64, data: vl(operands)?},
            "vlseg7e8.v"  => Vlsegv { nf: 7, eew: 8,  data: vl(operands)?},
            "vlseg7e16.v" => Vlsegv { nf: 7, eew: 16, data: vl(operands)?},
            "vlseg7e32.v" => Vlsegv { nf: 7, eew: 32, data: vl(operands)?},
            "vlseg7e64.v" => Vlsegv { nf: 7, eew: 64, data: vl(operands)?},
            "vlseg8e8.v"  => Vlsegv { nf: 8, eew: 8,  data: vl(operands)?},
            "vlseg8e16.v" => Vlsegv { nf: 8, eew: 16, data: vl(operands)?},
            "vlseg8e32.v" => Vlsegv { nf: 8, eew: 32, data: vl(operands)?},
            "vlseg8e64.v" => Vlsegv { nf: 8, eew: 64, data: vl(operands)?},

            "vsseg1e8.v"  => Vssegv { nf: 1, eew: 8,  data: vs(operands)?},
            "vsseg1e16.v" => Vssegv { nf: 1, eew: 16, data: vs(operands)?},
            "vsseg1e32.v" => Vssegv { nf: 1, eew: 32, data: vs(operands)?},
            "vsseg1e64.v" => Vssegv { nf: 1, eew: 64, data: vs(operands)?},
            "vsseg2e8.v"  => Vssegv { nf: 2, eew: 8,  data: vs(operands)?},
            "vsseg2e16.v" => Vssegv { nf: 2, eew: 16, data: vs(operands)?},
            "vsseg2e32.v" => Vssegv { nf: 2, eew: 32, data: vs(operands)?},
            "vsseg2e64.v" => Vssegv { nf: 2, eew: 64, data: vs(operands)?},
            "vsseg3e8.v"  => Vssegv { nf: 3, eew: 8,  data: vs(operands)?},
            "vsseg3e16.v" => Vssegv { nf: 3, eew: 16, data: vs(operands)?},
            "vsseg3e32.v" => Vssegv { nf: 3, eew: 32, data: vs(operands)?},
            "vsseg3e64.v" => Vssegv { nf: 3, eew: 64, data: vs(operands)?},
            "vsseg4e8.v"  => Vssegv { nf: 4, eew: 8,  data: vs(operands)?},
            "vsseg4e16.v" => Vssegv { nf: 4, eew: 16, data: vs(operands)?},
            "vsseg4e32.v" => Vssegv { nf: 4, eew: 32, data: vs(operands)?},
            "vsseg4e64.v" => Vssegv { nf: 4, eew: 64, data: vs(operands)?},
            "vsseg5e8.v"  => Vssegv { nf: 5, eew: 8,  data: vs(operands)?},
            "vsseg5e16.v" => Vssegv { nf: 5, eew: 16, data: vs(operands)?},
            "vsseg5e32.v" => Vssegv { nf: 5, eew: 32, data: vs(operands)?},
            "vsseg5e64.v" => Vssegv { nf: 5, eew: 64, data: vs(operands)?},
            "vsseg6e8.v"  => Vssegv { nf: 6, eew: 8,  data: vs(operands)?},
            "vsseg6e16.v" => Vssegv { nf: 6, eew: 16, data: vs(operands)?},
            "vsseg6e32.v" => Vssegv { nf: 6, eew: 32, data: vs(operands)?},
            "vsseg6e64.v" => Vssegv { nf: 6, eew: 64, data: vs(operands)?},
            "vsseg7e8.v"  => Vssegv { nf: 7, eew: 8,  data: vs(operands)?},
            "vsseg7e16.v" => Vssegv { nf: 7, eew: 16, data: vs(operands)?},
            "vsseg7e32.v" => Vssegv { nf: 7, eew: 32, data: vs(operands)?},
            "vsseg7e64.v" => Vssegv { nf: 7, eew: 64, data: vs(operands)?},
            "vsseg8e8.v"  => Vssegv { nf: 8, eew: 8,  data: vs(operands)?},
            "vsseg8e16.v" => Vssegv { nf: 8, eew: 16, data: vs(operands)?},
            "vsseg8e32.v" => Vssegv { nf: 8, eew: 32, data: vs(operands)?},
            "vsseg8e64.v" => Vssegv { nf: 8, eew: 64, data: vs(operands)?},

            "vlsseg1e8.v"  => Vlssegv { nf: 1, eew: 8,  data: vls(operands)?},
            "vlsseg1e16.v" => Vlssegv { nf: 1, eew: 16, data: vls(operands)?},
            "vlsseg1e32.v" => Vlssegv { nf: 1, eew: 32, data: vls(operands)?},
            "vlsseg1e64.v" => Vlssegv { nf: 1, eew: 64, data: vls(operands)?},
            "vlsseg2e8.v"  => Vlssegv { nf: 2, eew: 8,  data: vls(operands)?},
            "vlsseg2e16.v" => Vlssegv { nf: 2, eew: 16, data: vls(operands)?},
            "vlsseg2e32.v" => Vlssegv { nf: 2, eew: 32, data: vls(operands)?},
            "vlsseg2e64.v" => Vlssegv { nf: 2, eew: 64, data: vls(operands)?},
            "vlsseg3e8.v"  => Vlssegv { nf: 3, eew: 8,  data: vls(operands)?},
            "vlsseg3e16.v" => Vlssegv { nf: 3, eew: 16, data: vls(operands)?},
            "vlsseg3e32.v" => Vlssegv { nf: 3, eew: 32, data: vls(operands)?},
            "vlsseg3e64.v" => Vlssegv { nf: 3, eew: 64, data: vls(operands)?},
            "vlsseg4e8.v"  => Vlssegv { nf: 4, eew: 8,  data: vls(operands)?},
            "vlsseg4e16.v" => Vlssegv { nf: 4, eew: 16, data: vls(operands)?},
            "vlsseg4e32.v" => Vlssegv { nf: 4, eew: 32, data: vls(operands)?},
            "vlsseg4e64.v" => Vlssegv { nf: 4, eew: 64, data: vls(operands)?},
            "vlsseg5e8.v"  => Vlssegv { nf: 5, eew: 8,  data: vls(operands)?},
            "vlsseg5e16.v" => Vlssegv { nf: 5, eew: 16, data: vls(operands)?},
            "vlsseg5e32.v" => Vlssegv { nf: 5, eew: 32, data: vls(operands)?},
            "vlsseg5e64.v" => Vlssegv { nf: 5, eew: 64, data: vls(operands)?},
            "vlsseg6e8.v"  => Vlssegv { nf: 6, eew: 8,  data: vls(operands)?},
            "vlsseg6e16.v" => Vlssegv { nf: 6, eew: 16, data: vls(operands)?},
            "vlsseg6e32.v" => Vlssegv { nf: 6, eew: 32, data: vls(operands)?},
            "vlsseg6e64.v" => Vlssegv { nf: 6, eew: 64, data: vls(operands)?},
            "vlsseg7e8.v"  => Vlssegv { nf: 7, eew: 8,  data: vls(operands)?},
            "vlsseg7e16.v" => Vlssegv { nf: 7, eew: 16, data: vls(operands)?},
            "vlsseg7e32.v" => Vlssegv { nf: 7, eew: 32, data: vls(operands)?},
            "vlsseg7e64.v" => Vlssegv { nf: 7, eew: 64, data: vls(operands)?},
            "vlsseg8e8.v"  => Vlssegv { nf: 8, eew: 8,  data: vls(operands)?},
            "vlsseg8e16.v" => Vlssegv { nf: 8, eew: 16, data: vls(operands)?},
            "vlsseg8e32.v" => Vlssegv { nf: 8, eew: 32, data: vls(operands)?},
            "vlsseg8e64.v" => Vlssegv { nf: 8, eew: 64, data: vls(operands)?},
            
            "vssseg1e8.v"  => Vsssegv { nf: 1, eew: 8,  data: vss(operands)?},
            "vssseg1e16.v" => Vsssegv { nf: 1, eew: 16, data: vss(operands)?},
            "vssseg1e32.v" => Vsssegv { nf: 1, eew: 32, data: vss(operands)?},
            "vssseg1e64.v" => Vsssegv { nf: 1, eew: 64, data: vss(operands)?},
            "vssseg2e8.v"  => Vsssegv { nf: 2, eew: 8,  data: vss(operands)?},
            "vssseg2e16.v" => Vsssegv { nf: 2, eew: 16, data: vss(operands)?},
            "vssseg2e32.v" => Vsssegv { nf: 2, eew: 32, data: vss(operands)?},
            "vssseg2e64.v" => Vsssegv { nf: 2, eew: 64, data: vss(operands)?},
            "vssseg3e8.v"  => Vsssegv { nf: 3, eew: 8,  data: vss(operands)?},
            "vssseg3e16.v" => Vsssegv { nf: 3, eew: 16, data: vss(operands)?},
            "vssseg3e32.v" => Vsssegv { nf: 3, eew: 32, data: vss(operands)?},
            "vssseg3e64.v" => Vsssegv { nf: 3, eew: 64, data: vss(operands)?},
            "vssseg4e8.v"  => Vsssegv { nf: 4, eew: 8,  data: vss(operands)?},
            "vssseg4e16.v" => Vsssegv { nf: 4, eew: 16, data: vss(operands)?},
            "vssseg4e32.v" => Vsssegv { nf: 4, eew: 32, data: vss(operands)?},
            "vssseg4e64.v" => Vsssegv { nf: 4, eew: 64, data: vss(operands)?},
            "vssseg5e8.v"  => Vsssegv { nf: 5, eew: 8,  data: vss(operands)?},
            "vssseg5e16.v" => Vsssegv { nf: 5, eew: 16, data: vss(operands)?},
            "vssseg5e32.v" => Vsssegv { nf: 5, eew: 32, data: vss(operands)?},
            "vssseg5e64.v" => Vsssegv { nf: 5, eew: 64, data: vss(operands)?},
            "vssseg6e8.v"  => Vsssegv { nf: 6, eew: 8,  data: vss(operands)?},
            "vssseg6e16.v" => Vsssegv { nf: 6, eew: 16, data: vss(operands)?},
            "vssseg6e32.v" => Vsssegv { nf: 6, eew: 32, data: vss(operands)?},
            "vssseg6e64.v" => Vsssegv { nf: 6, eew: 64, data: vss(operands)?},
            "vssseg7e8.v"  => Vsssegv { nf: 7, eew: 8,  data: vss(operands)?},
            "vssseg7e16.v" => Vsssegv { nf: 7, eew: 16, data: vss(operands)?},
            "vssseg7e32.v" => Vsssegv { nf: 7, eew: 32, data: vss(operands)?},
            "vssseg7e64.v" => Vsssegv { nf: 7, eew: 64, data: vss(operands)?},
            "vssseg8e8.v"  => Vsssegv { nf: 8, eew: 8,  data: vss(operands)?},
            "vssseg8e16.v" => Vsssegv { nf: 8, eew: 16, data: vss(operands)?},
            "vssseg8e32.v" => Vsssegv { nf: 8, eew: 32, data: vss(operands)?},
            "vssseg8e64.v" => Vsssegv { nf: 8, eew: 64, data: vss(operands)?},

            "vluxseg1ei8.v"  => Vluxsegv { nf: 1, eew: 8,  data: vlx(operands)?},
            "vluxseg1ei16.v" => Vluxsegv { nf: 1, eew: 16, data: vlx(operands)?},
            "vluxseg1ei32.v" => Vluxsegv { nf: 1, eew: 32, data: vlx(operands)?},
            "vluxseg1ei64.v" => Vluxsegv { nf: 1, eew: 64, data: vlx(operands)?},
            "vluxseg2ei8.v"  => Vluxsegv { nf: 2, eew: 8,  data: vlx(operands)?},
            "vluxseg2ei16.v" => Vluxsegv { nf: 2, eew: 16, data: vlx(operands)?},
            "vluxseg2ei32.v" => Vluxsegv { nf: 2, eew: 32, data: vlx(operands)?},
            "vluxseg2ei64.v" => Vluxsegv { nf: 2, eew: 64, data: vlx(operands)?},
            "vluxseg3ei8.v"  => Vluxsegv { nf: 3, eew: 8,  data: vlx(operands)?},
            "vluxseg3ei16.v" => Vluxsegv { nf: 3, eew: 16, data: vlx(operands)?},
            "vluxseg3ei32.v" => Vluxsegv { nf: 3, eew: 32, data: vlx(operands)?},
            "vluxseg3ei64.v" => Vluxsegv { nf: 3, eew: 64, data: vlx(operands)?},
            "vluxseg4ei8.v"  => Vluxsegv { nf: 4, eew: 8,  data: vlx(operands)?},
            "vluxseg4ei16.v" => Vluxsegv { nf: 4, eew: 16, data: vlx(operands)?},
            "vluxseg4ei32.v" => Vluxsegv { nf: 4, eew: 32, data: vlx(operands)?},
            "vluxseg4ei64.v" => Vluxsegv { nf: 4, eew: 64, data: vlx(operands)?},
            "vluxseg5ei8.v"  => Vluxsegv { nf: 5, eew: 8,  data: vlx(operands)?},
            "vluxseg5ei16.v" => Vluxsegv { nf: 5, eew: 16, data: vlx(operands)?},
            "vluxseg5ei32.v" => Vluxsegv { nf: 5, eew: 32, data: vlx(operands)?},
            "vluxseg5ei64.v" => Vluxsegv { nf: 5, eew: 64, data: vlx(operands)?},
            "vluxseg6ei8.v"  => Vluxsegv { nf: 6, eew: 8,  data: vlx(operands)?},
            "vluxseg6ei16.v" => Vluxsegv { nf: 6, eew: 16, data: vlx(operands)?},
            "vluxseg6ei32.v" => Vluxsegv { nf: 6, eew: 32, data: vlx(operands)?},
            "vluxseg6ei64.v" => Vluxsegv { nf: 6, eew: 64, data: vlx(operands)?},
            "vluxseg7ei8.v"  => Vluxsegv { nf: 7, eew: 8,  data: vlx(operands)?},
            "vluxseg7ei16.v" => Vluxsegv { nf: 7, eew: 16, data: vlx(operands)?},
            "vluxseg7ei32.v" => Vluxsegv { nf: 7, eew: 32, data: vlx(operands)?},
            "vluxseg7ei64.v" => Vluxsegv { nf: 7, eew: 64, data: vlx(operands)?},
            "vluxseg8ei8.v"  => Vluxsegv { nf: 8, eew: 8,  data: vlx(operands)?},
            "vluxseg8ei16.v" => Vluxsegv { nf: 8, eew: 16, data: vlx(operands)?},
            "vluxseg8ei32.v" => Vluxsegv { nf: 8, eew: 32, data: vlx(operands)?},
            "vluxseg8ei64.v" => Vluxsegv { nf: 8, eew: 64, data: vlx(operands)?},

            "vloxseg1ei8.v"  => Vloxsegv { nf: 1, eew: 8,  data: vlx(operands)?},
            "vloxseg1ei16.v" => Vloxsegv { nf: 1, eew: 16, data: vlx(operands)?},
            "vloxseg1ei32.v" => Vloxsegv { nf: 1, eew: 32, data: vlx(operands)?},
            "vloxseg1ei64.v" => Vloxsegv { nf: 1, eew: 64, data: vlx(operands)?},
            "vloxseg2ei8.v"  => Vloxsegv { nf: 2, eew: 8,  data: vlx(operands)?},
            "vloxseg2ei16.v" => Vloxsegv { nf: 2, eew: 16, data: vlx(operands)?},
            "vloxseg2ei32.v" => Vloxsegv { nf: 2, eew: 32, data: vlx(operands)?},
            "vloxseg2ei64.v" => Vloxsegv { nf: 2, eew: 64, data: vlx(operands)?},
            "vloxseg3ei8.v"  => Vloxsegv { nf: 3, eew: 8,  data: vlx(operands)?},
            "vloxseg3ei16.v" => Vloxsegv { nf: 3, eew: 16, data: vlx(operands)?},
            "vloxseg3ei32.v" => Vloxsegv { nf: 3, eew: 32, data: vlx(operands)?},
            "vloxseg3ei64.v" => Vloxsegv { nf: 3, eew: 64, data: vlx(operands)?},
            "vloxseg4ei8.v"  => Vloxsegv { nf: 4, eew: 8,  data: vlx(operands)?},
            "vloxseg4ei16.v" => Vloxsegv { nf: 4, eew: 16, data: vlx(operands)?},
            "vloxseg4ei32.v" => Vloxsegv { nf: 4, eew: 32, data: vlx(operands)?},
            "vloxseg4ei64.v" => Vloxsegv { nf: 4, eew: 64, data: vlx(operands)?},
            "vloxseg5ei8.v"  => Vloxsegv { nf: 5, eew: 8,  data: vlx(operands)?},
            "vloxseg5ei16.v" => Vloxsegv { nf: 5, eew: 16, data: vlx(operands)?},
            "vloxseg5ei32.v" => Vloxsegv { nf: 5, eew: 32, data: vlx(operands)?},
            "vloxseg5ei64.v" => Vloxsegv { nf: 5, eew: 64, data: vlx(operands)?},
            "vloxseg6ei8.v"  => Vloxsegv { nf: 6, eew: 8,  data: vlx(operands)?},
            "vloxseg6ei16.v" => Vloxsegv { nf: 6, eew: 16, data: vlx(operands)?},
            "vloxseg6ei32.v" => Vloxsegv { nf: 6, eew: 32, data: vlx(operands)?},
            "vloxseg6ei64.v" => Vloxsegv { nf: 6, eew: 64, data: vlx(operands)?},
            "vloxseg7ei8.v"  => Vloxsegv { nf: 7, eew: 8,  data: vlx(operands)?},
            "vloxseg7ei16.v" => Vloxsegv { nf: 7, eew: 16, data: vlx(operands)?},
            "vloxseg7ei32.v" => Vloxsegv { nf: 7, eew: 32, data: vlx(operands)?},
            "vloxseg7ei64.v" => Vloxsegv { nf: 7, eew: 64, data: vlx(operands)?},
            "vloxseg8ei8.v"  => Vloxsegv { nf: 8, eew: 8,  data: vlx(operands)?},
            "vloxseg8ei16.v" => Vloxsegv { nf: 8, eew: 16, data: vlx(operands)?},
            "vloxseg8ei32.v" => Vloxsegv { nf: 8, eew: 32, data: vlx(operands)?},
            "vloxseg8ei64.v" => Vloxsegv { nf: 8, eew: 64, data: vlx(operands)?},

            "vsuxseg1ei8.v"  => Vsuxsegv { nf: 1, eew: 8,  data: vsx(operands)?},
            "vsuxseg1ei16.v" => Vsuxsegv { nf: 1, eew: 16, data: vsx(operands)?},
            "vsuxseg1ei32.v" => Vsuxsegv { nf: 1, eew: 32, data: vsx(operands)?},
            "vsuxseg1ei64.v" => Vsuxsegv { nf: 1, eew: 64, data: vsx(operands)?},
            "vsuxseg2ei8.v"  => Vsuxsegv { nf: 2, eew: 8,  data: vsx(operands)?},
            "vsuxseg2ei16.v" => Vsuxsegv { nf: 2, eew: 16, data: vsx(operands)?},
            "vsuxseg2ei32.v" => Vsuxsegv { nf: 2, eew: 32, data: vsx(operands)?},
            "vsuxseg2ei64.v" => Vsuxsegv { nf: 2, eew: 64, data: vsx(operands)?},
            "vsuxseg3ei8.v"  => Vsuxsegv { nf: 3, eew: 8,  data: vsx(operands)?},
            "vsuxseg3ei16.v" => Vsuxsegv { nf: 3, eew: 16, data: vsx(operands)?},
            "vsuxseg3ei32.v" => Vsuxsegv { nf: 3, eew: 32, data: vsx(operands)?},
            "vsuxseg3ei64.v" => Vsuxsegv { nf: 3, eew: 64, data: vsx(operands)?},
            "vsuxseg4ei8.v"  => Vsuxsegv { nf: 4, eew: 8,  data: vsx(operands)?},
            "vsuxseg4ei16.v" => Vsuxsegv { nf: 4, eew: 16, data: vsx(operands)?},
            "vsuxseg4ei32.v" => Vsuxsegv { nf: 4, eew: 32, data: vsx(operands)?},
            "vsuxseg4ei64.v" => Vsuxsegv { nf: 4, eew: 64, data: vsx(operands)?},
            "vsuxseg5ei8.v"  => Vsuxsegv { nf: 5, eew: 8,  data: vsx(operands)?},
            "vsuxseg5ei16.v" => Vsuxsegv { nf: 5, eew: 16, data: vsx(operands)?},
            "vsuxseg5ei32.v" => Vsuxsegv { nf: 5, eew: 32, data: vsx(operands)?},
            "vsuxseg5ei64.v" => Vsuxsegv { nf: 5, eew: 64, data: vsx(operands)?},
            "vsuxseg6ei8.v"  => Vsuxsegv { nf: 6, eew: 8,  data: vsx(operands)?},
            "vsuxseg6ei16.v" => Vsuxsegv { nf: 6, eew: 16, data: vsx(operands)?},
            "vsuxseg6ei32.v" => Vsuxsegv { nf: 6, eew: 32, data: vsx(operands)?},
            "vsuxseg6ei64.v" => Vsuxsegv { nf: 6, eew: 64, data: vsx(operands)?},
            "vsuxseg7ei8.v"  => Vsuxsegv { nf: 7, eew: 8,  data: vsx(operands)?},
            "vsuxseg7ei16.v" => Vsuxsegv { nf: 7, eew: 16, data: vsx(operands)?},
            "vsuxseg7ei32.v" => Vsuxsegv { nf: 7, eew: 32, data: vsx(operands)?},
            "vsuxseg7ei64.v" => Vsuxsegv { nf: 7, eew: 64, data: vsx(operands)?},
            "vsuxseg8ei8.v"  => Vsuxsegv { nf: 8, eew: 8,  data: vsx(operands)?},
            "vsuxseg8ei16.v" => Vsuxsegv { nf: 8, eew: 16, data: vsx(operands)?},
            "vsuxseg8ei32.v" => Vsuxsegv { nf: 8, eew: 32, data: vsx(operands)?},
            "vsuxseg8ei64.v" => Vsuxsegv { nf: 8, eew: 64, data: vsx(operands)?},
            
            "vsoxseg1ei8.v"  => Vsoxsegv { nf: 1, eew: 8,  data: vsx(operands)?},
            "vsoxseg1ei16.v" => Vsoxsegv { nf: 1, eew: 16, data: vsx(operands)?},
            "vsoxseg1ei32.v" => Vsoxsegv { nf: 1, eew: 32, data: vsx(operands)?},
            "vsoxseg1ei64.v" => Vsoxsegv { nf: 1, eew: 64, data: vsx(operands)?},
            "vsoxseg2ei8.v"  => Vsoxsegv { nf: 2, eew: 8,  data: vsx(operands)?},
            "vsoxseg2ei16.v" => Vsoxsegv { nf: 2, eew: 16, data: vsx(operands)?},
            "vsoxseg2ei32.v" => Vsoxsegv { nf: 2, eew: 32, data: vsx(operands)?},
            "vsoxseg2ei64.v" => Vsoxsegv { nf: 2, eew: 64, data: vsx(operands)?},
            "vsoxseg3ei8.v"  => Vsoxsegv { nf: 3, eew: 8,  data: vsx(operands)?},
            "vsoxseg3ei16.v" => Vsoxsegv { nf: 3, eew: 16, data: vsx(operands)?},
            "vsoxseg3ei32.v" => Vsoxsegv { nf: 3, eew: 32, data: vsx(operands)?},
            "vsoxseg3ei64.v" => Vsoxsegv { nf: 3, eew: 64, data: vsx(operands)?},
            "vsoxseg4ei8.v"  => Vsoxsegv { nf: 4, eew: 8,  data: vsx(operands)?},
            "vsoxseg4ei16.v" => Vsoxsegv { nf: 4, eew: 16, data: vsx(operands)?},
            "vsoxseg4ei32.v" => Vsoxsegv { nf: 4, eew: 32, data: vsx(operands)?},
            "vsoxseg4ei64.v" => Vsoxsegv { nf: 4, eew: 64, data: vsx(operands)?},
            "vsoxseg5ei8.v"  => Vsoxsegv { nf: 5, eew: 8,  data: vsx(operands)?},
            "vsoxseg5ei16.v" => Vsoxsegv { nf: 5, eew: 16, data: vsx(operands)?},
            "vsoxseg5ei32.v" => Vsoxsegv { nf: 5, eew: 32, data: vsx(operands)?},
            "vsoxseg5ei64.v" => Vsoxsegv { nf: 5, eew: 64, data: vsx(operands)?},
            "vsoxseg6ei8.v"  => Vsoxsegv { nf: 6, eew: 8,  data: vsx(operands)?},
            "vsoxseg6ei16.v" => Vsoxsegv { nf: 6, eew: 16, data: vsx(operands)?},
            "vsoxseg6ei32.v" => Vsoxsegv { nf: 6, eew: 32, data: vsx(operands)?},
            "vsoxseg6ei64.v" => Vsoxsegv { nf: 6, eew: 64, data: vsx(operands)?},
            "vsoxseg7ei8.v"  => Vsoxsegv { nf: 7, eew: 8,  data: vsx(operands)?},
            "vsoxseg7ei16.v" => Vsoxsegv { nf: 7, eew: 16, data: vsx(operands)?},
            "vsoxseg7ei32.v" => Vsoxsegv { nf: 7, eew: 32, data: vsx(operands)?},
            "vsoxseg7ei64.v" => Vsoxsegv { nf: 7, eew: 64, data: vsx(operands)?},
            "vsoxseg8ei8.v"  => Vsoxsegv { nf: 8, eew: 8,  data: vsx(operands)?},
            "vsoxseg8ei16.v" => Vsoxsegv { nf: 8, eew: 16, data: vsx(operands)?},
            "vsoxseg8ei32.v" => Vsoxsegv { nf: 8, eew: 32, data: vsx(operands)?},
            "vsoxseg8ei64.v" => Vsoxsegv { nf: 8, eew: 64, data: vsx(operands)?},

            "vl1re8.v" => Vlrv { nf: 1, eew: 8,  data: vlr(operands)?},
            "vl1re16.v" => Vlrv { nf: 1, eew: 16, data: vlr(operands)?},
            "vl1re32.v" => Vlrv { nf: 1, eew: 32, data: vlr(operands)?},
            "vl1re64.v" => Vlrv { nf: 1, eew: 64, data: vlr(operands)?},
            "vl2re8.v" => Vlrv { nf: 2, eew: 8,  data: vlr(operands)?},
            "vl2re16.v" => Vlrv { nf: 2, eew: 16, data: vlr(operands)?},
            "vl2re32.v" => Vlrv { nf: 2, eew: 32, data: vlr(operands)?},
            "vl2re64.v" => Vlrv { nf: 2, eew: 64, data: vlr(operands)?},
            "vl4re8.v" => Vlrv { nf: 4, eew: 8,  data: vlr(operands)?},
            "vl4re16.v" => Vlrv { nf: 4, eew: 16, data: vlr(operands)?},
            "vl4re32.v" => Vlrv { nf: 4, eew: 32, data: vlr(operands)?},
            "vl4re64.v" => Vlrv { nf: 4, eew: 64, data: vlr(operands)?},
            "vl8re8.v" => Vlrv { nf: 8, eew: 8,  data: vlr(operands)?},
            "vl8re16.v" => Vlrv { nf: 8, eew: 16, data: vlr(operands)?},
            "vl8re32.v" => Vlrv { nf: 8, eew: 32, data: vlr(operands)?},
            "vl8re64.v" => Vlrv { nf: 8, eew: 64, data: vlr(operands)?},

            "vs1r.v" => Vsrv { nf: 1, data: vsr(operands)?},
            "vs2r.v" => Vsrv { nf: 2, data: vsr(operands)?},
            "vs4r.v" => Vsrv { nf: 4, data: vsr(operands)?},
            "vs8r.v" => Vsrv { nf: 8, data: vsr(operands)?},

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

            "vadc.vvm" => Vadcvvm(opivv(operands)?),
            "vadc.vxm" => Vadcvxm(opivx(operands)?),
            "vadc.vim" => Vadcvim(opivi(operands)?),

            "vmadc.vvm" => Vmadcvvm(opivv(operands)?),
            "vmadc.vxm" => Vmadcvxm(opivx(operands)?),
            "vmadc.vim" => Vmadcvim(opivi(operands)?),
            "vmadc.vv" => Vmadcvv(opivv(operands)?),
            "vmadc.vx" => Vmadcvx(opivx(operands)?),
            "vmadc.vi" => Vmadcvi(opivi(operands)?),

            "vsbc.vvm" => Vsbcvvm(opivv(operands)?),
            "vsbc.vxm" => Vsbcvxm(opivx(operands)?),

            "vmsbc.vv" => Vmsbcvv(opivv(operands)?),
            "vmsbc.vx" => Vmsbcvx(opivx(operands)?),

            "vmerge.vvm" => Vmergevvm(opivv(operands)?),
            "vmerge.vxm" => Vmergevxm(opivx(operands)?),
            "vmerge.vim" => Vmergevim(opivi(operands)?),

            "vmv.v.v" => Vmvvv(opivv(operands)?),
            "vmv.v.x" => Vmvvx(opivx(operands)?),
            "vmv.v.i" => Vmvvi(opivi(operands)?),

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

            "vslide1up.vx"  => Vslide1upvx(opmvx(operands)?),

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

            "vcompress.vm" => Vcompressvm(opmvv(operands)?),

            "vmandn.mm" => Vmandnmm(opmvv(operands)?),
            "vmand.mm" => Vmandmm(opmvv(operands)?),
            "vmor.mm" => Vmormm(opmvv(operands)?),
            "vmxor.mm" => Vmxormm(opmvv(operands)?),
            "vmorn.mm" => Vmornmm(opmvv(operands)?),
            "vmnand.mm" => Vmnandmm(opmvv(operands)?),
            "vmnor.mm" => Vmnormm(opmvv(operands)?),
            "vmxnor.mm" => Vmxnormm(opmvv(operands)?),

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

            "vmadd.vv" => Vmaddvv(opmvv(operands)?),
            "vmadd.vx" => Vmaddvx(opmvx(operands)?),

            "vnmsub.vv" => Vnmsubvv(opmvv(operands)?),
            "vnmsub.vx" => Vnmsubvx(opmvx(operands)?),

            "vmacc.vv" => Vmaccvv(opmvv(operands)?),
            "vmacc.vx" => Vmaccvx(opmvx(operands)?),

            "vnmsac.vv" => Vnmsacvv(opmvv(operands)?),
            "vnmsac.vx" => Vnmsacvx(opmvx(operands)?),

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

            "vwmaccu.vv" => Vwmaccuvv(opmvv(operands)?),
            "vwmaccu.vx" => Vwmaccuvx(opmvx(operands)?),

            "vwmacc.vv" => Vwmaccvv(opmvv(operands)?),
            "vwmacc.vx" => Vwmaccvx(opmvx(operands)?),

            "vwmaccus.vx" => Vwmaccusvx(opmvx(operands)?),

            "vwmaccsu.vv" => Vwmaccsuvv(opmvv(operands)?),
            "vwmaccsu.vx" => Vwmaccsuvx(opmvx(operands)?),

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

            "vfmadd.vv" => Vfmaddvv(opfvv(operands)?),
            "vfmadd.vf" => Vfmaddvf(opfvf(operands)?),

            "vfnmadd.vv" => Vfnmaddvv(opfvv(operands)?),
            "vfnmadd.vf" => Vfnmaddvf(opfvf(operands)?),

            "vfmsub.vv" => Vfmsubvv(opfvv(operands)?),
            "vfmsub.vf" => Vfmsubvf(opfvf(operands)?),

            "vfnmsub.vv" => Vfnmsubvv(opfvv(operands)?),
            "vfnmsub.vf" => Vfnmsubvf(opfvf(operands)?),

            "vfmacc.vv" => Vfmaccvv(opfvv(operands)?),
            "vfmacc.vf" => Vfmaccvf(opfvf(operands)?),

            "vfnmacc.vv" => Vfnmaccvv(opfvv(operands)?),
            "vfnmacc.vf" => Vfnmaccvf(opfvf(operands)?),

            "vfmsac.vv" => Vfmsacvv(opfvv(operands)?),
            "vfmsac.vf" => Vfmsacvf(opfvf(operands)?),

            "vfnmsac.vv" => Vfnmsacvv(opfvv(operands)?),
            "vfnmsac.vf" => Vfnmsacvf(opfvf(operands)?),

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

            "vfwmacc.vv" => Vfwmaccvv(opfvv(operands)?),
            "vfwmacc.vf" => Vfwmaccvf(opfvf(operands)?),

            "vfwnmacc.vv" => Vfwnmaccvv(opfvv(operands)?),
            "vfwnmacc.vf" => Vfwnmaccvf(opfvf(operands)?),

            "vfwmsac.vv" => Vfwmsacvv(opfvv(operands)?),
            "vfwmsac.vf" => Vfwmsacvf(opfvf(operands)?),

            "vfwnmsac.vv" => Vfwnmsacvv(opfvv(operands)?),
            "vfwnmsac.vf" => Vfwnmsacvf(opfvf(operands)?),

            _ => return Err(format!("Unknown mnemonic: {}", mnemonic))
        };

        Ok(instruction)
    }

    fn split_instruction(instruction_line: &str) -> (&str, &str) {
        let mut lane = instruction_line.splitn(2, char::is_whitespace);
        let mnemonic = lane.next().unwrap_or_default().trim();
        let operands = lane.next().unwrap_or_default().trim();
        (mnemonic, operands)
    }
}