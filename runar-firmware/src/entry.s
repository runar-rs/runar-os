.section .text.entry
.global _start

_start:
    #Load MHartID into a0 for rust_main to use
    csrrs a0, mhartid, x0
    la sp, _boot_stack_end
    #Enter the rust main function
    call rust_main