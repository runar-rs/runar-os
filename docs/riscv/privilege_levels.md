At any time, a RISC-V hardware thread (hart) is running at some privilege level encoded as a mode in
one or more CSRs. Three RISC-V privilege levels are currently defined.

| Level | Encoding | Name             | Abbreviation |
| ----- | -------- | ---------------- | ------------ |
| 0     | 00       | User/Application | U            |
| 1     | 01       | Supervisor       | S            |
| 2     | 10       | *Reserved*       |              |
| 3     | 11       | Machine          | M            |