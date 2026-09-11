`sret` is used to return from a trap in S-mode.

Pseudo-Code
```rust
mstatus.sie = mstatus.spie;
privilege_mode = mstatus.spp;
if mstatus.spp != 0 {
    mstatus.sptv = 0;
}
mstatus.spie = 1;
mstatus.spp = 00;
```