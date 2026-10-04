use crate::arch::HART_IDS;

pub mod context;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HartId {
    id: usize,
}

#[inline(always)]
pub fn get_hart_id() -> HartId {
    let hartid: usize;
    unsafe {
        core::arch::asm!("csrrs {}, mhartid, x0", out(reg) hartid);
    }
    HartId {
        id: hartid
    }
}

/// Index of a hart.
/// 
/// The harts are sorted by id, with the HartNum representing the index in that sorted list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HartNum {
    index: usize
}

impl From<HartNum> for usize {
    #[inline(always)]
    fn from(value: HartNum) -> Self {
        value.index
    }
}

/// Returns the number of the hart
#[inline(always)]
pub fn get_mapped_hart_num() -> HartNum {
    let current_hart_id = get_hart_id().id;
    for (i, hart_id) in HART_IDS.iter().enumerate() {
        if *hart_id == current_hart_id {
            return HartNum {
                index: i
            };
        }
    }
    // The current hart id is not in the HART_IDS constant.
    panic!();
}

/// Base Integer Instruction Set
#[derive(Debug, Clone, Copy)]
pub enum Base {
    RV32I,
    RV32E,
    RV64I,
    RV64E,
}

/// Profiles
#[derive(Debug, Clone, Copy)]
pub enum Profile {
    RVI20U32,
    RVI20U64,
    RVA20U64,
    RVA20S64,
    RVA22U64,
    RVA22S64,
    RVA23U64,
    RVA23S64,
    RVB23U64,
    RVB23S64
}

/// Ratified Extensions
#[derive(Debug, Clone, Copy)]
pub enum Extension {
    /// Atomic instructions extension.
    A,
    /// BFloat16-precision floating-point extensions.
    BF16,

    /// Base cache management operations extension.
    CMO,
    /// Double-precision floating-point extension.
    D,
    /// Single-precision floating-point extension.
    F,
    /// Integer multiplication and division extension.
    M,
    /// Quad-precision floating-point extension.
    Q,
    /// Atomic memory operations extension.
    Zaamo,
    /// Byte and halfword atomic memory operations extension.
    Zabha,
    /// Atomic compare-and-swap instructions extension.
    Zacas,
    /// Atomic load-acquire and store-release instructions extension.
    Zalasr,
    /// Load-reserve/store-conditional instructions extension.
    Zalrsc,
    /// Wait-on-reservation-set instructions extension.
    Zawrs,
    /// Compressed may-be-operations extension.
    Zcmop,
    /// Additional floating-point instructions extension.
    /// 
    /// The Ufa extension depends on the [`Extension::F`] extension.
    Zfa,
    /// Minimal scalar BF16 convert extension.
    /// 
    /// This extension provides the minimal se of instruction needed to enable scalar support of
    /// the BF16 format. It enables BF16 as an interchange format as it provides conversion between
    /// BF16 values and FP32 values.
    Zfbfmin,
    /// Half-precision floating-point extension
    Zfh,
    /// Minimal half-precision floating-point extension
    Zfhmin,
    /// Cache-block management instructions extension.
    Zicbom,
    /// Cache-block prefetch instructions extension.
    Zicbop,
    /// Cache-block zero instructions extension.
    Zicboz,
    /// Base counters and timers extension.
    Zicntr,
    /// Integer conditional operations extension.
    Zicond,
    /// Control and status register instructions extension.
    Zicsr,
    /// Instruction-fetch fence extension.
    /// 
    /// This extension provides the `FENCE.I` instruction that provides explicit synchronization
    /// between writes to instruction memory and instruction fetches on the same hart. 
    Zifencei,
    /// Non-temporal locality hints extension.
    Zihintntl,
    /// Pause hint extension.
    Zihinzpause,
    /// Hardware performance counters extension.
    Zihpm,
    /// May-be-operations extension.
    Zimop,
    /// Multiplication extension.
    Zmmul,
    /// Total store ordering extension.
    Ztso,
    /// Minimal vector BF16 convert extension.
    /// 
    /// This extension provides the minimal set of instructions need to enable vector support of
    /// the BF16 format. It enables BF16 as an interchange format as it provides conversion between
    /// BF16 values and FP32 values.
    Zvfbfmin,
    /// Vector BF16 widening mul-add
    /// 
    /// This extension provides a vector widening BF16 mul-add instructions that accumulates into
    /// FP32.
    Zvfbfwma,
}