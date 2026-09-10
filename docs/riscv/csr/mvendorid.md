The `mvendorid` CSR is a 32-bit read-only register providing the JEDEC manufacturer ID of the
provider of the core. This register must be readable in any implementation, but a value of 0 can be
returned to indicate that the field is not implemented or that this is a non-commercial
implementation.

| Bits | Field  |
| ---- | ------ |
| 31:7 | Bank   |
| 6:0  | Offset |

JEDEC manufacturer IDs are ordinarily encoded as a sequence of one-byte continuation codes `0x7f`,
terminated by a one-byte ID not equal to `0x7f`, with an odd parity bit in the most-significant bit
of each byte. `mvendorid` encodes the number of one-byte continuation codes in the Bank field, and
encodes the final byte in the Offset field, discarding the parity bit.