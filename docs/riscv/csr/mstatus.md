The `mstatus` register is an MXLEN-bit read/write register. The `mstatus` register keeps track of
and controls the hart's current operating state. A restricted view of `mstatus` appears as the
`sstatus` register in the S-level ISA

# RV64
| Bits  | Field    |
| ----  | -------- |
| 63    | SD       |
| 62:43 | *WPRI*   |
| 42    | MDT      |
| 41    | MPELP    |
| 40    | WPRI     |
| 39    | MPV      |
| 38    | GVA      |
| 37    | MBE      |
| 36    | SBE      |
| 35:34 | SXL[1:0] |
| 33:32 | UXL[1:0] |
| 31:25 | *WPRI*   |
| 24    | SDT      |
| 23    | SPELP    |
| 22    | TSR      |
| 21    | TW       |
| 20    | TVM      |
| 19    | MXR      |
| 18    | SUM      |
| 17    | MPRV     |
| 16:15 | XS[1:0]  |
| 14:13 | FS[1:0]  |
| 12:11 | MPP[1:0] |
| 10:9  | VS[1:0]  |
| 8     | SPP      |
| 7     | MPIE     |
| 6     | UBE      |
| 5     | SPIE     |
| 4     | *WPRI*   |
| 3     | MIE      |
| 2     | *WPRI*   |
| 1     | SIE      |
| 0     | *WPRI*   |
# Fields
## MDT
M-mode-disable trap bit is a WARL field introduced by the Smdbltrp extension. Upon reset the MDT bit
is set to 1. When the `MDT` bit is set to 1 by an explicit CSR write, the `MIE` bit is cleared to 0.

When a trap is taken into M-mode and the `MDT` bit is currently 0 it is set to 1 and the trap is
delivered as expected. However, if `MDT` is already set to 1, the hart enters a critical error state
without updating any architectural state. This state involves ceasing execution, disabling all
interrupts and asserting a critical-error signal to the platform.

The `MRET` and `SRET` instructions, when executed in M-mode set the `MDT` bit to 0.
## MIE
M-mode global interrupt-enable bit. When a hart is executing in M mode, interrupts are globally
enabled when MIE = 1 and globally disabled when MIE = 0. The `MIE` bit can only be set to 1 if the
`MDT` bit is 0.
## MPIE
Value of MIE prior to the trap
## MPP
Privilege mode prior to the trap
## SIE
S-mode global interrupt-enable bit. If supervision mode is nit implemented, SIE is read-only 0. When
a hart is executing in S mode, interrupts are globally enabled when SIE = 1 and globally disabled
when SIE = 0.
## SPIE
Value of SIE prior to the trap
## SPP
Privilege mode prior to the trap