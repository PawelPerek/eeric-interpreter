mod operand;

use eeric::prelude::*;
use Instruction::*;
use format as F;

pub struct Decoder;

impl Decoder {
    pub fn decode(instruction_line: &str) -> eeric::Instruction {
        let (mnemonic, operands) = Self::split(instruction_line);

        match mnemonic {
            "add" => Add(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "addw" => Addw(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "sub" => Sub(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "subw" => Subw(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "addi" => Addi(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "addiw" => Addiw(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "slt" => Slt(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "slti" => Slti(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "sltu" => Sltu(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "sltiu" => Sltiu(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "lui" => Lui(F::U { rd: operands[0], imm20: operands[1] as u64 }),
            "auipc" => Auipc(F::U { rd: operands[0], imm20: operands[1] as u64 }),

            "and" => And(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "or" => Or(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "xor" => Xor(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "andi" => Andi(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "ori" => Ori(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "xori" => Xori(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "sll" => Sll(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "sllw" => Sllw(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "srl" => Srl(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "srlw" => Srlw(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "sra" => Sra(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "sraw" => Sraw(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "slli" => Slli(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "slliw" => Slliw(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "srli" => Srli(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "srliw" => Srliw(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "srai" => Srai(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "sraiw" => Sraiw(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),

            "ld" => Ld(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "lw" => Lw(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "lh" => Lh(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "lb" => Lb(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "lwu" => Lwu(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "lhu" => Lhu(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "lbu" => Lbu(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "sd" => Sd(F::S { rs1: operands[0], rs2: operands[1], imm12: operands[2] as u64 }),
            "sw" => Sw(F::S { rs1: operands[0], rs2: operands[1], imm12: operands[2] as u64 }),
            "sh" => Sh(F::S { rs1: operands[0], rs2: operands[1], imm12: operands[2] as u64 }),
            "sb" => Sb(F::S { rs1: operands[0], rs2: operands[1], imm12: operands[2] as u64 }),

            "beq" => Beq(F::S { rs1: operands[0], rs2: operands[1], imm12: operands[2] as u64 }),
            "bne" => Bne(F::S { rs1: operands[0], rs2: operands[1], imm12: operands[2] as u64 }),
            "bge" => Bge(F::S { rs1: operands[0], rs2: operands[1], imm12: operands[2] as u64 }),
            "bgeu" => Bgeu(F::S { rs1: operands[0], rs2: operands[1], imm12: operands[2] as u64 }),
            "blt" => Blt(F::S { rs1: operands[0], rs2: operands[1], imm12: operands[2] as u64 }),
            "bltu" => Bltu(F::S { rs1: operands[0], rs2: operands[1], imm12: operands[2] as u64 }),
            "jal" => Jal(F::U { rd: operands[0], imm20: operands[1] as u64 }),
            "jalr" => Jalr(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),

            "csrrw" => Csrrw(F::Csrr { rd: operands[0], rs1: operands[1], csr: operands[2] }),
            "csrrs" => Csrrs(F::Csrr { rd: operands[0], rs1: operands[1], csr: operands[2] }),
            "csrrc" => Csrrc(F::Csrr { rd: operands[0], rs1: operands[1], csr: operands[2] }),
            "csrrwi" => Csrrwi(F::Csri { rd: operands[0], uimm: operands[1], csr: operands[2] }),
            "csrrsi" => Csrrsi(F::Csri { rd: operands[0], uimm: operands[1], csr: operands[2] }),
            "csrrci" => Csrrci(F::Csri { rd: operands[0], uimm: operands[1], csr: operands[2] }),

            "mul" => Mul(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "mulh" => Mulh(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "mulhsu" => Mulhsu(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "mulhu" => Mulhu(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "div" => Div(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "divu" => Divu(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "rem" => Rem(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "remu" => Remu(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "mulw" => Mulw(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "divw" => Divw(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "divuw" => Divuw(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "remw" => Remw(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "remuw" => Remuw(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),

            "flw" => Flw(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "fsw" => Fsw(F::S { rs1: operands[0], rs2: operands[1], imm12: operands[2] as u64 }),
            "fmadd.s" => Fmadds(F::R4 { rd: operands[0], rs1: operands[1], rs2: operands[2], rs3: operands[3] }),
            "fmsub.s" => Fmsubs(F::R4 { rd: operands[0], rs1: operands[1], rs2: operands[2], rs3: operands[3] }),
            "fnmsub.s" => Fnmsubs(F::R4 { rd: operands[0], rs1: operands[1], rs2: operands[2], rs3: operands[3] }),
            "fnmadd.s" => Fnmadds(F::R4 { rd: operands[0], rs1: operands[1], rs2: operands[2], rs3: operands[3] }),
            "fadd.s" => Fadds(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fsub.s" => Fsubs(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fmul.s" => Fmuls(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fdiv.s" => Fdivs(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fsqrt.s" => Fsqrts(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fsgnj.s" => Fsgnjs(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fsgnjn.s" => Fsgnjns(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fsgnjx.s" => Fsgnjxs(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fmin.s" => Fmins(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fmax.s" => Fmaxs(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.w.s" => Fcvtws(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.wu.s" => Fcvtwus(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fmv.x.w" => Fmvxw(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "feq.s" => Feqs(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "flt.s" => Flts(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fle.s" => Fles(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fclass.s" => Fclasss(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.s.w" => Fcvtsw(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.s.wu" => Fcvtswu(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fmv.w.x" => Fmvwx(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.l.s" => Fcvtls(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.lu.s" => Fcvtlus(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.s.l" => Fcvtsl(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.s.lu" => Fcvtslu(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),

            "fld" => Fld(F::I { rd: operands[0], rs1: operands[1], imm12: operands[2] as u64 }),
            "fsd" => Fsd(F::S { rs1: operands[0], rs2: operands[1], imm12: operands[2] as u64 }),
            "fmadd.d" => Fmaddd(F::R4 { rd: operands[0], rs1: operands[1], rs2: operands[2], rs3: operands[3] }),
            "fmsub.d" => Fmsubd(F::R4 { rd: operands[0], rs1: operands[1], rs2: operands[2], rs3: operands[3] }),
            "fnmsub.d" => Fnmsubd(F::R4 { rd: operands[0], rs1: operands[1], rs2: operands[2], rs3: operands[3] }),
            "fnmadd.d" => Fnmaddd(F::R4 { rd: operands[0], rs1: operands[1], rs2: operands[2], rs3: operands[3] }),
            "fadd.d" => Faddd(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fsub.d" => Fsubd(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fmul.d" => Fmuld(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fdiv.d" => Fdivd(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fsqrt.d" => Fsqrtd(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fsgnj.d" => Fsgnjd(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fsgnjn.d" => Fsgnjnd(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fsgnjx.d" => Fsgnjxd(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fmin.d" => Fmind(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fmax.d" => Fmaxd(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.s.d" => Fcvtsd(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.d.s" => Fcvtds(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "feq.d" => Feqd(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "flt.d" => Fltd(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fle.d" => Fled(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fclass.d" => Fclassd(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.w.d" => Fcvtwd(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.wu.d" => Fcvtwud(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.d.w" => Fcvtdw(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.d.wu" => Fcvtdwu(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.l.d" => Fcvtld(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.lu.d" => Fcvtlud(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fmv.x.d" => Fmvxd(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.d.l" => Fcvtdl(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fcvt.d.lu" => Fcvtdlu(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),
            "fmv.d.x" => Fmvdx(F::R { rd: operands[0], rs1: operands[1], rs2: operands[2] }),

            "vsetvli" => Vsetvli(F::Vsetvli { rd: operands[0], rs1: operands[1], vtypei: operands[2] as u32 }),
            "vsetivli" => Vsetivli(F::Vsetivli { rd: operands[0], uimm: operands[1] as u32, vtypei: operands[2] as u32 }),
            "vsetvl" => Vsetvl(F::Vsetvl { rd: operands[0], rs1: operands[1], rs2: operands[2] }),

            "vle8.v" => Vlv { eew: 8, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vle16.v" => Vlv { eew: 16, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vle32.v" => Vlv { eew: 32, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vle64.v" => Vlv { eew: 64, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},

            "vse8.v" => Vsv { eew: 8, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vse16.v" => Vsv { eew: 16, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vse32.v" => Vsv { eew: 32, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vse64.v" => Vsv { eew: 64, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},

            "vlse8.v" => Vlsv { eew: 8, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlse16.v" => Vlsv { eew: 16, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlse32.v" => Vlsv { eew: 32, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlse64.v" => Vlsv { eew: 64, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},

            "vsse8.v" => Vssv { eew: 8, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vsse16.v" => Vssv { eew: 16, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vsse32.v" => Vssv { eew: 32, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vsse64.v" => Vssv { eew: 64, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},

            "vluxei8.v" => Vluxv { eew: 8, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxei16.v" => Vluxv { eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxei32.v" => Vluxv { eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxei64.v" => Vluxv { eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},

            "vloxei8.v" => Vloxv { eew: 8, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxei16.v" => Vloxv { eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxei32.v" => Vloxv { eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxei64.v" => Vloxv { eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},

            "vsuxei8.v" => Vsuxv { eew: 8, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxei16.v" => Vsuxv { eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxei32.v" => Vsuxv { eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxei64.v" => Vsuxv { eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},

            "vsuxeix8.v" => Vsuxv { eew: 8, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxeix16.v" => Vsuxv { eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxeix32.v" => Vsuxv { eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxeix64.v" => Vsuxv { eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},

            "vle8ff.v" => Vlffv { eew: 8, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vle16ff.v" => Vlffv { eew: 16, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vle32ff.v" => Vlffv { eew: 32, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vle64ff.v" => Vlffv { eew: 64, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},

            // Note: I need to list all combinations so that I can research const-generification segmented load/stores in the future

            "vlseg1e8.v"  => Vlsegv { nf: 1, eew: 8,  data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg1e16.v" => Vlsegv { nf: 1, eew: 16, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg1e32.v" => Vlsegv { nf: 1, eew: 32, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg1e64.v" => Vlsegv { nf: 1, eew: 64, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg2e8.v"  => Vlsegv { nf: 2, eew: 8,  data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg2e16.v" => Vlsegv { nf: 2, eew: 16, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg2e32.v" => Vlsegv { nf: 2, eew: 32, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg2e64.v" => Vlsegv { nf: 2, eew: 64, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg3e8.v"  => Vlsegv { nf: 3, eew: 8,  data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg3e16.v" => Vlsegv { nf: 3, eew: 16, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg3e32.v" => Vlsegv { nf: 3, eew: 32, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg3e64.v" => Vlsegv { nf: 3, eew: 64, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg4e8.v"  => Vlsegv { nf: 4, eew: 8,  data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg4e16.v" => Vlsegv { nf: 4, eew: 16, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg4e32.v" => Vlsegv { nf: 4, eew: 32, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg4e64.v" => Vlsegv { nf: 4, eew: 64, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg5e8.v"  => Vlsegv { nf: 5, eew: 8,  data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg5e16.v" => Vlsegv { nf: 5, eew: 16, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg5e32.v" => Vlsegv { nf: 5, eew: 32, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg5e64.v" => Vlsegv { nf: 5, eew: 64, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg6e8.v"  => Vlsegv { nf: 6, eew: 8,  data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg6e16.v" => Vlsegv { nf: 6, eew: 16, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg6e32.v" => Vlsegv { nf: 6, eew: 32, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg6e64.v" => Vlsegv { nf: 6, eew: 64, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg7e8.v"  => Vlsegv { nf: 7, eew: 8,  data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg7e16.v" => Vlsegv { nf: 7, eew: 16, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg7e32.v" => Vlsegv { nf: 7, eew: 32, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg7e64.v" => Vlsegv { nf: 7, eew: 64, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg8e8.v"  => Vlsegv { nf: 8, eew: 8,  data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg8e16.v" => Vlsegv { nf: 8, eew: 16, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg8e32.v" => Vlsegv { nf: 8, eew: 32, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vlseg8e64.v" => Vlsegv { nf: 8, eew: 64, data: F::Vl { vd: operands[0], rs1: operands[1], vm: operands.len() == 2 }},

            "vsseg1e8.v"  => Vssegv { nf: 1, eew: 8,  data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg1e16.v" => Vssegv { nf: 1, eew: 16, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg1e32.v" => Vssegv { nf: 1, eew: 32, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg1e64.v" => Vssegv { nf: 1, eew: 64, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg2e8.v"  => Vssegv { nf: 2, eew: 8,  data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg2e16.v" => Vssegv { nf: 2, eew: 16, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg2e32.v" => Vssegv { nf: 2, eew: 32, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg2e64.v" => Vssegv { nf: 2, eew: 64, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg3e8.v"  => Vssegv { nf: 3, eew: 8,  data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg3e16.v" => Vssegv { nf: 3, eew: 16, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg3e32.v" => Vssegv { nf: 3, eew: 32, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg3e64.v" => Vssegv { nf: 3, eew: 64, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg4e8.v"  => Vssegv { nf: 4, eew: 8,  data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg4e16.v" => Vssegv { nf: 4, eew: 16, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg4e32.v" => Vssegv { nf: 4, eew: 32, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg4e64.v" => Vssegv { nf: 4, eew: 64, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg5e8.v"  => Vssegv { nf: 5, eew: 8,  data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg5e16.v" => Vssegv { nf: 5, eew: 16, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg5e32.v" => Vssegv { nf: 5, eew: 32, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg5e64.v" => Vssegv { nf: 5, eew: 64, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg6e8.v"  => Vssegv { nf: 6, eew: 8,  data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg6e16.v" => Vssegv { nf: 6, eew: 16, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg6e32.v" => Vssegv { nf: 6, eew: 32, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg6e64.v" => Vssegv { nf: 6, eew: 64, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg7e8.v"  => Vssegv { nf: 7, eew: 8,  data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg7e16.v" => Vssegv { nf: 7, eew: 16, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg7e32.v" => Vssegv { nf: 7, eew: 32, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg7e64.v" => Vssegv { nf: 7, eew: 64, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg8e8.v"  => Vssegv { nf: 8, eew: 8,  data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg8e16.v" => Vssegv { nf: 8, eew: 16, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg8e32.v" => Vssegv { nf: 8, eew: 32, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},
            "vsseg8e64.v" => Vssegv { nf: 8, eew: 64, data: F::Vs { vs3: operands[0], rs1: operands[1], vm: operands.len() == 2 }},

            "vlsseg1e8.v"  => Vlssegv { nf: 1, eew: 8,  data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg1e16.v" => Vlssegv { nf: 1, eew: 16, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg1e32.v" => Vlssegv { nf: 1, eew: 32, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg1e64.v" => Vlssegv { nf: 1, eew: 64, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg2e8.v"  => Vlssegv { nf: 2, eew: 8,  data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg2e16.v" => Vlssegv { nf: 2, eew: 16, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg2e32.v" => Vlssegv { nf: 2, eew: 32, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg2e64.v" => Vlssegv { nf: 2, eew: 64, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg3e8.v"  => Vlssegv { nf: 3, eew: 8,  data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg3e16.v" => Vlssegv { nf: 3, eew: 16, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg3e32.v" => Vlssegv { nf: 3, eew: 32, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg3e64.v" => Vlssegv { nf: 3, eew: 64, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg4e8.v"  => Vlssegv { nf: 4, eew: 8,  data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg4e16.v" => Vlssegv { nf: 4, eew: 16, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg4e32.v" => Vlssegv { nf: 4, eew: 32, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg4e64.v" => Vlssegv { nf: 4, eew: 64, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg5e8.v"  => Vlssegv { nf: 5, eew: 8,  data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg5e16.v" => Vlssegv { nf: 5, eew: 16, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg5e32.v" => Vlssegv { nf: 5, eew: 32, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg5e64.v" => Vlssegv { nf: 5, eew: 64, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg6e8.v"  => Vlssegv { nf: 6, eew: 8,  data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg6e16.v" => Vlssegv { nf: 6, eew: 16, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg6e32.v" => Vlssegv { nf: 6, eew: 32, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg6e64.v" => Vlssegv { nf: 6, eew: 64, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg7e8.v"  => Vlssegv { nf: 7, eew: 8,  data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg7e16.v" => Vlssegv { nf: 7, eew: 16, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg7e32.v" => Vlssegv { nf: 7, eew: 32, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg7e64.v" => Vlssegv { nf: 7, eew: 64, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg8e8.v"  => Vlssegv { nf: 8, eew: 8,  data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg8e16.v" => Vlssegv { nf: 8, eew: 16, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg8e32.v" => Vlssegv { nf: 8, eew: 32, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vlsseg8e64.v" => Vlssegv { nf: 8, eew: 64, data: F::Vls { vd: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            
            "vssseg1e8.v"  => Vsssegv { nf: 1, eew: 8,  data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg1e16.v" => Vsssegv { nf: 1, eew: 16, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg1e32.v" => Vsssegv { nf: 1, eew: 32, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg1e64.v" => Vsssegv { nf: 1, eew: 64, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg2e8.v"  => Vsssegv { nf: 2, eew: 8,  data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg2e16.v" => Vsssegv { nf: 2, eew: 16, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg2e32.v" => Vsssegv { nf: 2, eew: 32, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg2e64.v" => Vsssegv { nf: 2, eew: 64, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg3e8.v"  => Vsssegv { nf: 3, eew: 8,  data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg3e16.v" => Vsssegv { nf: 3, eew: 16, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg3e32.v" => Vsssegv { nf: 3, eew: 32, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg3e64.v" => Vsssegv { nf: 3, eew: 64, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg4e8.v"  => Vsssegv { nf: 4, eew: 8,  data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg4e16.v" => Vsssegv { nf: 4, eew: 16, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg4e32.v" => Vsssegv { nf: 4, eew: 32, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg4e64.v" => Vsssegv { nf: 4, eew: 64, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg5e8.v"  => Vsssegv { nf: 5, eew: 8,  data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg5e16.v" => Vsssegv { nf: 5, eew: 16, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg5e32.v" => Vsssegv { nf: 5, eew: 32, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg5e64.v" => Vsssegv { nf: 5, eew: 64, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg6e8.v"  => Vsssegv { nf: 6, eew: 8,  data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg6e16.v" => Vsssegv { nf: 6, eew: 16, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg6e32.v" => Vsssegv { nf: 6, eew: 32, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg6e64.v" => Vsssegv { nf: 6, eew: 64, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg7e8.v"  => Vsssegv { nf: 7, eew: 8,  data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg7e16.v" => Vsssegv { nf: 7, eew: 16, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg7e32.v" => Vsssegv { nf: 7, eew: 32, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg7e64.v" => Vsssegv { nf: 7, eew: 64, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg8e8.v"  => Vsssegv { nf: 8, eew: 8,  data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg8e16.v" => Vsssegv { nf: 8, eew: 16, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg8e32.v" => Vsssegv { nf: 8, eew: 32, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},
            "vssseg8e64.v" => Vsssegv { nf: 8, eew: 64, data: F::Vss { vs3: operands[0], rs1: operands[1], rs2: operands[2], vm: operands.len() == 3 }},

            "vluxseg1ei8.v"  => Vluxsegv { nf: 1, eew: 8,  data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg1ei16.v" => Vluxsegv { nf: 1, eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg1ei32.v" => Vluxsegv { nf: 1, eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg1ei64.v" => Vluxsegv { nf: 1, eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg2ei8.v"  => Vluxsegv { nf: 2, eew: 8,  data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg2ei16.v" => Vluxsegv { nf: 2, eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg2ei32.v" => Vluxsegv { nf: 2, eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg2ei64.v" => Vluxsegv { nf: 2, eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg3ei8.v"  => Vluxsegv { nf: 3, eew: 8,  data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg3ei16.v" => Vluxsegv { nf: 3, eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg3ei32.v" => Vluxsegv { nf: 3, eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg3ei64.v" => Vluxsegv { nf: 3, eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg4ei8.v"  => Vluxsegv { nf: 4, eew: 8,  data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg4ei16.v" => Vluxsegv { nf: 4, eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg4ei32.v" => Vluxsegv { nf: 4, eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg4ei64.v" => Vluxsegv { nf: 4, eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg5ei8.v"  => Vluxsegv { nf: 5, eew: 8,  data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg5ei16.v" => Vluxsegv { nf: 5, eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg5ei32.v" => Vluxsegv { nf: 5, eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg5ei64.v" => Vluxsegv { nf: 5, eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg6ei8.v"  => Vluxsegv { nf: 6, eew: 8,  data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg6ei16.v" => Vluxsegv { nf: 6, eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg6ei32.v" => Vluxsegv { nf: 6, eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg6ei64.v" => Vluxsegv { nf: 6, eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg7ei8.v"  => Vluxsegv { nf: 7, eew: 8,  data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg7ei16.v" => Vluxsegv { nf: 7, eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg7ei32.v" => Vluxsegv { nf: 7, eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg7ei64.v" => Vluxsegv { nf: 7, eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg8ei8.v"  => Vluxsegv { nf: 8, eew: 8,  data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg8ei16.v" => Vluxsegv { nf: 8, eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg8ei32.v" => Vluxsegv { nf: 8, eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vluxseg8ei64.v" => Vluxsegv { nf: 8, eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},

            "vloxseg1ei8.v"  => Vloxsegv { nf: 1, eew: 8,  data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg1ei16.v" => Vloxsegv { nf: 1, eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg1ei32.v" => Vloxsegv { nf: 1, eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg1ei64.v" => Vloxsegv { nf: 1, eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg2ei8.v"  => Vloxsegv { nf: 2, eew: 8,  data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg2ei16.v" => Vloxsegv { nf: 2, eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg2ei32.v" => Vloxsegv { nf: 2, eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg2ei64.v" => Vloxsegv { nf: 2, eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg3ei8.v"  => Vloxsegv { nf: 3, eew: 8,  data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg3ei16.v" => Vloxsegv { nf: 3, eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg3ei32.v" => Vloxsegv { nf: 3, eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg3ei64.v" => Vloxsegv { nf: 3, eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg4ei8.v"  => Vloxsegv { nf: 4, eew: 8,  data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg4ei16.v" => Vloxsegv { nf: 4, eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg4ei32.v" => Vloxsegv { nf: 4, eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg4ei64.v" => Vloxsegv { nf: 4, eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg5ei8.v"  => Vloxsegv { nf: 5, eew: 8,  data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg5ei16.v" => Vloxsegv { nf: 5, eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg5ei32.v" => Vloxsegv { nf: 5, eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg5ei64.v" => Vloxsegv { nf: 5, eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg6ei8.v"  => Vloxsegv { nf: 6, eew: 8,  data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg6ei16.v" => Vloxsegv { nf: 6, eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg6ei32.v" => Vloxsegv { nf: 6, eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg6ei64.v" => Vloxsegv { nf: 6, eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg7ei8.v"  => Vloxsegv { nf: 7, eew: 8,  data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg7ei16.v" => Vloxsegv { nf: 7, eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg7ei32.v" => Vloxsegv { nf: 7, eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg7ei64.v" => Vloxsegv { nf: 7, eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg8ei8.v"  => Vloxsegv { nf: 8, eew: 8,  data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg8ei16.v" => Vloxsegv { nf: 8, eew: 16, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg8ei32.v" => Vloxsegv { nf: 8, eew: 32, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vloxseg8ei64.v" => Vloxsegv { nf: 8, eew: 64, data: F::Vlx { vd: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},

            "vsuxseg1ei8.v"  => Vsuxsegv { nf: 1, eew: 8,  data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg1ei16.v" => Vsuxsegv { nf: 1, eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg1ei32.v" => Vsuxsegv { nf: 1, eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg1ei64.v" => Vsuxsegv { nf: 1, eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg2ei8.v"  => Vsuxsegv { nf: 2, eew: 8,  data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg2ei16.v" => Vsuxsegv { nf: 2, eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg2ei32.v" => Vsuxsegv { nf: 2, eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg2ei64.v" => Vsuxsegv { nf: 2, eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg3ei8.v"  => Vsuxsegv { nf: 3, eew: 8,  data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg3ei16.v" => Vsuxsegv { nf: 3, eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg3ei32.v" => Vsuxsegv { nf: 3, eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg3ei64.v" => Vsuxsegv { nf: 3, eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg4ei8.v"  => Vsuxsegv { nf: 4, eew: 8,  data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg4ei16.v" => Vsuxsegv { nf: 4, eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg4ei32.v" => Vsuxsegv { nf: 4, eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg4ei64.v" => Vsuxsegv { nf: 4, eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg5ei8.v"  => Vsuxsegv { nf: 5, eew: 8,  data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg5ei16.v" => Vsuxsegv { nf: 5, eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg5ei32.v" => Vsuxsegv { nf: 5, eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg5ei64.v" => Vsuxsegv { nf: 5, eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg6ei8.v"  => Vsuxsegv { nf: 6, eew: 8,  data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg6ei16.v" => Vsuxsegv { nf: 6, eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg6ei32.v" => Vsuxsegv { nf: 6, eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg6ei64.v" => Vsuxsegv { nf: 6, eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg7ei8.v"  => Vsuxsegv { nf: 7, eew: 8,  data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg7ei16.v" => Vsuxsegv { nf: 7, eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg7ei32.v" => Vsuxsegv { nf: 7, eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg7ei64.v" => Vsuxsegv { nf: 7, eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg8ei8.v"  => Vsuxsegv { nf: 8, eew: 8,  data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg8ei16.v" => Vsuxsegv { nf: 8, eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg8ei32.v" => Vsuxsegv { nf: 8, eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsuxseg8ei64.v" => Vsuxsegv { nf: 8, eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            
            "vsoxseg1ei8.v"  => Vsoxsegv { nf: 1, eew: 8,  data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg1ei16.v" => Vsoxsegv { nf: 1, eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg1ei32.v" => Vsoxsegv { nf: 1, eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg1ei64.v" => Vsoxsegv { nf: 1, eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg2ei8.v"  => Vsoxsegv { nf: 2, eew: 8,  data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg2ei16.v" => Vsoxsegv { nf: 2, eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg2ei32.v" => Vsoxsegv { nf: 2, eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg2ei64.v" => Vsoxsegv { nf: 2, eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg3ei8.v"  => Vsoxsegv { nf: 3, eew: 8,  data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg3ei16.v" => Vsoxsegv { nf: 3, eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg3ei32.v" => Vsoxsegv { nf: 3, eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg3ei64.v" => Vsoxsegv { nf: 3, eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg4ei8.v"  => Vsoxsegv { nf: 4, eew: 8,  data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg4ei16.v" => Vsoxsegv { nf: 4, eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg4ei32.v" => Vsoxsegv { nf: 4, eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg4ei64.v" => Vsoxsegv { nf: 4, eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg5ei8.v"  => Vsoxsegv { nf: 5, eew: 8,  data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg5ei16.v" => Vsoxsegv { nf: 5, eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg5ei32.v" => Vsoxsegv { nf: 5, eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg5ei64.v" => Vsoxsegv { nf: 5, eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg6ei8.v"  => Vsoxsegv { nf: 6, eew: 8,  data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg6ei16.v" => Vsoxsegv { nf: 6, eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg6ei32.v" => Vsoxsegv { nf: 6, eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg6ei64.v" => Vsoxsegv { nf: 6, eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg7ei8.v"  => Vsoxsegv { nf: 7, eew: 8,  data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg7ei16.v" => Vsoxsegv { nf: 7, eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg7ei32.v" => Vsoxsegv { nf: 7, eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg7ei64.v" => Vsoxsegv { nf: 7, eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg8ei8.v"  => Vsoxsegv { nf: 8, eew: 8,  data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg8ei16.v" => Vsoxsegv { nf: 8, eew: 16, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg8ei32.v" => Vsoxsegv { nf: 8, eew: 32, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},
            "vsoxseg8ei64.v" => Vsoxsegv { nf: 8, eew: 64, data: F::Vsx { vs3: operands[0], rs1: operands[1], vs2: operands[2], vm: operands.len() == 3 }},

            "vl1re8.v" => Vlrv { nf: 1, eew: 8,  data: F::Vlr { vd: operands[0], rs1: operands[1] }},
            "vl1re16.v" => Vlrv { nf: 1, eew: 16, data: F::Vlr { vd: operands[0], rs1: operands[1] }},
            "vl1re32.v" => Vlrv { nf: 1, eew: 32, data: F::Vlr { vd: operands[0], rs1: operands[1] }},
            "vl1re64.v" => Vlrv { nf: 1, eew: 64, data: F::Vlr { vd: operands[0], rs1: operands[1] }},
            "vl2re8.v" => Vlrv { nf: 2, eew: 8,  data: F::Vlr { vd: operands[0], rs1: operands[1] }},
            "vl2re16.v" => Vlrv { nf: 2, eew: 16, data: F::Vlr { vd: operands[0], rs1: operands[1] }},
            "vl2re32.v" => Vlrv { nf: 2, eew: 32, data: F::Vlr { vd: operands[0], rs1: operands[1] }},
            "vl2re64.v" => Vlrv { nf: 2, eew: 64, data: F::Vlr { vd: operands[0], rs1: operands[1] }},
            "vl4re8.v" => Vlrv { nf: 4, eew: 8,  data: F::Vlr { vd: operands[0], rs1: operands[1] }},
            "vl4re16.v" => Vlrv { nf: 4, eew: 16, data: F::Vlr { vd: operands[0], rs1: operands[1] }},
            "vl4re32.v" => Vlrv { nf: 4, eew: 32, data: F::Vlr { vd: operands[0], rs1: operands[1] }},
            "vl4re64.v" => Vlrv { nf: 4, eew: 64, data: F::Vlr { vd: operands[0], rs1: operands[1] }},
            "vl8re8.v" => Vlrv { nf: 8, eew: 8,  data: F::Vlr { vd: operands[0], rs1: operands[1] }},
            "vl8re16.v" => Vlrv { nf: 8, eew: 16, data: F::Vlr { vd: operands[0], rs1: operands[1] }},
            "vl8re32.v" => Vlrv { nf: 8, eew: 32, data: F::Vlr { vd: operands[0], rs1: operands[1] }},
            "vl8re64.v" => Vlrv { nf: 8, eew: 64, data: F::Vlr { vd: operands[0], rs1: operands[1] }},

            "vadd.vv" => Vaddvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vadd.vx" => Vaddvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vadd.vi" => Vaddvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vsub.vv" => Vsubvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vsub.vx" => Vsubvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vrsub.vx" => Vrsubvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vrsub.vi" => Vrsubvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vminu.vv" => Vminuvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vminu.vx" => Vminuvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmin.vv" => Vminvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmin.vx" => Vminvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmaxu.vv" => Vmaxuvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmaxu.vx" => Vmaxuvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmax.vv" => Vmaxvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmax.vx" => Vmaxvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vand.vv" => Vandvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vand.vx" => Vandvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vand.vi" => Vandvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vor.vv" => Vorvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vor.vx" => Vorvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vor.vi" => Vorvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vxor.vv" => Vxorvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vxor.vx" => Vxorvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vxor.vi" => Vxorvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vrgather.vv" => Vrgathervv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vrgather.vx" => Vrgathervx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vrgather.vi" => Vrgathervi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vrgatherei16.v" => Vrgatherei16vv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),

            "vslideup.vx" => Vslideupvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vslideup.vi" => Vslideupvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vslidedown.vx" => Vslidedownvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vslidedown.vi" => Vslidedownvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vadc.vvm" => Vadcvvm(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vadc.vxm" => Vadcvxm(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vadc.vim" => Vadcvim(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vmadc.vvm" => Vmadcvvm(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmadc.vxm" => Vmadcvxm(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vmadc.vim" => Vmadcvim(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),
            "vmadc.vv" => Vmadcvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmadc.vx" => Vmadcvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vmadc.vi" => Vmadcvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vsbc.vvm" => Vsbcvvm(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vsbc.vxm" => Vsbcvxm(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmsbc.vv" => Vmsbcvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmsbc.vx" => Vmsbcvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmerge.vvm" => Vmergevvm(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmerge.vxm" => Vmergevxm(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vmerge.vim" => Vmergevim(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vmv.v.v" => Vmvvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmv.v.x" => Vmvvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vmv.v.i" => Vmvvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vmseq.vv" => Vmseqvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmseq.vx" => Vmseqvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vmseq.vi" => Vmseqvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vmsne.vv" => Vmsnevv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmsne.vx" => Vmsnevx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vmsne.vi" => Vmsnevi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vmsltu.vv" => Vmsltuvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmsltu.vx" => Vmsltuvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmslt.vv" => Vmsltvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmslt.vx" => Vmsltvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmsleu.vv" => Vmsleuvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmsleu.vx" => Vmsleuvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vmsleu.vi" => Vmsleuvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vmsle.vv" => Vmslevv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmsle.vx" => Vmslevx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vmsle.vi" => Vmslevi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vmsgtu.vx" => Vmsgtuvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vmsgtu.vi" => Vmsgtuvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vmsgt.vx" => Vmsgtvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vmsgt.vi" => Vmsgtvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vsaddu.vv" => Vsadduvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vsaddu.vx" => Vsadduvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vsaddu.vi" => Vsadduvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vsadd.vv" => Vsaddvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vsadd.vx" => Vsaddvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vsadd.vi" => Vsaddvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vssubu.vv" => Vssubuvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vssubu.vx" => Vssubuvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vssub.vv" => Vssubvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vssub.vx" => Vssubvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vsll.vv" => Vsllvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vsll.vx" => Vsllvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vsll.vi" => Vsllvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vsmul.vv" => Vsmulvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vsmul.vx" => Vsmulvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmv1r.v" => Vmv1rv(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),
            "vmv2r.v" => Vmv2rv(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),
            "vmv4r.v" => Vmv4rv(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),
            "vmv8r.v" => Vmv8rv(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vsrl.vv" => Vsrlvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vsrl.vx" => Vsrlvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vsrl.vi" => Vsrlvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vsra.vv" => Vsravv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vsra.vx" => Vsravx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vsra.vi" => Vsravi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vssrl.vv" => Vssrlvv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vssrl.vx" => Vssrlvx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vssrl.vi" => Vssrlvi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vssra.vv" => Vssravv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vssra.vx" => Vssravx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vssra.vi" => Vssravi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vnsrl.wv" => Vnsrlwv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vnsrl.wx" => Vnsrlwx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vnsrl.wi" => Vnsrlwi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vnsra.wv" => Vnsrawv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vnsra.wx" => Vnsrawx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vnsra.wi" => Vnsrawi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vnclipu.wv" => Vnclipuwv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vnclipu.wx" => Vnclipuwx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vnclipu.wi" => Vnclipuwi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vnclip.wv" => Vnclipwv(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vnclip.wx" => Vnclipwx(F::Opivx { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vnclip.wi" => Vnclipwi(F::Opivi { vd: operands[0], vs2: operands[1], imm5: operands[2] as u64, vm: operands.len() == 3 }),

            "vwredsumu.vs" => Vwredsumuvs(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vwredsum.vs" => Vwredsumvs(F::Opivv { vd: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),

            "vredsum.vs" => Vredsumvs(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vredand.vs" => Vredandvs(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vredor.vs" => Vredorvs(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vredxor.vs" => Vredxorvs(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vredminu.vs" => Vredminuvs(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vredmin.vs" => Vredminvs(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vredmaxu.vs" => Vredmaxuvs(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vredmax.vs" => Vredmaxvs(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),

            "vaaddu.vv" => Vaadduvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vaaddu.vx" => Vaadduvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vaadd.vv" => Vaaddvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vaadd.vx" => Vaaddvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vasubu.vv" => Vasubuvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vasubu.vx" => Vasubuvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vasub.vv" => Vasubvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vasub.vx" => Vasubvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vslide1up.vx"  => Vslide1upvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vslide1down.vx" => Vslide1downvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmv.x.s" => Vmvxs(F::Vwxunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: true }),
            "vcpop.m" => Vcpopm(F::Vwxunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfirst.m" => Vfirstm(F::Vwxunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),

            "vmv.s.x" => Vmvsx(F::Vrxunary0 { dest: operands[0], vs2: 0, rs1: operands[1], vm: true }),

            "vsext.vf2" => Vsextvf2(F::Vxunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vsext.vf4" => Vsextvf4(F::Vxunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vsext.vf8" => Vsextvf8(F::Vxunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),

            "vzext.vf2" => Vzextvf2(F::Vxunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vzext.vf4" => Vzextvf4(F::Vxunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vzext.vf8" => Vzextvf8(F::Vxunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),

            "vmsbf.m" => Vmsbfm(F::Vmunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vmsof.m" => Vmsofm(F::Vmunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vmsif.m" => Vmsifm(F::Vmunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "viota.m" => Viotam(F::Vmunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vid.v" => Vidv(F::Vmunary0 { dest: operands[0], vs2: 0, vs1: 0, vm: operands.len() == 1 }),

            "vcompress.vm" => Vcompressvm(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: true }),

            "vmandn.mm" => Vmandnmm(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmand.mm" => Vmandmm(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmor.mm" => Vmormm(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmxor.mm" => Vmxormm(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmorn.mm" => Vmornmm(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmnand.mm" => Vmnandmm(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmnor.mm" => Vmnormm(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmxnor.mm" => Vmxnormm(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),

            "vdivu.vv" => Vdivuvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vdivu.vx" => Vdivuvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vdiv.vv" => Vdivvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vdiv.vx" => Vdivvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vremu.vv" => Vremuvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vremu.vx" => Vremuvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vrem.vv" => Vremvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vrem.vx" => Vremvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmulhu.vv" => Vmulhuvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmulhu.vx" => Vmulhuvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmul.vv" => Vmulvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmul.vx" => Vmulvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmulhsu.vv" => Vmulhsuvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmulhsu.vx" => Vmulhsuvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            
            "vmulh.vv" => Vmulhvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmulh.vx" => Vmulhvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmadd.vv" => Vmaddvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmadd.vx" => Vmaddvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vnmsub.vv" => Vnmsubvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vnmsub.vx" => Vnmsubvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmacc.vv" => Vmaccvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmacc.vx" => Vmaccvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vnmsac.vv" => Vnmsacvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vnmsac.vx" => Vnmsacvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vwaddu.vv" => Vwadduvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vwaddu.vx" => Vwadduvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vwadd.vv" => Vwaddvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vwadd.vx" => Vwaddvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vwsubu.vv" => Vwsubuvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vwsubu.vx" => Vwsubuvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vwsub.vv" => Vwsubvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vwsub.vx" => Vwsubvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vwaddu.wv" => Vwadduwv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vwaddu.wx" => Vwadduwx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vwadd.wv" => Vwaddwv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vwadd.wx" => Vwaddwx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vwsubu.wv" => Vwsubuwv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vwsubu.wx" => Vwsubuwx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vwsub.wv" => Vwsubwv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vwsub.wx" => Vwsubwx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vwmulu.vv" => Vwmuluvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vwmulu.vx" => Vwmuluvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vwmulsu.vv" => Vwmulsuvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vwmulsu.vx" => Vwmulsuvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vwmul.vv" => Vwmulvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vwmul.vx" => Vwmulvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vwmaccu.vv" => Vwmaccuvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vwmaccu.vx" => Vwmaccuvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vwmacc.vv" => Vwmaccvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vwmacc.vx" => Vwmaccvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vwmaccus.vx" => Vwmaccusvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vwmaccsu.vv" => Vwmaccsuvv(F::Opmvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vwmaccsu.vx" => Vwmaccsuvx(F::Opmvx { dest: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfadd.vv" => Vfaddvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfadd.vf" => Vfaddvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfredusum.vs" => Vfredusumvs(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),

            "vfsub.vv" => Vfsubvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfsub.vf" => Vfsubvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfredosum.vs" => Vfredosumvs(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),

            "vfmin.vv" => Vfminvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfmin.vf" => Vfminvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfredmin.vs" => Vfredminvs(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),

            "vfmax.vv" => Vfmaxvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfmax.vf" => Vfmaxvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfredmax.vs" => Vfredmaxvs(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),

            "vfsgnj.vv" => Vfsgnjvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfsgnj.vf" => Vfsgnjvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfsgnjn.vv" => Vfsgnjnvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfsgnjn.vf" => Vfsgnjnvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfsgnjx.vv" => Vfsgnjxvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfsgnjx.vf" => Vfsgnjxvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfslide1up.vf" => Vfslide1upvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfslide1down.vf" => Vfslide1downvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfmv.f.s" => Vfmvfs(F::Vwfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: true }),

            "vfmv.s.f" => Vfmvsf(F::Vrfunary0 { vd: operands[0], vs2: 0, rs1: operands[1], vm: true }),

            "vfcvt.xu.f.v" => Vfcvtxufv(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfcvt.x.f.v" => Vfcvtxfv(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfcvt.f.xu.v" => Vfcvtfxuv(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfcvt.f.x.v" => Vfcvtfxv(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfcvt.rtz.xu.f.v" => VfcvtRtzxufv(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfcvt.rtz.x.f.v" => VfcvtRtzxfv(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),

            "vfwcvt.xu.f.v" => Vfwcvtxufv(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfwcvt.x.f.v" => Vfwcvtxfv(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfwcvt.f.xu.v" => Vfwcvtfxuv(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfwcvt.f.x.v" => Vfwcvtfxv(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfwcvt.f.f.v" => Vfwcvtffv(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfwcvt.rtz.xu.f.v" => VfwcvtRtzxufv(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfwcvt.rtz.x.f.v" => VfwcvtRtzxfv(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),

            "vfncvt.xu.f.w" => Vfncvtxufw(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfncvt.x.f.w" => Vfncvtxfw(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfncvt.f.xu.w" => Vfncvtfxuw(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfncvt.f.x.w" => Vfncvtfxw(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfncvt.f.f.w" => Vfncvtffw(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfncvt.rod.f.f.w" => VfncvtRodffw(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfncvt.rtz.xu.f.w" => VfncvtRtzxufw(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfncvt.rtz.x.f.w" => VfncvtRtzxfw(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),

            "vfsqrt.v" => Vfsqrtv(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfrsqrt7.v" => Vfrsqrt7v(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfrec7.v" => Vfrec7v(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),
            "vfclass.v" => Vfclassv(F::Vfunary0 { dest: operands[0], vs2: operands[1], vs1: 0, vm: operands.len() == 2 }),

            "vfmerge.vfm" => Vfmergevfm(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),
            "vfmv.v.f" => Vfmvvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmfeq.vv" => Vmfeqvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmfeq.vf" => Vmfeqvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmfle.vv" => Vmflevv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmfle.vf" => Vmflevf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmflt.vv" => Vmfltvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmflt.vf" => Vmfltvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmfne.vv" => Vmfnevv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vmfne.vf" => Vmfnevf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmfgt.vf" => Vmfgtvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vmfge.vf" => Vmfgevf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfdiv.vv" => Vfdivvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfdiv.vf" => Vfdivvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfrdiv.vf" => Vfrdivvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfmul.vv" => Vfmulvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfmul.vf" => Vfmulvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfrsub.vf" => Vfrsubvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfmadd.vv" => Vfmaddvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfmadd.vf" => Vfmaddvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfnmadd.vv" => Vfnmaddvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfnmadd.vf" => Vfnmaddvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfmsub.vv" => Vfmsubvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfmsub.vf" => Vfmsubvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfnmsub.vv" => Vfnmsubvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfnmsub.vf" => Vfnmsubvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfmacc.vv" => Vfmaccvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfmacc.vf" => Vfmaccvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfnmacc.vv" => Vfnmaccvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfnmacc.vf" => Vfnmaccvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfmsac.vv" => Vfmsacvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfmsac.vf" => Vfmsacvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfnmsac.vv" => Vfnmsacvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfnmsac.vf" => Vfnmsacvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfwadd.vv" => Vfwaddvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfwadd.vf" => Vfwaddvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfwredusum.vs" => Vfwredusumvs(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),

            "vfwsub.vv" => Vfwsubvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfwsub.vf" => Vfwsubvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfwredosum.vs" => Vfwredosumvs(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),

            "vfwadd.wv" => Vfwaddwv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfwadd.wf" => Vfwaddwf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfwsub.wv" => Vfwsubwv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfwsub.wf" => Vfwsubwf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfwmul.vv" => Vfwmulvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfwmul.vf" => Vfwmulvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfwmacc.vv" => Vfwmaccvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfwmacc.vf" => Vfwmaccvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfwnmacc.vv" => Vfwnmaccvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfwnmacc.vf" => Vfwnmaccvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfwmsac.vv" => Vfwmsacvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfwmsac.vf" => Vfwmsacvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            "vfwnmsac.vv" => Vfwnmsacvv(F::Opfvv { dest: operands[0], vs2: operands[1], vs1: operands[2], vm: operands.len() == 3 }),
            "vfwnmsac.vf" => Vfwnmsacvf(F::Opfvf { vd: operands[0], vs2: operands[1], rs1: operands[2], vm: operands.len() == 3 }),

            _ => panic!("Unknown mnemonic: {}", mnemonic)
        }
    }

    fn split(instruction_line: &str) -> (&str, Vec<usize>) {
        let mut lane = instruction_line.split_ascii_whitespace();
        let mnemonic = lane.next().unwrap();
        let operands = lane
            .map(|s| s.trim_end_matches(','))
            .map(operand::parse_operand)
            .map(Result::unwrap)
            .collect();

        (mnemonic, operands)
    }

}