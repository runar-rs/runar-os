The `marchid` CSR is an `MXLEN`-bit read only register encoding the base microarchitecture of the
hart. This register must be readable in any implementation, but a value of 0 can be returned to
indicate that the field is not implemented. The combination of `mvendorid` and `marchid` should
uniquely identify the type of hart microarchitecture that is implemented.

| Bits        | Field           |
| ----------- | --------------- |
| MXLEN - 1:0 | Architecture ID |

Open-source project architecture IDs have non-zero architecture IDs with a zero most-significant
bit. Commercial architecture IDs must have the most-significant bit set and cannot contain zero in
the remaining MXLEN-1 bits.