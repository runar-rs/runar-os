`mret` is used to return from a trap in M-mode.

Pseudo-Code
```rust
mstatus.mie = mstatus.mpie;
privilege_mode = mstatus.mpp;
if mstatus.mpp != 0 {
    mstatus.mptv = 0;
}
mstatus.mpie = 1;
mstatus.mpp = 00;
mstatus.mdt = 0;
```