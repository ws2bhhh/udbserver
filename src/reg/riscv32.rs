use crate::reg::RegMap;
use unicorn_engine::RegisterRISCV;

pub static REGMAP: RegMap = RegMap {
    regs: &[
        (Some(RegisterRISCV::ZERO as i32), 4),
        (Some(RegisterRISCV::RA as i32), 4),
        (Some(RegisterRISCV::SP as i32), 4),
        (Some(RegisterRISCV::GP as i32), 4),
        (Some(RegisterRISCV::TP as i32), 4),
        (Some(RegisterRISCV::T0 as i32), 4),
        (Some(RegisterRISCV::T1 as i32), 4),
        (Some(RegisterRISCV::T2 as i32), 4),
        (Some(RegisterRISCV::S0 as i32), 4),
        (Some(RegisterRISCV::S1 as i32), 4),
        (Some(RegisterRISCV::A0 as i32), 4),
        (Some(RegisterRISCV::A1 as i32), 4),
        (Some(RegisterRISCV::A2 as i32), 4),
        (Some(RegisterRISCV::A3 as i32), 4),
        (Some(RegisterRISCV::A4 as i32), 4),
        (Some(RegisterRISCV::A5 as i32), 4),
        (Some(RegisterRISCV::A6 as i32), 4),
        (Some(RegisterRISCV::A7 as i32), 4),
        (Some(RegisterRISCV::S2 as i32), 4),
        (Some(RegisterRISCV::S3 as i32), 4),
        (Some(RegisterRISCV::S4 as i32), 4),
        (Some(RegisterRISCV::S5 as i32), 4),
        (Some(RegisterRISCV::S6 as i32), 4),
        (Some(RegisterRISCV::S7 as i32), 4),
        (Some(RegisterRISCV::S8 as i32), 4),
        (Some(RegisterRISCV::S9 as i32), 4),
        (Some(RegisterRISCV::S10 as i32), 4),
        (Some(RegisterRISCV::S11 as i32), 4),
        (Some(RegisterRISCV::T3 as i32), 4),
        (Some(RegisterRISCV::T4 as i32), 4),
        (Some(RegisterRISCV::T5 as i32), 4),
        (Some(RegisterRISCV::T6 as i32), 4),
        (Some(RegisterRISCV::PC as i32), 4),
    ],
    len: 33,
    desc: r#"<target version="1.0"><architecture>riscv:rv32</architecture></target>"#,
};
