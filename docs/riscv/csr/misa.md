The **misa** CSR is a **WARL** read-write register reporting the ISA supported by the hart. This
register must be readable in any implementation, but a value of zero can be returned to indicate the
**misa** register has not been implemented, requiring that CPU capabilities be determined through a
separate non-standard mechanism. [#1](https://github.com/runar-rs/runar-os/issues/1)

# MXL
The MXL (Machine XLEN) field [MXLEN-1:MXLEN-2] encodes the native base ISA width. The MXL field is read-only. If
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

| **Bit** | **Character** | **Description**                                            |
| ------- | ------------- | ---------------------------------------------------------- |
| 0       | A             | Atomic extension                                           |
| 1       | B             | B extension                                                |
| 2       | C             | Compressed extension                                       |
| 3       | D             | Double-precision floating-point extension                  |
| 4       | E             | RV32E/64E base ISA                                         |
| 5       | F             | Single-precision floating-point extension                  |
| 6       | G             | *Reserved*                                                 |
| 7       | H             | Hypervisor extension                                       |
| 8       | I             | RV32I/64I base ISA                                         |
| 9       | J             | *Reserved*                                                 |
| 10      | K             | *Reserved*                                                 |
| 11      | L             | *Reserved*                                                 |
| 12      | M             | Integer Multiply/Divide extension                          |
| 13      | N             | *Tentatively reserved for User-Level Interrupts extension* |
| 14      | O             | *Reserved*                                                 |
| 15      | P             | *Tentatively reserved for Packed-SIMD extension*           |
| 16      | Q             | Quad-precision floating-point extension                    |
| 17      | R             | *Reserved*                                                 |
| 18      | S             | Supervisor mode implemented                                |
| 19      | T             | *Reserved*                                                 |
| 20      | U             | User mode implemented                                      |
| 21      | V             | Vector extension                                           |
| 22      | W             | *Reserved*                                                 |
| 23      | X             | Non-standard extensions present                            |
| 24      | Y             | *Reserved*                                                 |
| 25      | Z             | *Reserved*                                                 |