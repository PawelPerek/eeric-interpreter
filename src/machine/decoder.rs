mod operand;

use eeric::prelude::*;
use Instruction::*;
use format as F;
use operand::{IntegerParser, CsrParser, FloatParser, VectorParser};


pub struct Decoder;

pub enum LineClassification {
    Instruction,
    Label,
    Empty
}

impl Decoder {
    pub fn classify(line: String) -> LineClassification {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            LineClassification::Empty
        } else if trimmed_line.ends_with(":") {
            LineClassification::Label
        } else {
            LineClassification::Instruction
        }
    }

    pub fn decode(instruction_line: &str) -> eeric::Instruction {
        let (mnemonic, operands) = Self::split(instruction_line);

        match mnemonic {
            "add" => Add(IntegerParser::parse_r_format(operands).unwrap()),
            "addw" => Addw(IntegerParser::parse_r_format(operands).unwrap()),
            "sub" => Sub(IntegerParser::parse_r_format(operands).unwrap()),
            "subw" => Subw(IntegerParser::parse_r_format(operands).unwrap()),
            "addi" => Addi(IntegerParser::parse_i_format(operands).unwrap()),
            "addiw" => Addiw(IntegerParser::parse_i_format(operands).unwrap()),
            "slt" => Slt(IntegerParser::parse_r_format(operands).unwrap()),
            "slti" => Slti(IntegerParser::parse_i_format(operands).unwrap()),
            "sltu" => Sltu(IntegerParser::parse_r_format(operands).unwrap()),
            "sltiu" => Sltiu(IntegerParser::parse_i_format(operands).unwrap()),
            "lui" => Lui(IntegerParser::parse_u_format(operands).unwrap()),
            "auipc" => Auipc(IntegerParser::parse_u_format(operands).unwrap()),

            "and" => And(IntegerParser::parse_r_format(operands).unwrap()),
            "or" => Or(IntegerParser::parse_r_format(operands).unwrap()),
            "xor" => Xor(IntegerParser::parse_r_format(operands).unwrap()),
            "andi" => Andi(IntegerParser::parse_i_format(operands).unwrap()),
            "ori" => Ori(IntegerParser::parse_i_format(operands).unwrap()),
            "xori" => Xori(IntegerParser::parse_i_format(operands).unwrap()),
            "sll" => Sll(IntegerParser::parse_r_format(operands).unwrap()),
            "sllw" => Sllw(IntegerParser::parse_r_format(operands).unwrap()),
            "srl" => Srl(IntegerParser::parse_r_format(operands).unwrap()),
            "srlw" => Srlw(IntegerParser::parse_r_format(operands).unwrap()),
            "sra" => Sra(IntegerParser::parse_r_format(operands).unwrap()),
            "sraw" => Sraw(IntegerParser::parse_r_format(operands).unwrap()),
            "slli" => Slli(IntegerParser::parse_i_format(operands).unwrap()),
            "slliw" => Slliw(IntegerParser::parse_i_format(operands).unwrap()),
            "srli" => Srli(IntegerParser::parse_i_format(operands).unwrap()),
            "srliw" => Srliw(IntegerParser::parse_i_format(operands).unwrap()),
            "srai" => Srai(IntegerParser::parse_i_format(operands).unwrap()),
            "sraiw" => Sraiw(IntegerParser::parse_i_format(operands).unwrap()),

            "ld" => Ld(IntegerParser::parse_i_format(operands).unwrap()),
            "lw" => Lw(IntegerParser::parse_i_format(operands).unwrap()),
            "lh" => Lh(IntegerParser::parse_i_format(operands).unwrap()),
            "lb" => Lb(IntegerParser::parse_i_format(operands).unwrap()),
            "lwu" => Lwu(IntegerParser::parse_i_format(operands).unwrap()),
            "lhu" => Lhu(IntegerParser::parse_i_format(operands).unwrap()),
            "lbu" => Lbu(IntegerParser::parse_i_format(operands).unwrap()),
            "sd" => Sd(IntegerParser::parse_s_format(operands).unwrap()),
            "sw" => Sw(IntegerParser::parse_s_format(operands).unwrap()),
            "sh" => Sh(IntegerParser::parse_s_format(operands).unwrap()),
            "sb" => Sb(IntegerParser::parse_s_format(operands).unwrap()),

            "beq" => Beq(IntegerParser::parse_s_format(operands).unwrap()),
            "bne" => Bne(IntegerParser::parse_s_format(operands).unwrap()),
            "bge" => Bge(IntegerParser::parse_s_format(operands).unwrap()),
            "bgeu" => Bgeu(IntegerParser::parse_s_format(operands).unwrap()),
            "blt" => Blt(IntegerParser::parse_s_format(operands).unwrap()),
            "bltu" => Bltu(IntegerParser::parse_s_format(operands).unwrap()),
            "jal" => Jal(IntegerParser::parse_u_format(operands).unwrap()),
            "jalr" => Jalr(IntegerParser::parse_i_format(operands).unwrap()),

            "csrrw" => Csrrw(CsrParser::parse_csrr_format(operands).unwrap()),
            "csrrs" => Csrrs(CsrParser::parse_csrr_format(operands).unwrap()),
            "csrrc" => Csrrc(CsrParser::parse_csrr_format(operands).unwrap()),
            "csrrwi" => Csrrwi(CsrParser::parse_csri_format(operands).unwrap()),
            "csrrsi" => Csrrsi(CsrParser::parse_csri_format(operands).unwrap()),
            "csrrci" => Csrrci(CsrParser::parse_csri_format(operands).unwrap()),

            "mul" => Mul(IntegerParser::parse_r_format(operands).unwrap()),
            "mulh" => Mulh(IntegerParser::parse_r_format(operands).unwrap()),
            "mulhsu" => Mulhsu(IntegerParser::parse_r_format(operands).unwrap()),
            "mulhu" => Mulhu(IntegerParser::parse_r_format(operands).unwrap()),
            "div" => Div(IntegerParser::parse_r_format(operands).unwrap()),
            "divu" => Divu(IntegerParser::parse_r_format(operands).unwrap()),
            "rem" => Rem(IntegerParser::parse_r_format(operands).unwrap()),
            "remu" => Remu(IntegerParser::parse_r_format(operands).unwrap()),
            "mulw" => Mulw(IntegerParser::parse_r_format(operands).unwrap()),
            "divw" => Divw(IntegerParser::parse_r_format(operands).unwrap()),
            "divuw" => Divuw(IntegerParser::parse_r_format(operands).unwrap()),
            "remw" => Remw(IntegerParser::parse_r_format(operands).unwrap()),
            "remuw" => Remuw(IntegerParser::parse_r_format(operands).unwrap()),

            "flw" => Flw(IntegerParser::parse_i_format(operands).unwrap()),
            "fsw" => Fsw(IntegerParser::parse_s_format(operands).unwrap()),
            "fmadd.s" => Fmadds(FloatParser::parse_r4_format(operands).unwrap()),
            "fmsub.s" => Fmsubs(FloatParser::parse_r4_format(operands).unwrap()),
            "fnmsub.s" => Fnmsubs(FloatParser::parse_r4_format(operands).unwrap()),
            "fnmadd.s" => Fnmadds(FloatParser::parse_r4_format(operands).unwrap()),
            "fadd.s" => Fadds(IntegerParser::parse_r_format(operands).unwrap()),
            "fsub.s" => Fsubs(IntegerParser::parse_r_format(operands).unwrap()),
            "fmul.s" => Fmuls(IntegerParser::parse_r_format(operands).unwrap()),
            "fdiv.s" => Fdivs(IntegerParser::parse_r_format(operands).unwrap()),
            "fsqrt.s" => Fsqrts(IntegerParser::parse_r_format(operands).unwrap()),
            "fsgnj.s" => Fsgnjs(IntegerParser::parse_r_format(operands).unwrap()),
            "fsgnjn.s" => Fsgnjns(IntegerParser::parse_r_format(operands).unwrap()),
            "fsgnjx.s" => Fsgnjxs(IntegerParser::parse_r_format(operands).unwrap()),
            "fmin.s" => Fmins(IntegerParser::parse_r_format(operands).unwrap()),
            "fmax.s" => Fmaxs(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.w.s" => Fcvtws(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.wu.s" => Fcvtwus(IntegerParser::parse_r_format(operands).unwrap()),
            "fmv.x.w" => Fmvxw(IntegerParser::parse_r_format(operands).unwrap()),
            "feq.s" => Feqs(IntegerParser::parse_r_format(operands).unwrap()),
            "flt.s" => Flts(IntegerParser::parse_r_format(operands).unwrap()),
            "fle.s" => Fles(IntegerParser::parse_r_format(operands).unwrap()),
            "fclass.s" => Fclasss(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.s.w" => Fcvtsw(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.s.wu" => Fcvtswu(IntegerParser::parse_r_format(operands).unwrap()),
            "fmv.w.x" => Fmvwx(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.l.s" => Fcvtls(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.lu.s" => Fcvtlus(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.s.l" => Fcvtsl(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.s.lu" => Fcvtslu(IntegerParser::parse_r_format(operands).unwrap()),

            "fld" => Fld(IntegerParser::parse_i_format(operands).unwrap()),
            "fsd" => Fsd(IntegerParser::parse_s_format(operands).unwrap()),
            "fmadd.d" => Fmaddd(FloatParser::parse_r4_format(operands).unwrap()),
            "fmsub.d" => Fmsubd(FloatParser::parse_r4_format(operands).unwrap()),
            "fnmsub.d" => Fnmsubd(FloatParser::parse_r4_format(operands).unwrap()),
            "fnmadd.d" => Fnmaddd(FloatParser::parse_r4_format(operands).unwrap()),
            "fadd.d" => Faddd(IntegerParser::parse_r_format(operands).unwrap()),
            "fsub.d" => Fsubd(IntegerParser::parse_r_format(operands).unwrap()),
            "fmul.d" => Fmuld(IntegerParser::parse_r_format(operands).unwrap()),
            "fdiv.d" => Fdivd(IntegerParser::parse_r_format(operands).unwrap()),
            "fsqrt.d" => Fsqrtd(IntegerParser::parse_r_format(operands).unwrap()),
            "fsgnj.d" => Fsgnjd(IntegerParser::parse_r_format(operands).unwrap()),
            "fsgnjn.d" => Fsgnjnd(IntegerParser::parse_r_format(operands).unwrap()),
            "fsgnjx.d" => Fsgnjxd(IntegerParser::parse_r_format(operands).unwrap()),
            "fmin.d" => Fmind(IntegerParser::parse_r_format(operands).unwrap()),
            "fmax.d" => Fmaxd(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.s.d" => Fcvtsd(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.d.s" => Fcvtds(IntegerParser::parse_r_format(operands).unwrap()),
            "feq.d" => Feqd(IntegerParser::parse_r_format(operands).unwrap()),
            "flt.d" => Fltd(IntegerParser::parse_r_format(operands).unwrap()),
            "fle.d" => Fled(IntegerParser::parse_r_format(operands).unwrap()),
            "fclass.d" => Fclassd(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.w.d" => Fcvtwd(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.wu.d" => Fcvtwud(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.d.w" => Fcvtdw(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.d.wu" => Fcvtdwu(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.l.d" => Fcvtld(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.lu.d" => Fcvtlud(IntegerParser::parse_r_format(operands).unwrap()),
            "fmv.x.d" => Fmvxd(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.d.l" => Fcvtdl(IntegerParser::parse_r_format(operands).unwrap()),
            "fcvt.d.lu" => Fcvtdlu(IntegerParser::parse_r_format(operands).unwrap()),
            "fmv.d.x" => Fmvdx(IntegerParser::parse_r_format(operands).unwrap()),

            "vsetvli" => Vsetvli(VectorParser::parse_vsetvli_format(operands).unwrap()),
            "vsetivli" => Vsetivli(VectorParser::parse_vsetivli_format(operands).unwrap()),
            "vsetvl" => Vsetvl(VectorParser::parse_vsetvl_format(operands).unwrap()),

            "vle8.v" => Vlv { eew: 8, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vle16.v" => Vlv { eew: 16, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vle32.v" => Vlv { eew: 32, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vle64.v" => Vlv { eew: 64, data: VectorParser::parse_vl_format(operands).unwrap()},

            "vse8.v" => Vsv { eew: 8, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vse16.v" => Vsv { eew: 16, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vse32.v" => Vsv { eew: 32, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vse64.v" => Vsv { eew: 64, data: VectorParser::parse_vs_format(operands).unwrap()},

            "vlse8.v" => Vlsv { eew: 8, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlse16.v" => Vlsv { eew: 16, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlse32.v" => Vlsv { eew: 32, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlse64.v" => Vlsv { eew: 64, data: VectorParser::parse_vls_format(operands).unwrap()},

            "vsse8.v" => Vssv { eew: 8, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vsse16.v" => Vssv { eew: 16, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vsse32.v" => Vssv { eew: 32, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vsse64.v" => Vssv { eew: 64, data: VectorParser::parse_vss_format(operands).unwrap()},

            "vluxei8.v" => Vluxv { eew: 8, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxei16.v" => Vluxv { eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxei32.v" => Vluxv { eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxei64.v" => Vluxv { eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},

            "vloxei8.v" => Vloxv { eew: 8, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxei16.v" => Vloxv { eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxei32.v" => Vloxv { eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxei64.v" => Vloxv { eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},

            "vsuxei8.v" => Vsuxv { eew: 8, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxei16.v" => Vsuxv { eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxei32.v" => Vsuxv { eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxei64.v" => Vsuxv { eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},

            "vsuxeix8.v" => Vsuxv { eew: 8, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxeix16.v" => Vsuxv { eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxeix32.v" => Vsuxv { eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxeix64.v" => Vsuxv { eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},

            "vle8ff.v" => Vlffv { eew: 8, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vle16ff.v" => Vlffv { eew: 16, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vle32ff.v" => Vlffv { eew: 32, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vle64ff.v" => Vlffv { eew: 64, data: VectorParser::parse_vl_format(operands).unwrap()},

            // Note: I need to list all combinations so that I can research const-generification segmented load/stores in the future

            "vlseg1e8.v"  => Vlsegv { nf: 1, eew: 8,  data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg1e16.v" => Vlsegv { nf: 1, eew: 16, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg1e32.v" => Vlsegv { nf: 1, eew: 32, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg1e64.v" => Vlsegv { nf: 1, eew: 64, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg2e8.v"  => Vlsegv { nf: 2, eew: 8,  data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg2e16.v" => Vlsegv { nf: 2, eew: 16, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg2e32.v" => Vlsegv { nf: 2, eew: 32, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg2e64.v" => Vlsegv { nf: 2, eew: 64, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg3e8.v"  => Vlsegv { nf: 3, eew: 8,  data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg3e16.v" => Vlsegv { nf: 3, eew: 16, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg3e32.v" => Vlsegv { nf: 3, eew: 32, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg3e64.v" => Vlsegv { nf: 3, eew: 64, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg4e8.v"  => Vlsegv { nf: 4, eew: 8,  data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg4e16.v" => Vlsegv { nf: 4, eew: 16, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg4e32.v" => Vlsegv { nf: 4, eew: 32, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg4e64.v" => Vlsegv { nf: 4, eew: 64, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg5e8.v"  => Vlsegv { nf: 5, eew: 8,  data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg5e16.v" => Vlsegv { nf: 5, eew: 16, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg5e32.v" => Vlsegv { nf: 5, eew: 32, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg5e64.v" => Vlsegv { nf: 5, eew: 64, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg6e8.v"  => Vlsegv { nf: 6, eew: 8,  data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg6e16.v" => Vlsegv { nf: 6, eew: 16, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg6e32.v" => Vlsegv { nf: 6, eew: 32, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg6e64.v" => Vlsegv { nf: 6, eew: 64, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg7e8.v"  => Vlsegv { nf: 7, eew: 8,  data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg7e16.v" => Vlsegv { nf: 7, eew: 16, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg7e32.v" => Vlsegv { nf: 7, eew: 32, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg7e64.v" => Vlsegv { nf: 7, eew: 64, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg8e8.v"  => Vlsegv { nf: 8, eew: 8,  data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg8e16.v" => Vlsegv { nf: 8, eew: 16, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg8e32.v" => Vlsegv { nf: 8, eew: 32, data: VectorParser::parse_vl_format(operands).unwrap()},
            "vlseg8e64.v" => Vlsegv { nf: 8, eew: 64, data: VectorParser::parse_vl_format(operands).unwrap()},

            "vsseg1e8.v"  => Vssegv { nf: 1, eew: 8,  data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg1e16.v" => Vssegv { nf: 1, eew: 16, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg1e32.v" => Vssegv { nf: 1, eew: 32, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg1e64.v" => Vssegv { nf: 1, eew: 64, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg2e8.v"  => Vssegv { nf: 2, eew: 8,  data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg2e16.v" => Vssegv { nf: 2, eew: 16, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg2e32.v" => Vssegv { nf: 2, eew: 32, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg2e64.v" => Vssegv { nf: 2, eew: 64, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg3e8.v"  => Vssegv { nf: 3, eew: 8,  data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg3e16.v" => Vssegv { nf: 3, eew: 16, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg3e32.v" => Vssegv { nf: 3, eew: 32, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg3e64.v" => Vssegv { nf: 3, eew: 64, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg4e8.v"  => Vssegv { nf: 4, eew: 8,  data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg4e16.v" => Vssegv { nf: 4, eew: 16, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg4e32.v" => Vssegv { nf: 4, eew: 32, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg4e64.v" => Vssegv { nf: 4, eew: 64, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg5e8.v"  => Vssegv { nf: 5, eew: 8,  data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg5e16.v" => Vssegv { nf: 5, eew: 16, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg5e32.v" => Vssegv { nf: 5, eew: 32, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg5e64.v" => Vssegv { nf: 5, eew: 64, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg6e8.v"  => Vssegv { nf: 6, eew: 8,  data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg6e16.v" => Vssegv { nf: 6, eew: 16, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg6e32.v" => Vssegv { nf: 6, eew: 32, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg6e64.v" => Vssegv { nf: 6, eew: 64, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg7e8.v"  => Vssegv { nf: 7, eew: 8,  data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg7e16.v" => Vssegv { nf: 7, eew: 16, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg7e32.v" => Vssegv { nf: 7, eew: 32, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg7e64.v" => Vssegv { nf: 7, eew: 64, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg8e8.v"  => Vssegv { nf: 8, eew: 8,  data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg8e16.v" => Vssegv { nf: 8, eew: 16, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg8e32.v" => Vssegv { nf: 8, eew: 32, data: VectorParser::parse_vs_format(operands).unwrap()},
            "vsseg8e64.v" => Vssegv { nf: 8, eew: 64, data: VectorParser::parse_vs_format(operands).unwrap()},

            "vlsseg1e8.v"  => Vlssegv { nf: 1, eew: 8,  data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg1e16.v" => Vlssegv { nf: 1, eew: 16, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg1e32.v" => Vlssegv { nf: 1, eew: 32, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg1e64.v" => Vlssegv { nf: 1, eew: 64, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg2e8.v"  => Vlssegv { nf: 2, eew: 8,  data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg2e16.v" => Vlssegv { nf: 2, eew: 16, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg2e32.v" => Vlssegv { nf: 2, eew: 32, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg2e64.v" => Vlssegv { nf: 2, eew: 64, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg3e8.v"  => Vlssegv { nf: 3, eew: 8,  data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg3e16.v" => Vlssegv { nf: 3, eew: 16, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg3e32.v" => Vlssegv { nf: 3, eew: 32, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg3e64.v" => Vlssegv { nf: 3, eew: 64, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg4e8.v"  => Vlssegv { nf: 4, eew: 8,  data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg4e16.v" => Vlssegv { nf: 4, eew: 16, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg4e32.v" => Vlssegv { nf: 4, eew: 32, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg4e64.v" => Vlssegv { nf: 4, eew: 64, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg5e8.v"  => Vlssegv { nf: 5, eew: 8,  data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg5e16.v" => Vlssegv { nf: 5, eew: 16, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg5e32.v" => Vlssegv { nf: 5, eew: 32, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg5e64.v" => Vlssegv { nf: 5, eew: 64, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg6e8.v"  => Vlssegv { nf: 6, eew: 8,  data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg6e16.v" => Vlssegv { nf: 6, eew: 16, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg6e32.v" => Vlssegv { nf: 6, eew: 32, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg6e64.v" => Vlssegv { nf: 6, eew: 64, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg7e8.v"  => Vlssegv { nf: 7, eew: 8,  data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg7e16.v" => Vlssegv { nf: 7, eew: 16, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg7e32.v" => Vlssegv { nf: 7, eew: 32, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg7e64.v" => Vlssegv { nf: 7, eew: 64, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg8e8.v"  => Vlssegv { nf: 8, eew: 8,  data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg8e16.v" => Vlssegv { nf: 8, eew: 16, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg8e32.v" => Vlssegv { nf: 8, eew: 32, data: VectorParser::parse_vls_format(operands).unwrap()},
            "vlsseg8e64.v" => Vlssegv { nf: 8, eew: 64, data: VectorParser::parse_vls_format(operands).unwrap()},
            
            "vssseg1e8.v"  => Vsssegv { nf: 1, eew: 8,  data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg1e16.v" => Vsssegv { nf: 1, eew: 16, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg1e32.v" => Vsssegv { nf: 1, eew: 32, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg1e64.v" => Vsssegv { nf: 1, eew: 64, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg2e8.v"  => Vsssegv { nf: 2, eew: 8,  data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg2e16.v" => Vsssegv { nf: 2, eew: 16, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg2e32.v" => Vsssegv { nf: 2, eew: 32, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg2e64.v" => Vsssegv { nf: 2, eew: 64, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg3e8.v"  => Vsssegv { nf: 3, eew: 8,  data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg3e16.v" => Vsssegv { nf: 3, eew: 16, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg3e32.v" => Vsssegv { nf: 3, eew: 32, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg3e64.v" => Vsssegv { nf: 3, eew: 64, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg4e8.v"  => Vsssegv { nf: 4, eew: 8,  data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg4e16.v" => Vsssegv { nf: 4, eew: 16, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg4e32.v" => Vsssegv { nf: 4, eew: 32, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg4e64.v" => Vsssegv { nf: 4, eew: 64, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg5e8.v"  => Vsssegv { nf: 5, eew: 8,  data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg5e16.v" => Vsssegv { nf: 5, eew: 16, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg5e32.v" => Vsssegv { nf: 5, eew: 32, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg5e64.v" => Vsssegv { nf: 5, eew: 64, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg6e8.v"  => Vsssegv { nf: 6, eew: 8,  data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg6e16.v" => Vsssegv { nf: 6, eew: 16, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg6e32.v" => Vsssegv { nf: 6, eew: 32, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg6e64.v" => Vsssegv { nf: 6, eew: 64, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg7e8.v"  => Vsssegv { nf: 7, eew: 8,  data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg7e16.v" => Vsssegv { nf: 7, eew: 16, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg7e32.v" => Vsssegv { nf: 7, eew: 32, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg7e64.v" => Vsssegv { nf: 7, eew: 64, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg8e8.v"  => Vsssegv { nf: 8, eew: 8,  data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg8e16.v" => Vsssegv { nf: 8, eew: 16, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg8e32.v" => Vsssegv { nf: 8, eew: 32, data: VectorParser::parse_vss_format(operands).unwrap()},
            "vssseg8e64.v" => Vsssegv { nf: 8, eew: 64, data: VectorParser::parse_vss_format(operands).unwrap()},

            "vluxseg1ei8.v"  => Vluxsegv { nf: 1, eew: 8,  data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg1ei16.v" => Vluxsegv { nf: 1, eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg1ei32.v" => Vluxsegv { nf: 1, eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg1ei64.v" => Vluxsegv { nf: 1, eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg2ei8.v"  => Vluxsegv { nf: 2, eew: 8,  data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg2ei16.v" => Vluxsegv { nf: 2, eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg2ei32.v" => Vluxsegv { nf: 2, eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg2ei64.v" => Vluxsegv { nf: 2, eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg3ei8.v"  => Vluxsegv { nf: 3, eew: 8,  data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg3ei16.v" => Vluxsegv { nf: 3, eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg3ei32.v" => Vluxsegv { nf: 3, eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg3ei64.v" => Vluxsegv { nf: 3, eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg4ei8.v"  => Vluxsegv { nf: 4, eew: 8,  data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg4ei16.v" => Vluxsegv { nf: 4, eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg4ei32.v" => Vluxsegv { nf: 4, eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg4ei64.v" => Vluxsegv { nf: 4, eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg5ei8.v"  => Vluxsegv { nf: 5, eew: 8,  data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg5ei16.v" => Vluxsegv { nf: 5, eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg5ei32.v" => Vluxsegv { nf: 5, eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg5ei64.v" => Vluxsegv { nf: 5, eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg6ei8.v"  => Vluxsegv { nf: 6, eew: 8,  data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg6ei16.v" => Vluxsegv { nf: 6, eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg6ei32.v" => Vluxsegv { nf: 6, eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg6ei64.v" => Vluxsegv { nf: 6, eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg7ei8.v"  => Vluxsegv { nf: 7, eew: 8,  data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg7ei16.v" => Vluxsegv { nf: 7, eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg7ei32.v" => Vluxsegv { nf: 7, eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg7ei64.v" => Vluxsegv { nf: 7, eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg8ei8.v"  => Vluxsegv { nf: 8, eew: 8,  data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg8ei16.v" => Vluxsegv { nf: 8, eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg8ei32.v" => Vluxsegv { nf: 8, eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vluxseg8ei64.v" => Vluxsegv { nf: 8, eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},

            "vloxseg1ei8.v"  => Vloxsegv { nf: 1, eew: 8,  data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg1ei16.v" => Vloxsegv { nf: 1, eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg1ei32.v" => Vloxsegv { nf: 1, eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg1ei64.v" => Vloxsegv { nf: 1, eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg2ei8.v"  => Vloxsegv { nf: 2, eew: 8,  data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg2ei16.v" => Vloxsegv { nf: 2, eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg2ei32.v" => Vloxsegv { nf: 2, eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg2ei64.v" => Vloxsegv { nf: 2, eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg3ei8.v"  => Vloxsegv { nf: 3, eew: 8,  data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg3ei16.v" => Vloxsegv { nf: 3, eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg3ei32.v" => Vloxsegv { nf: 3, eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg3ei64.v" => Vloxsegv { nf: 3, eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg4ei8.v"  => Vloxsegv { nf: 4, eew: 8,  data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg4ei16.v" => Vloxsegv { nf: 4, eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg4ei32.v" => Vloxsegv { nf: 4, eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg4ei64.v" => Vloxsegv { nf: 4, eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg5ei8.v"  => Vloxsegv { nf: 5, eew: 8,  data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg5ei16.v" => Vloxsegv { nf: 5, eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg5ei32.v" => Vloxsegv { nf: 5, eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg5ei64.v" => Vloxsegv { nf: 5, eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg6ei8.v"  => Vloxsegv { nf: 6, eew: 8,  data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg6ei16.v" => Vloxsegv { nf: 6, eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg6ei32.v" => Vloxsegv { nf: 6, eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg6ei64.v" => Vloxsegv { nf: 6, eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg7ei8.v"  => Vloxsegv { nf: 7, eew: 8,  data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg7ei16.v" => Vloxsegv { nf: 7, eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg7ei32.v" => Vloxsegv { nf: 7, eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg7ei64.v" => Vloxsegv { nf: 7, eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg8ei8.v"  => Vloxsegv { nf: 8, eew: 8,  data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg8ei16.v" => Vloxsegv { nf: 8, eew: 16, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg8ei32.v" => Vloxsegv { nf: 8, eew: 32, data: VectorParser::parse_vlx_format(operands).unwrap()},
            "vloxseg8ei64.v" => Vloxsegv { nf: 8, eew: 64, data: VectorParser::parse_vlx_format(operands).unwrap()},

            "vsuxseg1ei8.v"  => Vsuxsegv { nf: 1, eew: 8,  data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg1ei16.v" => Vsuxsegv { nf: 1, eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg1ei32.v" => Vsuxsegv { nf: 1, eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg1ei64.v" => Vsuxsegv { nf: 1, eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg2ei8.v"  => Vsuxsegv { nf: 2, eew: 8,  data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg2ei16.v" => Vsuxsegv { nf: 2, eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg2ei32.v" => Vsuxsegv { nf: 2, eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg2ei64.v" => Vsuxsegv { nf: 2, eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg3ei8.v"  => Vsuxsegv { nf: 3, eew: 8,  data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg3ei16.v" => Vsuxsegv { nf: 3, eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg3ei32.v" => Vsuxsegv { nf: 3, eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg3ei64.v" => Vsuxsegv { nf: 3, eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg4ei8.v"  => Vsuxsegv { nf: 4, eew: 8,  data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg4ei16.v" => Vsuxsegv { nf: 4, eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg4ei32.v" => Vsuxsegv { nf: 4, eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg4ei64.v" => Vsuxsegv { nf: 4, eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg5ei8.v"  => Vsuxsegv { nf: 5, eew: 8,  data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg5ei16.v" => Vsuxsegv { nf: 5, eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg5ei32.v" => Vsuxsegv { nf: 5, eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg5ei64.v" => Vsuxsegv { nf: 5, eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg6ei8.v"  => Vsuxsegv { nf: 6, eew: 8,  data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg6ei16.v" => Vsuxsegv { nf: 6, eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg6ei32.v" => Vsuxsegv { nf: 6, eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg6ei64.v" => Vsuxsegv { nf: 6, eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg7ei8.v"  => Vsuxsegv { nf: 7, eew: 8,  data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg7ei16.v" => Vsuxsegv { nf: 7, eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg7ei32.v" => Vsuxsegv { nf: 7, eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg7ei64.v" => Vsuxsegv { nf: 7, eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg8ei8.v"  => Vsuxsegv { nf: 8, eew: 8,  data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg8ei16.v" => Vsuxsegv { nf: 8, eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg8ei32.v" => Vsuxsegv { nf: 8, eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsuxseg8ei64.v" => Vsuxsegv { nf: 8, eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},
            
            "vsoxseg1ei8.v"  => Vsoxsegv { nf: 1, eew: 8,  data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg1ei16.v" => Vsoxsegv { nf: 1, eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg1ei32.v" => Vsoxsegv { nf: 1, eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg1ei64.v" => Vsoxsegv { nf: 1, eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg2ei8.v"  => Vsoxsegv { nf: 2, eew: 8,  data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg2ei16.v" => Vsoxsegv { nf: 2, eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg2ei32.v" => Vsoxsegv { nf: 2, eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg2ei64.v" => Vsoxsegv { nf: 2, eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg3ei8.v"  => Vsoxsegv { nf: 3, eew: 8,  data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg3ei16.v" => Vsoxsegv { nf: 3, eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg3ei32.v" => Vsoxsegv { nf: 3, eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg3ei64.v" => Vsoxsegv { nf: 3, eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg4ei8.v"  => Vsoxsegv { nf: 4, eew: 8,  data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg4ei16.v" => Vsoxsegv { nf: 4, eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg4ei32.v" => Vsoxsegv { nf: 4, eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg4ei64.v" => Vsoxsegv { nf: 4, eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg5ei8.v"  => Vsoxsegv { nf: 5, eew: 8,  data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg5ei16.v" => Vsoxsegv { nf: 5, eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg5ei32.v" => Vsoxsegv { nf: 5, eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg5ei64.v" => Vsoxsegv { nf: 5, eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg6ei8.v"  => Vsoxsegv { nf: 6, eew: 8,  data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg6ei16.v" => Vsoxsegv { nf: 6, eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg6ei32.v" => Vsoxsegv { nf: 6, eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg6ei64.v" => Vsoxsegv { nf: 6, eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg7ei8.v"  => Vsoxsegv { nf: 7, eew: 8,  data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg7ei16.v" => Vsoxsegv { nf: 7, eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg7ei32.v" => Vsoxsegv { nf: 7, eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg7ei64.v" => Vsoxsegv { nf: 7, eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg8ei8.v"  => Vsoxsegv { nf: 8, eew: 8,  data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg8ei16.v" => Vsoxsegv { nf: 8, eew: 16, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg8ei32.v" => Vsoxsegv { nf: 8, eew: 32, data: VectorParser::parse_vsx_format(operands).unwrap()},
            "vsoxseg8ei64.v" => Vsoxsegv { nf: 8, eew: 64, data: VectorParser::parse_vsx_format(operands).unwrap()},

            "vl1re8.v" => Vlrv { nf: 1, eew: 8,  data: VectorParser::parse_vlr_format(operands).unwrap()},
            "vl1re16.v" => Vlrv { nf: 1, eew: 16, data: VectorParser::parse_vlr_format(operands).unwrap()},
            "vl1re32.v" => Vlrv { nf: 1, eew: 32, data: VectorParser::parse_vlr_format(operands).unwrap()},
            "vl1re64.v" => Vlrv { nf: 1, eew: 64, data: VectorParser::parse_vlr_format(operands).unwrap()},
            "vl2re8.v" => Vlrv { nf: 2, eew: 8,  data: VectorParser::parse_vlr_format(operands).unwrap()},
            "vl2re16.v" => Vlrv { nf: 2, eew: 16, data: VectorParser::parse_vlr_format(operands).unwrap()},
            "vl2re32.v" => Vlrv { nf: 2, eew: 32, data: VectorParser::parse_vlr_format(operands).unwrap()},
            "vl2re64.v" => Vlrv { nf: 2, eew: 64, data: VectorParser::parse_vlr_format(operands).unwrap()},
            "vl4re8.v" => Vlrv { nf: 4, eew: 8,  data: VectorParser::parse_vlr_format(operands).unwrap()},
            "vl4re16.v" => Vlrv { nf: 4, eew: 16, data: VectorParser::parse_vlr_format(operands).unwrap()},
            "vl4re32.v" => Vlrv { nf: 4, eew: 32, data: VectorParser::parse_vlr_format(operands).unwrap()},
            "vl4re64.v" => Vlrv { nf: 4, eew: 64, data: VectorParser::parse_vlr_format(operands).unwrap()},
            "vl8re8.v" => Vlrv { nf: 8, eew: 8,  data: VectorParser::parse_vlr_format(operands).unwrap()},
            "vl8re16.v" => Vlrv { nf: 8, eew: 16, data: VectorParser::parse_vlr_format(operands).unwrap()},
            "vl8re32.v" => Vlrv { nf: 8, eew: 32, data: VectorParser::parse_vlr_format(operands).unwrap()},
            "vl8re64.v" => Vlrv { nf: 8, eew: 64, data: VectorParser::parse_vlr_format(operands).unwrap()},

            "vadd.vv" => Vaddvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vadd.vx" => Vaddvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vadd.vi" => Vaddvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vsub.vv" => Vsubvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vsub.vx" => Vsubvx(VectorParser::parse_opivx_format(operands).unwrap()),

            "vrsub.vx" => Vrsubvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vrsub.vi" => Vrsubvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vminu.vv" => Vminuvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vminu.vx" => Vminuvx(VectorParser::parse_opivx_format(operands).unwrap()),

            "vmin.vv" => Vminvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vmin.vx" => Vminvx(VectorParser::parse_opivx_format(operands).unwrap()),

            "vmaxu.vv" => Vmaxuvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vmaxu.vx" => Vmaxuvx(VectorParser::parse_opivx_format(operands).unwrap()),

            "vmax.vv" => Vmaxvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vmax.vx" => Vmaxvx(VectorParser::parse_opivx_format(operands).unwrap()),

            "vand.vv" => Vandvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vand.vx" => Vandvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vand.vi" => Vandvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vor.vv" => Vorvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vor.vx" => Vorvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vor.vi" => Vorvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vxor.vv" => Vxorvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vxor.vx" => Vxorvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vxor.vi" => Vxorvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vrgather.vv" => Vrgathervv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vrgather.vx" => Vrgathervx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vrgather.vi" => Vrgathervi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vrgatherei16.v" => Vrgatherei16vv(VectorParser::parse_opivv_format(operands).unwrap()),

            "vslideup.vx" => Vslideupvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vslideup.vi" => Vslideupvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vslidedown.vx" => Vslidedownvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vslidedown.vi" => Vslidedownvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vadc.vvm" => Vadcvvm(VectorParser::parse_opivv_format(operands).unwrap()),
            "vadc.vxm" => Vadcvxm(VectorParser::parse_opivx_format(operands).unwrap()),
            "vadc.vim" => Vadcvim(VectorParser::parse_opivi_format(operands).unwrap()),

            "vmadc.vvm" => Vmadcvvm(VectorParser::parse_opivv_format(operands).unwrap()),
            "vmadc.vxm" => Vmadcvxm(VectorParser::parse_opivx_format(operands).unwrap()),
            "vmadc.vim" => Vmadcvim(VectorParser::parse_opivi_format(operands).unwrap()),
            "vmadc.vv" => Vmadcvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vmadc.vx" => Vmadcvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vmadc.vi" => Vmadcvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vsbc.vvm" => Vsbcvvm(VectorParser::parse_opivv_format(operands).unwrap()),
            "vsbc.vxm" => Vsbcvxm(VectorParser::parse_opivx_format(operands).unwrap()),

            "vmsbc.vv" => Vmsbcvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vmsbc.vx" => Vmsbcvx(VectorParser::parse_opivx_format(operands).unwrap()),

            "vmerge.vvm" => Vmergevvm(VectorParser::parse_opivv_format(operands).unwrap()),
            "vmerge.vxm" => Vmergevxm(VectorParser::parse_opivx_format(operands).unwrap()),
            "vmerge.vim" => Vmergevim(VectorParser::parse_opivi_format(operands).unwrap()),

            "vmv.v.v" => Vmvvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vmv.v.x" => Vmvvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vmv.v.i" => Vmvvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vmseq.vv" => Vmseqvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vmseq.vx" => Vmseqvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vmseq.vi" => Vmseqvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vmsne.vv" => Vmsnevv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vmsne.vx" => Vmsnevx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vmsne.vi" => Vmsnevi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vmsltu.vv" => Vmsltuvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vmsltu.vx" => Vmsltuvx(VectorParser::parse_opivx_format(operands).unwrap()),

            "vmslt.vv" => Vmsltvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vmslt.vx" => Vmsltvx(VectorParser::parse_opivx_format(operands).unwrap()),

            "vmsleu.vv" => Vmsleuvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vmsleu.vx" => Vmsleuvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vmsleu.vi" => Vmsleuvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vmsle.vv" => Vmslevv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vmsle.vx" => Vmslevx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vmsle.vi" => Vmslevi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vmsgtu.vx" => Vmsgtuvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vmsgtu.vi" => Vmsgtuvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vmsgt.vx" => Vmsgtvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vmsgt.vi" => Vmsgtvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vsaddu.vv" => Vsadduvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vsaddu.vx" => Vsadduvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vsaddu.vi" => Vsadduvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vsadd.vv" => Vsaddvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vsadd.vx" => Vsaddvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vsadd.vi" => Vsaddvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vssubu.vv" => Vssubuvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vssubu.vx" => Vssubuvx(VectorParser::parse_opivx_format(operands).unwrap()),

            "vssub.vv" => Vssubvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vssub.vx" => Vssubvx(VectorParser::parse_opivx_format(operands).unwrap()),

            "vsll.vv" => Vsllvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vsll.vx" => Vsllvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vsll.vi" => Vsllvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vsmul.vv" => Vsmulvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vsmul.vx" => Vsmulvx(VectorParser::parse_opivx_format(operands).unwrap()),

            "vmv1r.v" => Vmv1rv(VectorParser::parse_opivi_format(operands).unwrap()),
            "vmv2r.v" => Vmv2rv(VectorParser::parse_opivi_format(operands).unwrap()),
            "vmv4r.v" => Vmv4rv(VectorParser::parse_opivi_format(operands).unwrap()),
            "vmv8r.v" => Vmv8rv(VectorParser::parse_opivi_format(operands).unwrap()),

            "vsrl.vv" => Vsrlvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vsrl.vx" => Vsrlvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vsrl.vi" => Vsrlvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vsra.vv" => Vsravv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vsra.vx" => Vsravx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vsra.vi" => Vsravi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vssrl.vv" => Vssrlvv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vssrl.vx" => Vssrlvx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vssrl.vi" => Vssrlvi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vssra.vv" => Vssravv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vssra.vx" => Vssravx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vssra.vi" => Vssravi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vnsrl.wv" => Vnsrlwv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vnsrl.wx" => Vnsrlwx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vnsrl.wi" => Vnsrlwi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vnsra.wv" => Vnsrawv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vnsra.wx" => Vnsrawx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vnsra.wi" => Vnsrawi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vnclipu.wv" => Vnclipuwv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vnclipu.wx" => Vnclipuwx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vnclipu.wi" => Vnclipuwi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vnclip.wv" => Vnclipwv(VectorParser::parse_opivv_format(operands).unwrap()),
            "vnclip.wx" => Vnclipwx(VectorParser::parse_opivx_format(operands).unwrap()),
            "vnclip.wi" => Vnclipwi(VectorParser::parse_opivi_format(operands).unwrap()),

            "vwredsumu.vs" => Vwredsumuvs(VectorParser::parse_opivv_format(operands).unwrap()),
            "vwredsum.vs" => Vwredsumvs(VectorParser::parse_opivv_format(operands).unwrap()),

            "vredsum.vs" => Vredsumvs(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vredand.vs" => Vredandvs(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vredor.vs" => Vredorvs(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vredxor.vs" => Vredxorvs(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vredminu.vs" => Vredminuvs(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vredmin.vs" => Vredminvs(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vredmaxu.vs" => Vredmaxuvs(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vredmax.vs" => Vredmaxvs(VectorParser::parse_opmvv_format(operands).unwrap()),

            "vaaddu.vv" => Vaadduvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vaaddu.vx" => Vaadduvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vaadd.vv" => Vaaddvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vaadd.vx" => Vaaddvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vasubu.vv" => Vasubuvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vasubu.vx" => Vasubuvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vasub.vv" => Vasubvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vasub.vx" => Vasubvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vslide1up.vx"  => Vslide1upvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vslide1down.vx" => Vslide1downvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vmv.x.s" => Vmvxs(VectorParser::parse_vwxunary0_format(operands).unwrap()),
            "vcpop.m" => Vcpopm(VectorParser::parse_vwxunary0_format(operands).unwrap()),
            "vfirst.m" => Vfirstm(VectorParser::parse_vwxunary0_format(operands).unwrap()),

            "vmv.s.x" => Vmvsx(VectorParser::parse_vrxunary0_format(operands).unwrap()),

            "vsext.vf2" => Vsextvf2(VectorParser::parse_vxunary0_format(operands).unwrap()),
            "vsext.vf4" => Vsextvf4(VectorParser::parse_vxunary0_format(operands).unwrap()),
            "vsext.vf8" => Vsextvf8(VectorParser::parse_vxunary0_format(operands).unwrap()),

            "vzext.vf2" => Vzextvf2(VectorParser::parse_vxunary0_format(operands).unwrap()),
            "vzext.vf4" => Vzextvf4(VectorParser::parse_vxunary0_format(operands).unwrap()),
            "vzext.vf8" => Vzextvf8(VectorParser::parse_vxunary0_format(operands).unwrap()),

            "vmsbf.m" => Vmsbfm(VectorParser::parse_vmunary0_format(operands).unwrap()),
            "vmsof.m" => Vmsofm(VectorParser::parse_vmunary0_format(operands).unwrap()),
            "vmsif.m" => Vmsifm(VectorParser::parse_vmunary0_format(operands).unwrap()),
            "viota.m" => Viotam(VectorParser::parse_vmunary0_format(operands).unwrap()),
            "vid.v" => Vidv(VectorParser::parse_vmunary0_format(operands).unwrap()),

            "vcompress.vm" => Vcompressvm(VectorParser::parse_opmvv_format(operands).unwrap()),

            "vmandn.mm" => Vmandnmm(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vmand.mm" => Vmandmm(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vmor.mm" => Vmormm(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vmxor.mm" => Vmxormm(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vmorn.mm" => Vmornmm(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vmnand.mm" => Vmnandmm(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vmnor.mm" => Vmnormm(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vmxnor.mm" => Vmxnormm(VectorParser::parse_opmvv_format(operands).unwrap()),

            "vdivu.vv" => Vdivuvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vdivu.vx" => Vdivuvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vdiv.vv" => Vdivvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vdiv.vx" => Vdivvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vremu.vv" => Vremuvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vremu.vx" => Vremuvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vrem.vv" => Vremvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vrem.vx" => Vremvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vmulhu.vv" => Vmulhuvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vmulhu.vx" => Vmulhuvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vmul.vv" => Vmulvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vmul.vx" => Vmulvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vmulhsu.vv" => Vmulhsuvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vmulhsu.vx" => Vmulhsuvx(VectorParser::parse_opmvx_format(operands).unwrap()),
            
            "vmulh.vv" => Vmulhvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vmulh.vx" => Vmulhvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vmadd.vv" => Vmaddvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vmadd.vx" => Vmaddvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vnmsub.vv" => Vnmsubvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vnmsub.vx" => Vnmsubvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vmacc.vv" => Vmaccvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vmacc.vx" => Vmaccvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vnmsac.vv" => Vnmsacvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vnmsac.vx" => Vnmsacvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vwaddu.vv" => Vwadduvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vwaddu.vx" => Vwadduvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vwadd.vv" => Vwaddvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vwadd.vx" => Vwaddvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vwsubu.vv" => Vwsubuvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vwsubu.vx" => Vwsubuvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vwsub.vv" => Vwsubvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vwsub.vx" => Vwsubvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vwaddu.wv" => Vwadduwv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vwaddu.wx" => Vwadduwx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vwadd.wv" => Vwaddwv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vwadd.wx" => Vwaddwx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vwsubu.wv" => Vwsubuwv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vwsubu.wx" => Vwsubuwx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vwsub.wv" => Vwsubwv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vwsub.wx" => Vwsubwx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vwmulu.vv" => Vwmuluvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vwmulu.vx" => Vwmuluvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vwmulsu.vv" => Vwmulsuvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vwmulsu.vx" => Vwmulsuvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vwmul.vv" => Vwmulvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vwmul.vx" => Vwmulvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vwmaccu.vv" => Vwmaccuvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vwmaccu.vx" => Vwmaccuvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vwmacc.vv" => Vwmaccvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vwmacc.vx" => Vwmaccvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vwmaccus.vx" => Vwmaccusvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vwmaccsu.vv" => Vwmaccsuvv(VectorParser::parse_opmvv_format(operands).unwrap()),
            "vwmaccsu.vx" => Vwmaccsuvx(VectorParser::parse_opmvx_format(operands).unwrap()),

            "vfadd.vv" => Vfaddvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfadd.vf" => Vfaddvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfredusum.vs" => Vfredusumvs(VectorParser::parse_opfvv_format(operands).unwrap()),

            "vfsub.vv" => Vfsubvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfsub.vf" => Vfsubvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfredosum.vs" => Vfredosumvs(VectorParser::parse_opfvv_format(operands).unwrap()),

            "vfmin.vv" => Vfminvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfmin.vf" => Vfminvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfredmin.vs" => Vfredminvs(VectorParser::parse_opfvv_format(operands).unwrap()),

            "vfmax.vv" => Vfmaxvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfmax.vf" => Vfmaxvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfredmax.vs" => Vfredmaxvs(VectorParser::parse_opfvv_format(operands).unwrap()),

            "vfsgnj.vv" => Vfsgnjvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfsgnj.vf" => Vfsgnjvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfsgnjn.vv" => Vfsgnjnvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfsgnjn.vf" => Vfsgnjnvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfsgnjx.vv" => Vfsgnjxvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfsgnjx.vf" => Vfsgnjxvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfslide1up.vf" => Vfslide1upvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfslide1down.vf" => Vfslide1downvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfmv.f.s" => Vfmvfs(VectorParser::parse_vwfunary0_format(operands).unwrap()),

            "vfmv.s.f" => Vfmvsf(VectorParser::parse_vrfunary0_format(operands).unwrap()),

            "vfcvt.xu.f.v" => Vfcvtxufv(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfcvt.x.f.v" => Vfcvtxfv(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfcvt.f.xu.v" => Vfcvtfxuv(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfcvt.f.x.v" => Vfcvtfxv(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfcvt.rtz.xu.f.v" => VfcvtRtzxufv(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfcvt.rtz.x.f.v" => VfcvtRtzxfv(VectorParser::parse_vfunary0_format(operands).unwrap()),

            "vfwcvt.xu.f.v" => Vfwcvtxufv(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfwcvt.x.f.v" => Vfwcvtxfv(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfwcvt.f.xu.v" => Vfwcvtfxuv(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfwcvt.f.x.v" => Vfwcvtfxv(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfwcvt.f.f.v" => Vfwcvtffv(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfwcvt.rtz.xu.f.v" => VfwcvtRtzxufv(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfwcvt.rtz.x.f.v" => VfwcvtRtzxfv(VectorParser::parse_vfunary0_format(operands).unwrap()),

            "vfncvt.xu.f.w" => Vfncvtxufw(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfncvt.x.f.w" => Vfncvtxfw(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfncvt.f.xu.w" => Vfncvtfxuw(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfncvt.f.x.w" => Vfncvtfxw(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfncvt.f.f.w" => Vfncvtffw(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfncvt.rod.f.f.w" => VfncvtRodffw(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfncvt.rtz.xu.f.w" => VfncvtRtzxufw(VectorParser::parse_vfunary0_format(operands).unwrap()),
            "vfncvt.rtz.x.f.w" => VfncvtRtzxfw(VectorParser::parse_vfunary0_format(operands).unwrap()),

            "vfsqrt.v" => Vfsqrtv(VectorParser::parse_vfunary1_format(operands).unwrap()),
            "vfrsqrt7.v" => Vfrsqrt7v(VectorParser::parse_vfunary1_format(operands).unwrap()),
            "vfrec7.v" => Vfrec7v(VectorParser::parse_vfunary1_format(operands).unwrap()),
            "vfclass.v" => Vfclassv(VectorParser::parse_vfunary1_format(operands).unwrap()),

            "vfmerge.vfm" => Vfmergevfm(VectorParser::parse_opfvf_format(operands).unwrap()),
            "vfmv.v.f" => Vfmvvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vmfeq.vv" => Vmfeqvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vmfeq.vf" => Vmfeqvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vmfle.vv" => Vmflevv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vmfle.vf" => Vmflevf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vmflt.vv" => Vmfltvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vmflt.vf" => Vmfltvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vmfne.vv" => Vmfnevv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vmfne.vf" => Vmfnevf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vmfgt.vf" => Vmfgtvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vmfge.vf" => Vmfgevf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfdiv.vv" => Vfdivvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfdiv.vf" => Vfdivvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfrdiv.vf" => Vfrdivvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfmul.vv" => Vfmulvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfmul.vf" => Vfmulvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfrsub.vf" => Vfrsubvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfmadd.vv" => Vfmaddvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfmadd.vf" => Vfmaddvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfnmadd.vv" => Vfnmaddvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfnmadd.vf" => Vfnmaddvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfmsub.vv" => Vfmsubvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfmsub.vf" => Vfmsubvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfnmsub.vv" => Vfnmsubvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfnmsub.vf" => Vfnmsubvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfmacc.vv" => Vfmaccvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfmacc.vf" => Vfmaccvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfnmacc.vv" => Vfnmaccvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfnmacc.vf" => Vfnmaccvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfmsac.vv" => Vfmsacvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfmsac.vf" => Vfmsacvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfnmsac.vv" => Vfnmsacvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfnmsac.vf" => Vfnmsacvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfwadd.vv" => Vfwaddvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfwadd.vf" => Vfwaddvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfwredusum.vs" => Vfwredusumvs(VectorParser::parse_opfvv_format(operands).unwrap()),

            "vfwsub.vv" => Vfwsubvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfwsub.vf" => Vfwsubvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfwredosum.vs" => Vfwredosumvs(VectorParser::parse_opfvv_format(operands).unwrap()),

            "vfwadd.wv" => Vfwaddwv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfwadd.wf" => Vfwaddwf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfwsub.wv" => Vfwsubwv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfwsub.wf" => Vfwsubwf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfwmul.vv" => Vfwmulvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfwmul.vf" => Vfwmulvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfwmacc.vv" => Vfwmaccvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfwmacc.vf" => Vfwmaccvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfwnmacc.vv" => Vfwnmaccvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfwnmacc.vf" => Vfwnmaccvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfwmsac.vv" => Vfwmsacvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfwmsac.vf" => Vfwmsacvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            "vfwnmsac.vv" => Vfwnmsacvv(VectorParser::parse_opfvv_format(operands).unwrap()),
            "vfwnmsac.vf" => Vfwnmsacvf(VectorParser::parse_opfvf_format(operands).unwrap()),

            _ => panic!("Unknown mnemonic: {}", mnemonic)
        }
    }

    fn split(instruction_line: &str) -> (&str, &str) {
        let mut lane = instruction_line.splitn(2, char::is_whitespace);
        let mnemonic = lane.next().unwrap_or_default().trim();
        let operands = lane.next().unwrap_or_default().trim();
        (mnemonic, operands)
    }
}