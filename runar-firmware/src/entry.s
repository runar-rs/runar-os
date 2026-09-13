.section .text.entry
.global _start

_start:
    #Load MHartID into a0 for rust_main to use
    #Safe, because the harts are in M-mode and mhartid must be readable in any implementation.
    csrrs a0, mhartid, x0
    la sp, _boot_stack_end
    
    la t0, _bss_start
    la t1, _bss_end

# Zero BSS segment
1:
    bgeu t0, t1, 2f
    sd zero, 0(t0)
    addi t0, t0, 8
    j 1b

2:
    #Enter the rust main function
    call rust_main

switch_to_supervisor:
    # a0 contains the target address
    csrrw x0, mepc, a0
    # set mpp to 01
    csrrc t0, mstatus, 11
    csrrs t0, mstatus, 10
