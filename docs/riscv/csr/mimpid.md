The `mimpid` CSR provides a unique encoding of the version of the processor implementation. This
register must be readable in any implementation, but a value of 0 can be returned to indicate that
the field is not implemented. The Implementation value should reflect the design of the RISC-V
processor itself and not any surrounding system.

| Bits        | Field          |
| ----------- | -------------- |
| MXLEN - 1:0 | Implementation |