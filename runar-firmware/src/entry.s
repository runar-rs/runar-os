.section .text.entry
.global _start

_start:
    #Load MHartID into a0 for rust_main to use
    csrrs a0, mhartid, x0
    la sp, _boot_stack_end
    
    la t0, _bss_start
    la t1, _bss_end

1:
    bgeu t0, t1, 2f
    sd zero, 0(t0)
    addi t0, t0, 8
    j 1b

2:
    #Enter the rust main function
    call rust_main