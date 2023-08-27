
pub fn parse_operand(str_repr: &str) -> Result<usize, &'static str> {
    let mut tidy_str = str_repr.trim().to_lowercase().trim_end_matches(",").to_string();

    if tidy_str.starts_with("(") && tidy_str.ends_with(")") {
        tidy_str = tidy_str[1..tidy_str.len() - 1].to_string();
    }

    match tidy_str.to_lowercase().as_str() {
        "x0" | "zero" | "f0" | "ft0" | "v0" | "v0.t" => Ok(0),
        "x1" | "ra" | "f1" | "ft1" | "v1" => Ok(1),
        "x2" | "sp" | "f2" | "ft2" | "v2" => Ok(2),
        "x3" | "gp" | "f3" | "ft3" | "v3" => Ok(3),
        "x4" | "tp" | "f4" | "ft4" | "v4" => Ok(4),
        "x5" | "t0" | "f5" | "ft5" | "v5" => Ok(5),
        "x6" | "t1" | "f6" | "ft6" | "v6" => Ok(6),
        "x7" | "t2" | "f7" | "ft7" | "v7" => Ok(7),
        "x8" | "s0" | "fp" | "f8" | "fs0" | "v8" => Ok(8),
        "x9" | "s1" | "f9" | "fs1" | "v9" => Ok(9),
        "x10" | "a0" | "f10" | "fa0" | "v10" => Ok(10),
        "x11" | "a1" | "f11" | "fa1" | "v11" => Ok(11),
        "x12" | "a2" | "f12" | "fa2" | "v12" => Ok(12),
        "x13" | "a3" | "f13" | "fa3" | "v13" => Ok(13),
        "x14" | "a4" | "f14" | "fa4" | "v14" => Ok(14),
        "x15" | "a5" | "f15" | "fa5" | "v15" => Ok(15),
        "x16" | "a6" | "f16" | "fa6" | "v16" => Ok(16),
        "x17" | "a7" | "f17" | "fa7" | "v17" => Ok(17),
        "x18" | "s2" | "f18" | "fs2" | "v18" => Ok(18),
        "x19" | "s3" | "f19" | "fs3" | "v19" => Ok(19),
        "x20" | "s4" | "f20" | "fs4" | "v20" => Ok(20),
        "x21" | "s5" | "f21" | "fs5" | "v21" => Ok(21),
        "x22" | "s6" | "f22" | "fs6" | "v22" => Ok(22),
        "x23" | "s7" | "f23" | "fs7" | "v23" => Ok(23),
        "x24" | "s8" | "f24" | "fs8" | "v24" => Ok(24),
        "x25" | "s9" | "f25" | "fs9" | "v25" => Ok(25),
        "x26" | "s10" | "f26" | "fs10" | "v26" => Ok(26),
        "x27" | "s11" | "f27" | "fs11" | "v27" => Ok(27),
        "x28" | "t3" | "f28" | "ft8" | "v28" => Ok(28),
        "x29" | "t4" | "f29" | "ft9" | "v29" => Ok(29),
        "x30" | "t5" | "f30" | "ft10" | "v30" => Ok(30),
        "x31" | "t6" | "f31" | "ft11" | "v31" => Ok(31),
        maybe_immediate => maybe_immediate.parse::<usize>().map_err(|_| "Invalid operand")
    }
}