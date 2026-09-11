The `mhartid` CSR is an `MXLEN`-bit read-only register containing the integer ID of the hardware
thread running the code. This register must be readable in any implementation. Hart IDs might not
necessarily be numbered contiguously in a multiprocessor system, but one hart must have a hart ID of
0. Hart IDs must be unique within the execution environment

| Bits      | Field   |
| --------- | ------- |
| MXLEN-1:0 | Hart ID |