The **misa** CSR is a **WARL** read-write register reporting the ISA supported by the hart. This
register must be readable in any implementation, but a value of zero can be returned to indicate the
**misa** register has not been implemented, requiring that CPU capabilities be determined through a
separate non-standard mechanism. [#1](https://github.com/runar-rs/runar-os/issues/1)

# MXL
The MXL (Machine XLEN) field encodes the native base ISA width. The MXL field is read-only. If
**misa** is non-zero, the MXL field indicates the effective XLEN in M-mode, a constant termed MXLEN.
XLEN is never greater than MXLEN but XLEN might be smaller than MXLEN in less-privileged modes.

| **MXL** | **XLEN**   |
| ------- | ---------- |
| 1       | 32         |
| 2       | 64         |
| 3       | *Reserved* |

# Extensions

When a standard extension is disabled by clearing its bit in **misa**, the instruction and CSRs
defined or modified by the extension revert to their defined or reserved behaviors as if the
extension is not implemented.