Overview over the currently allocated RISC-V CSR addresses. Note that RV32 only registers are not in
the list.
<table>
    <tr>
        <th>Number</th>
        <th>Privilege</th>
        <th>Name</th>
        <th>Description</th>
    </tr>
    <tr>
        <td><b>0x001</b></td>
        <td>URW</td>
        <td><b>fflags</b></td>
        <td>Floating-Point Accrued Exceptions</td>
    </tr>
    <tr>
        <td><b>0x002</b></td>
        <td>URW</td>
        <td><b>frm</b></td>
        <td>Floating-Point Dynamic Rounding Mode</td>
    </tr>
    <tr>
        <td><b>0x003</b></td>
        <td>URW</td>
        <td><b>fcsr</b></td>
        <td>Floating-Point Control and Status Register</td>
    </tr>
    <tr>
        <td><b>0x008</b></td>
        <td>URW</td>
        <td><b>vstart</b></td>
        <td>Vector start position</td>
    </tr>
    <tr>
        <td><b>0x009</b></td>
        <td>URW</td>
        <td><b>vxsat</b></td>
        <td>Fixed-point accrued saturation flag</td>
    </tr>
    <tr>
        <td><b>0x00A</b></td>
        <td>URW</td>
        <td><b>vxrm</b></td>
        <td>Fixed-point rounding mode</td>
    </tr>
    <tr>
        <td><b>0x00F</b></td>
        <td>URW</td>
        <td><b>vcsr</b></td>
        <td>Vector control and status register</td>
    </tr>
    <tr>
        <td><b>0x011</b></td>
        <td>URW</td>
        <td><b>ssp</b></td>
        <td>Shadow Stack Pointer</td>
    </tr>
    <tr>
        <td><b>0x015</b></td>
        <td>URW</td>
        <td><b>seed</b></td>
        <td>Seed for cryptographic random bit generators</td>
    </tr>
    <tr>
        <td><b>0x017</b></td>
        <td>URW</td>
        <td><b>jvt</b></td>
        <td>Table jump base vector and control register</td>
    </tr>
    <tr>
        <td><b>0x100</b></td>
        <td>SRW</td>
        <td><b>sstatus</b></td>
        <td>Supervisor status register</td>
    </tr>
    <tr>
        <td><b>0x104</b></td>
        <td>SRW</td>
        <td><b>sie</b></td>
        <td>Supervisor interrupt-enable register</td>
    </tr>
    <tr>
        <td><b>0x105</b></td>
        <td>SRW</td>
        <td><b>stvec</b></td>
        <td>Supervisor trap handler base address</td>
    </tr>
    <tr>
        <td><b>0x106</b></td>
        <td>SRW</td>
        <td><b>scounteren</b></td>
        <td>Supervisor counter enable</td>
    </tr>
    <tr>
        <td><b>0x10A</b></td>
        <td>SRW</td>
        <td><b>senvcfg</b></td>
        <td>Supervisor environment configuration register</td>
    </tr>
    <tr>
        <td><b>0x10C</b></td>
        <td>SRW</td>
        <td><b>sstateen0</b></td>
        <td>Supervisor State Enable 0 Register</td>
    </tr>
    <tr>
        <td><b>0x10D</b></td>
        <td>SRW</td>
        <td><b>sstateen1</b></td>
        <td>Supervisor State Enable 1 Register</td>
    </tr>
    <tr>
        <td><b>0x10E</b></td>
        <td>SRW</td>
        <td><b>sstateen2</b></td>
        <td>Supervisor State Enable 2 Register</td>
    </tr>
    <tr>
        <td><b>0x10F</b></td>
        <td>SRW</td>
        <td><b>sstateen3</b></td>
        <td>Supervisor State Enable 3 Register</td>
    </tr>
    <tr>
        <td><b>0x120</b></td>
        <td>SRW</td>
        <td><b>scountinhibit</b></td>
        <td>Supervisor counter-inhibit register</td>
    </tr>
    <tr>
        <td><b>0x140</b></td>
        <td>SRW</td>
        <td><b>sscratch</b></td>
        <td>Supervisor scratch register</td>
    </tr>
    <tr>
        <td><b>0x141</b></td>
        <td>SRW</td>
        <td><b>sepc</b></td>
        <td>Supervisor exception program counter</td>
    </tr>
    <tr>
        <td><b>0x142</b></td>
        <td>SRW</td>
        <td><b>scause</b></td>
        <td>Supervisor trap cause</td>
    </tr>
    <tr>
        <td><b>0x143</b></td>
        <td>SRW</td>
        <td><b>stval</b></td>
        <td>Supervisor trap value</td>
    </tr>
    <tr>
        <td><b>0x144</b></td>
        <td>SRW</td>
        <td><b>sip</b></td>
        <td>Supervisor interrupt pending</td>
    </tr>
    <tr>
        <td><b>0x14D</b></td>
        <td>SRW</td>
        <td><b>stimecmp</b></td>
        <td>Supervisor timer compare</td>
    </tr>
    <tr>
        <td><b>0x14E</b></td>
        <td>SRW</td>
        <td><b>sctrctl</b></td>
        <td>Supervisor Control Transfer Records Control Register</td>
    </tr>
    <tr>
        <td><b>0x14F</b></td>
        <td>SRW</td>
        <td><b>sctrstatus</b></td>
        <td>Supervisor Control Transfer Records Status Register</td>
    </tr>
    <tr>
        <td><b>0x150</b></td>
        <td>SRW</td>
        <td><b>siselect</b></td>
        <td>Supervisor indirect register select</td>
    </tr>
    <tr>
        <td><b>0x151</b></td>
        <td>SRW</td>
        <td><b>sireg</b></td>
        <td>Supervisor indirect register alias</td>
    </tr>
    <tr>
        <td><b>0x152</b></td>
        <td>SRW</td>
        <td><b>sireg2</b></td>
        <td>Supervisor indirect register alias 2</td>
    </tr>
    <tr>
        <td><b>0x153</b></td>
        <td>SRW</td>
        <td><b>sireg3</b></td>
        <td>Supervisor indirect register alias 3</td>
    </tr>
    <tr>
        <td><b>0x155</b></td>
        <td>SRW</td>
        <td><b>sireg4</b></td>
        <td>Supervisor indirect register alias 4</td>
    </tr>
    <tr>
        <td><b>0x156</b></td>
        <td>SRW</td>
        <td><b>sireg5</b></td>
        <td>Supervisor indirect register alias 5</td>
    </tr>
    <tr>
        <td><b>0x157</b></td>
        <td>SRW</td>
        <td><b>sireg6</b></td>
        <td>Supervisor indirect register alias 6</td>
    </tr>
    <tr>
        <td><b>0x15F</b></td>
        <td>SRW</td>
        <td><b>sctrdepth</b></td>
        <td>Supervisor Control Transfer Records Depth Register</td>
    </tr>
    <tr>
        <td><b>0x180</b></td>
        <td>SRW</td>
        <td><b>satp</b></td>
        <td>Supervisor address translation and protection</td>
    </tr>
    <tr>
        <td><b>0x181</b></td>
        <td>SRW</td>
        <td><b>srmcfg</b></td>
        <td>Supervisor Resource Management Configuration</td>
    </tr>
    <tr>
        <td><b>0x200</b></td>
        <td>HRW</td>
        <td><b>vsstatus</b></td>
        <td>Virtual supervisor status register</td>
    </tr>
    <tr>
        <td><b>0x204</b></td>
        <td>HRW</td>
        <td><b>vsie</b></td>
        <td>Virtual supervisor interrupt-enable register</td>
    </tr>
    <tr>
        <td><b>0x205</b></td>
        <td>HRW</td>
        <td><b>vstvec</b></td>
        <td>Virtual supervisor trap handler base address</td>
    </tr>
    <tr>
        <td><b>0x240</b></td>
        <td>HRW</td>
        <td><b>vsscratch</b></td>
        <td>Virtual supervisor scratch register</td>
    </tr>
    <tr>
        <td><b>0x241</b></td>
        <td>HRW</td>
        <td><b>vsepc</b></td>
        <td>Virtual supervisor exception program counter</td>
    </tr>
    <tr>
        <td><b>0x242</b></td>
        <td>HRW</td>
        <td><b>vscause</b></td>
        <td>Virtual supervisor trap cause</td>
    </tr>
    <tr>
        <td><b>0x243</b></td>
        <td>HRW</td>
        <td><b>vstval</b></td>
        <td>Virtual supervisor trap value</td>
    </tr>
    <tr>
        <td><b>0x244</b></td>
        <td>HRW</td>
        <td><b>vsip</b></td>
        <td>Virtual supervisor interrupt pending</td>
    </tr>
    <tr>
        <td><b>0x24D</b></td>
        <td>HRW</td>
        <td><b>vstimecmp</b></td>
        <td>Virtual supervisor timer compare</td>
    </tr>
    <tr>
        <td><b>0x24E</b></td>
        <td>HRW</td>
        <td><b>vsctrctl</b></td>
        <td>Virtual supervisor control transfer records control register</td>
    </tr>
    <tr>
        <td><b>0x250</b></td>
        <td>HRW</td>
        <td><b>vsiselect</b></td>
        <td>Virtual supervisor indirect register select</td>
    </tr>
    <tr>
        <td><b>0x251</b></td>
        <td>HRW</td>
        <td><b>vsireg</b></td>
        <td>Virtual supervisor indirect register alias</td>
    </tr>
    <tr>
        <td><b>0x252</b></td>
        <td>HRW</td>
        <td><b>vsireg2</b></td>
        <td>Virtual supervisor indirect register alias 2</td>
    </tr>
    <tr>
        <td><b>0x253</b></td>
        <td>HRW</td>
        <td><b>vsireg3</b></td>
        <td>Virtual supervisor indirect register alias 3</td>
    </tr>
    <tr>
        <td><b>0x255</b></td>
        <td>HRW</td>
        <td><b>vsireg4</b></td>
        <td>Virtual supervisor indirect register alias 4</td>
    </tr>
    <tr>
        <td><b>0x256</b></td>
        <td>HRW</td>
        <td><b>vsireg5</b></td>
        <td>Virtual supervisor indirect register alias 5</td>
    </tr>
    <tr>
        <td><b>0x257</b></td>
        <td>HRW</td>
        <td><b>vsireg6</b></td>
        <td>Virtual supervisor indirect register alias 6</td>
    </tr>
    <tr>
        <td><b>0x280</b></td>
        <td>HRW</td>
        <td><b>vsatp</b></td>
        <td>Virtual supervisor address translation and protection</td>
    </tr>
    <tr>
        <td><b>0x300</b></td>
        <td>MRW</td>
        <td><b>mstatus</b></td>
        <td>Machine status register</td>
    </tr>
    <tr>
        <td><b>0x301</b></td>
        <td>MRW</td>
        <td><b>misa</b></td>
        <td>ISA and extensions</td>
    </tr>
    <tr>
        <td><b>0x302</b></td>
        <td>MRW</td>
        <td><b>medeleg</b></td>
        <td>Machine exception delegation register</td>
    </tr>
    <tr>
        <td><b>0x303</b></td>
        <td>MRW</td>
        <td><b>mideleg</b></td>
        <td>Machine interrupt delegation register</td>
    </tr>
    <tr>
        <td><b>0x304</b></td>
        <td>MRW</td>
        <td><b>mie</b></td>
        <td>Machine interrupt</td>
    </tr>
    <tr>
        <td><b>0x305</b></td>
        <td>MRW</td>
        <td><b>mtvec</b></td>
        <td>Machine trap-handler base address</td>
    </tr>
    <tr>
        <td><b>0x306</b></td>
        <td>MRW</td>
        <td><b>mcounteren</b></td>
        <td>Machine counter enable</td>
    </tr>
    <tr>
        <td><b>0x30A</b></td>
        <td>MRW</td>
        <td><b>menvcfg</b></td>
        <td>Machine environment configuration register</td>
    </tr>
    <tr>
        <td><b>0x30C</b></td>
        <td>MRW</td>
        <td><b>mstateem0</b></td>
        <td>Machine state enable 0 register</td>
    </tr>
    <tr>
        <td><b>0x30D</b></td>
        <td>MRW</td>
        <td><b>mstateem1</b></td>
        <td>Machine state enable 1 register</td>
    </tr>
    <tr>
        <td><b>0x30E</b></td>
        <td>MRW</td>
        <td><b>mstateem2</b></td>
        <td>Machine state enable 2 register</td>
    </tr>
    <tr>
        <td><b>0x30F</b></td>
        <td>MRW</td>
        <td><b>mstateem3</b></td>
        <td>Machine state enable 3 register</td>
    </tr>
    <tr>
        <td><b>0x320</b></td>
        <td>MRW</td>
        <td><b>mcountinhibit</b></td>
        <td>Machine counter-inhibit register</td>
    </tr>
    <tr>
        <td><b>0x321</b></td>
        <td>MRW</td>
        <td><b>mcyclecfg</b></td>
        <td>Machine cycle counter configuration register</td>
    </tr>
    <tr>
        <td><b>0x322</b></td>
        <td>MRW</td>
        <td><b>minstretcfg</b></td>
        <td>Machine instret counter configuration register</td>
    </tr>
    <tr>
        <td><b>0x323</b></td>
        <td>MRW</td>
        <td><b>mhpevent3</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x324</b></td>
        <td>MRW</td>
        <td><b>mhpevent4</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x325</b></td>
        <td>MRW</td>
        <td><b>mhpevent5</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x326</b></td>
        <td>MRW</td>
        <td><b>mhpevent6</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x327</b></td>
        <td>MRW</td>
        <td><b>mhpevent7</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x328</b></td>
        <td>MRW</td>
        <td><b>mhpevent8</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x329</b></td>
        <td>MRW</td>
        <td><b>mhpevent9</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x32A</b></td>
        <td>MRW</td>
        <td><b>mhpevent10</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x32B</b></td>
        <td>MRW</td>
        <td><b>mhpevent11</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x32C</b></td>
        <td>MRW</td>
        <td><b>mhpevent12</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x32D</b></td>
        <td>MRW</td>
        <td><b>mhpevent13</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x32E</b></td>
        <td>MRW</td>
        <td><b>mhpevent14</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x32F</b></td>
        <td>MRW</td>
        <td><b>mhpevent15</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x330</b></td>
        <td>MRW</td>
        <td><b>mhpevent16</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x331</b></td>
        <td>MRW</td>
        <td><b>mhpevent17</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x332</b></td>
        <td>MRW</td>
        <td><b>mhpevent18</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x333</b></td>
        <td>MRW</td>
        <td><b>mhpevent19</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x334</b></td>
        <td>MRW</td>
        <td><b>mhpevent20</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x335</b></td>
        <td>MRW</td>
        <td><b>mhpevent21</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x336</b></td>
        <td>MRW</td>
        <td><b>mhpevent22</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x337</b></td>
        <td>MRW</td>
        <td><b>mhpevent23</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x338</b></td>
        <td>MRW</td>
        <td><b>mhpevent24</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x339</b></td>
        <td>MRW</td>
        <td><b>mhpevent25</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x33A</b></td>
        <td>MRW</td>
        <td><b>mhpevent26</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x33B</b></td>
        <td>MRW</td>
        <td><b>mhpevent27</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x33C</b></td>
        <td>MRW</td>
        <td><b>mhpevent28</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x33D</b></td>
        <td>MRW</td>
        <td><b>mhpevent29</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x33E</b></td>
        <td>MRW</td>
        <td><b>mhpevent30</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x33F</b></td>
        <td>MRW</td>
        <td><b>mhpevent31</b></td>
        <td>Machine performance-monitoring event selector</td>
    </tr>
    <tr>
        <td><b>0x340</b></td>
        <td>MRW</td>
        <td><b>mscratch</b></td>
        <td>Machine scratch register</td>
    </tr>
    <tr>
        <td><b>0x341</b></td>
        <td>MRW</td>
        <td><b>mepc</b></td>
        <td>Machine exception program counter</td>
    </tr>
    <tr>
        <td><b>0x342</b></td>
        <td>MRW</td>
        <td><b>mcause</b></td>
        <td>Machine trap cause</td>
    </tr>
    <tr>
        <td><b>0x343</b></td>
        <td>MRW</td>
        <td><b>mtval</b></td>
        <td>Machine trap value</td>
    </tr>
    <tr>
        <td><b>0x344</b></td>
        <td>MRW</td>
        <td><b>mip</b></td>
        <td>Machine interrupt pending</td>
    </tr>
    <tr>
        <td><b>0x34A</b></td>
        <td>MRW</td>
        <td><b>mtinst</b></td>
        <td>Machine trap instruction (transformed)</td>
    </tr>
    <tr>
        <td><b>0x34B</b></td>
        <td>MRW</td>
        <td><b>mtval2</b></td>
        <td>Machine second trap value.</td>
    </tr>
    <tr>
        <td><b>0x34E</b></td>
        <td>MRW</td>
        <td><b>mctrctl</b></td>
        <td>Machine control transfer records control register</td>
    </tr>
    <tr>
        <td><b>0x350</b></td>
        <td>MRW</td>
        <td><b>miselect</b></td>
        <td>Machine indirect register select</td>
    </tr>
    <tr>
        <td><b>0x351</b></td>
        <td>MRW</td>
        <td><b>mireg</b></td>
        <td>Machine indirect register alias</td>
    </tr>
    <tr>
        <td><b>0x352</b></td>
        <td>MRW</td>
        <td><b>mireg2</b></td>
        <td>Machine indirect register alias 2</td>
    </tr>
    <tr>
        <td><b>0x353</b></td>
        <td>MRW</td>
        <td><b>mireg3</b></td>
        <td>Machine indirect register alias 3</td>
    </tr>
    <tr>
        <td><b>0x355</b></td>
        <td>MRW</td>
        <td><b>mireg4</b></td>
        <td>Machine indirect register alias 4</td>
    </tr>
    <tr>
        <td><b>0x356</b></td>
        <td>MRW</td>
        <td><b>mireg5</b></td>
        <td>Machine indirect register alias 5</td>
    </tr>
    <tr>
        <td><b>0x357</b></td>
        <td>MRW</td>
        <td><b>mireg6</b></td>
        <td>Machine indirect register alias 6</td>
    </tr>
    <tr>
        <td><b>0x3A0</b></td>
        <td>MRW</td>
        <td><b>pmpcfg0</b></td>
        <td>Physical memory protection configuration</td>
    </tr>
    <tr>
        <td><b>0x3A2</b></td>
        <td>MRW</td>
        <td><b>pmpcfg2</b></td>
        <td>Physical memory protection configuration</td>
    </tr>
    <tr>
        <td><b>0x3A4</b></td>
        <td>MRW</td>
        <td><b>pmpcfg4</b></td>
        <td>Physical memory protection configuration</td>
    </tr>
    <tr>
        <td><b>0x3A6</b></td>
        <td>MRW</td>
        <td><b>pmpcfg6</b></td>
        <td>Physical memory protection configuration</td>
    </tr>
    <tr>
        <td><b>0x3A8</b></td>
        <td>MRW</td>
        <td><b>pmpcfg8</b></td>
        <td>Physical memory protection configuration</td>
    </tr>
    <tr>
        <td><b>0x3AA</b></td>
        <td>MRW</td>
        <td><b>pmpcfg10</b></td>
        <td>Physical memory protection configuration</td>
    </tr>
    <tr>
        <td><b>0x3AC</b></td>
        <td>MRW</td>
        <td><b>pmpcfg12</b></td>
        <td>Physical memory protection configuration</td>
    </tr>
    <tr>
        <td><b>0x3AE</b></td>
        <td>MRW</td>
        <td><b>pmpcfg14</b></td>
        <td>Physical memory protection configuration</td>
    </tr>
    <tr>
        <td><b>0x3B0</b></td>
        <td>MRW</td>
        <td><b>pmpaddr0</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3B1</b></td>
        <td>MRW</td>
        <td><b>pmpaddr1</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3B2</b></td>
        <td>MRW</td>
        <td><b>pmpaddr2</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3B3</b></td>
        <td>MRW</td>
        <td><b>pmpaddr3</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3B4</b></td>
        <td>MRW</td>
        <td><b>pmpaddr4</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3B5</b></td>
        <td>MRW</td>
        <td><b>pmpaddr5</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3B6</b></td>
        <td>MRW</td>
        <td><b>pmpaddr6</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3B7</b></td>
        <td>MRW</td>
        <td><b>pmpaddr7</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3B8</b></td>
        <td>MRW</td>
        <td><b>pmpaddr8</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3B9</b></td>
        <td>MRW</td>
        <td><b>pmpaddr9</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3BA</b></td>
        <td>MRW</td>
        <td><b>pmpaddr10</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3BB</b></td>
        <td>MRW</td>
        <td><b>pmpaddr11</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3BC</b></td>
        <td>MRW</td>
        <td><b>pmpaddr12</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3BD</b></td>
        <td>MRW</td>
        <td><b>pmpaddr13</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3BE</b></td>
        <td>MRW</td>
        <td><b>pmpaddr14</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3BF</b></td>
        <td>MRW</td>
        <td><b>pmpaddr15</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3C0</b></td>
        <td>MRW</td>
        <td><b>pmpaddr16</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3C1</b></td>
        <td>MRW</td>
        <td><b>pmpaddr17</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3C2</b></td>
        <td>MRW</td>
        <td><b>pmpaddr18</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3C3</b></td>
        <td>MRW</td>
        <td><b>pmpaddr19</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3C4</b></td>
        <td>MRW</td>
        <td><b>pmpaddr20</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3C5</b></td>
        <td>MRW</td>
        <td><b>pmpaddr21</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3C6</b></td>
        <td>MRW</td>
        <td><b>pmpaddr22</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3C7</b></td>
        <td>MRW</td>
        <td><b>pmpaddr23</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3C8</b></td>
        <td>MRW</td>
        <td><b>pmpaddr24</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3C9</b></td>
        <td>MRW</td>
        <td><b>pmpaddr25</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3CA</b></td>
        <td>MRW</td>
        <td><b>pmpaddr26</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3CB</b></td>
        <td>MRW</td>
        <td><b>pmpaddr27</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3CC</b></td>
        <td>MRW</td>
        <td><b>pmpaddr28</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3CD</b></td>
        <td>MRW</td>
        <td><b>pmpaddr29</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3CE</b></td>
        <td>MRW</td>
        <td><b>pmpaddr30</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3CF</b></td>
        <td>MRW</td>
        <td><b>pmpaddr31</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3D0</b></td>
        <td>MRW</td>
        <td><b>pmpaddr32</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3D1</b></td>
        <td>MRW</td>
        <td><b>pmpaddr33</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3D2</b></td>
        <td>MRW</td>
        <td><b>pmpaddr34</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3D3</b></td>
        <td>MRW</td>
        <td><b>pmpaddr35</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3D4</b></td>
        <td>MRW</td>
        <td><b>pmpaddr36</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3D5</b></td>
        <td>MRW</td>
        <td><b>pmpaddr37</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3D6</b></td>
        <td>MRW</td>
        <td><b>pmpaddr38</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3D7</b></td>
        <td>MRW</td>
        <td><b>pmpaddr39</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3D8</b></td>
        <td>MRW</td>
        <td><b>pmpaddr40</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3D9</b></td>
        <td>MRW</td>
        <td><b>pmpaddr41</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3DA</b></td>
        <td>MRW</td>
        <td><b>pmpaddr42</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3DB</b></td>
        <td>MRW</td>
        <td><b>pmpaddr43</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3DC</b></td>
        <td>MRW</td>
        <td><b>pmpaddr44</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3DD</b></td>
        <td>MRW</td>
        <td><b>pmpaddr45</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3DE</b></td>
        <td>MRW</td>
        <td><b>pmpaddr46</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3DF</b></td>
        <td>MRW</td>
        <td><b>pmpaddr47</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3E0</b></td>
        <td>MRW</td>
        <td><b>pmpaddr48</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3E1</b></td>
        <td>MRW</td>
        <td><b>pmpaddr49</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3E2</b></td>
        <td>MRW</td>
        <td><b>pmpaddr50</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3E3</b></td>
        <td>MRW</td>
        <td><b>pmpaddr51</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3E4</b></td>
        <td>MRW</td>
        <td><b>pmpaddr52</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3E5</b></td>
        <td>MRW</td>
        <td><b>pmpaddr53</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3E6</b></td>
        <td>MRW</td>
        <td><b>pmpaddr54</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3E7</b></td>
        <td>MRW</td>
        <td><b>pmpaddr55</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3E8</b></td>
        <td>MRW</td>
        <td><b>pmpaddr56</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3E9</b></td>
        <td>MRW</td>
        <td><b>pmpaddr57</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3EA</b></td>
        <td>MRW</td>
        <td><b>pmpaddr58</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3EB</b></td>
        <td>MRW</td>
        <td><b>pmpaddr59</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3EC</b></td>
        <td>MRW</td>
        <td><b>pmpaddr60</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3BD</b></td>
        <td>MRW</td>
        <td><b>pmpaddr61</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3BE</b></td>
        <td>MRW</td>
        <td><b>pmpaddr62</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x3BF</b></td>
        <td>MRW</td>
        <td><b>pmpaddr63</b></td>
        <td>Physical memory protection address register</td>
    </tr>
    <tr>
        <td><b>0x5A8</b></td>
        <td>SRW</td>
        <td><b>scontext</b></td>
        <td>Supervisor-mode context register</td>
    </tr>
    <tr>
        <td><b>0x600</b></td>
        <td>HRW</td>
        <td><b>hstatus</b></td>
        <td>Hypervisor status register</td>
    </tr>
    <tr>
        <td><b>0x602</b></td>
        <td>HRW</td>
        <td><b>hedeleg</b></td>
        <td>Hypervisor exception delegation register</td>
    </tr>
    <tr>
        <td><b>0x603</b></td>
        <td>HRW</td>
        <td><b>hideleg</b></td>
        <td>Hypervisor interrupt delegation register</td>
    </tr>
    <tr>
        <td><b>0x604</b></td>
        <td>HRW</td>
        <td><b>hie</b></td>
        <td>Hypervisor interrupt enable register</td>
    </tr>
    <tr>
        <td><b>0x605</b></td>
        <td>HRW</td>
        <td><b>htimedelta</b></td>
        <td>Delta for VS/VU-mode timer</td>
    </tr>
    <tr>
        <td><b>0x606</b></td>
        <td>HRW</td>
        <td><b>hcounteren</b></td>
        <td>Hypervisor counter enable</td>
    </tr>
    <tr>
        <td><b>0x607</b></td>
        <td>HRW</td>
        <td><b>hgeie</b></td>
        <td>Hypervisor guest external interrupt-enable register</td>
    </tr>
    <tr>
        <td><b>0x60A</b></td>
        <td>HRW</td>
        <td><b>henvcfg</b></td>
        <td>Hypervisor environment configuration register</td>
    </tr>
    <tr>
        <td><b>0x60C</b></td>
        <td>HRW</td>
        <td><b>hstateen0</b></td>
        <td>Hypervisor state enable 0 register</td>
    </tr>
    <tr>
        <td><b>0x60D</b></td>
        <td>HRW</td>
        <td><b>hstateen1</b></td>
        <td>Hypervisor state enable 1 register</td>
    </tr>
    <tr>
        <td><b>0x60E</b></td>
        <td>HRW</td>
        <td><b>hstateen2</b></td>
        <td>Hypervisor state enable 2 register</td>
    </tr>
    <tr>
        <td><b>0x60F</b></td>
        <td>HRW</td>
        <td><b>hstateen3</b></td>
        <td>Hypervisor state enable 3 register</td>
    </tr>
    <tr>
        <td><b>0x643</b></td>
        <td>HRW</td>
        <td><b>htval</b></td>
        <td>Hypervisor trap value</td>
    </tr>
    <tr>
        <td><b>0x644</b></td>
        <td>HRW</td>
        <td><b>hip</b></td>
        <td>Hypervisor interrupt pending</td>
    </tr>
    <tr>
        <td><b>0x645</b></td>
        <td>HRW</td>
        <td><b>hvip</b></td>
        <td>Hypervisor virtual interrupt pending</td>
    </tr>
    <tr>
        <td><b>0x64A</b></td>
        <td>HRW</td>
        <td><b>htinst</b></td>
        <td>Hypervisor trap instruction (transformed)</td>
    </tr>
    <tr>
        <td><b>0x680</b></td>
        <td>HRW</td>
        <td><b>hgatp</b></td>
        <td>Hypervisor guest address translation and protection</td>
    </tr>
    <tr>
        <td><b>0x6A8</b></td>
        <td>HRW</td>
        <td><b>hcontext</b></td>
        <td>Hypervisor-mode context register</td>
    </tr>
    <tr>
        <td><b>0x740</b></td>
        <td>MRW</td>
        <td><b>mnscratch</b></td>
        <td>Resumable NMI scratch register</td>
    </tr>
    <tr>
        <td><b>0x741</b></td>
        <td>MRW</td>
        <td><b>mnepc</b></td>
        <td>Resumable NMI program counter</td>
    </tr>
    <tr>
        <td><b>0x742</b></td>
        <td>MRW</td>
        <td><b>mncause</b></td>
        <td>Resumable NMI cause</td>
    </tr>
    <tr>
        <td><b>0x744</b></td>
        <td>MRW</td>
        <td><b>mnstatus</b></td>
        <td>Resumable NMI status</td>
    </tr>
    <tr>
        <td><b>0x747</b></td>
        <td>MRW</td>
        <td><b>mseccfg</b></td>
        <td>Machine security configuration register</td>
    </tr>
    <tr>
        <td><b>0x7A0</b></td>
        <td>MRW</td>
        <td><b>tselect</b></td>
        <td>Debug/Trace trigger register select</td>
    </tr>
    <tr>
        <td><b>0x7A1</b></td>
        <td>MRW</td>
        <td><b>tdata1</b></td>
        <td>First Debug/Trace data register</td>
    </tr>
    <tr>
        <td><b>0x7A2</b></td>
        <td>MRW</td>
        <td><b>tdata2</b></td>
        <td>Second Debug/Trace data register</td>
    </tr>
    <tr>
        <td><b>0x7A3</b></td>
        <td>MRW</td>
        <td><b>tdata3</b></td>
        <td>Third Debug/Trace data register</td>
    </tr>
    <tr>
        <td><b>0x7A8</b></td>
        <td>MRW</td>
        <td><b>mcontext</b></td>
        <td>Machine-mode context register</td>
    </tr>
    <tr>
        <td><b>0x7B0</b></td>
        <td>DRW</td>
        <td><b>dcsr</b></td>
        <td>Debug control and status register</td>
    </tr>
    <tr>
        <td><b>0x7B1</b></td>
        <td>DRW</td>
        <td><b>dpc</b></td>
        <td>Debug program counter</td>
    </tr>
    <tr>
        <td><b>0x7B2</b></td>
        <td>DRW</td>
        <td><b>dscratch0</b></td>
        <td>Debug scratch register 0</td>
    </tr>
    <tr>
        <td><b>0x7B3</b></td>
        <td>DRW</td>
        <td><b>dscratch1</b></td>
        <td>Debug scratch register 1</td>
    </tr>
    <tr>
        <td><b>0xB00</b></td>
        <td>MRW</td>
        <td><b>mcycle</b></td>
        <td>Machine cycle counter</td>
    </tr>
    <tr>
        <td><b>0xB02</b></td>
        <td>MRW</td>
        <td><b>minstret</b></td>
        <td>Machine instructions-retired counter</td>
    </tr>
    <tr>
        <td><b>0xB03</b></td>
        <td>MRW</td>
        <td><b>mhomcounter3</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB04</b></td>
        <td>MRW</td>
        <td><b>mhomcounter4</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB05</b></td>
        <td>MRW</td>
        <td><b>mhomcounter5</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB06</b></td>
        <td>MRW</td>
        <td><b>mhomcounter6</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB07</b></td>
        <td>MRW</td>
        <td><b>mhomcounter7</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB08</b></td>
        <td>MRW</td>
        <td><b>mhomcounter8</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB09</b></td>
        <td>MRW</td>
        <td><b>mhomcounter9</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB0A</b></td>
        <td>MRW</td>
        <td><b>mhomcounter10</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB0B</b></td>
        <td>MRW</td>
        <td><b>mhomcounter11</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB0C</b></td>
        <td>MRW</td>
        <td><b>mhomcounter12</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB0D</b></td>
        <td>MRW</td>
        <td><b>mhomcounter13</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB0E</b></td>
        <td>MRW</td>
        <td><b>mhomcounter14</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB0F</b></td>
        <td>MRW</td>
        <td><b>mhomcounter15</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB10</b></td>
        <td>MRW</td>
        <td><b>mhomcounter16</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB11</b></td>
        <td>MRW</td>
        <td><b>mhomcounter17</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB12</b></td>
        <td>MRW</td>
        <td><b>mhomcounter18</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB13</b></td>
        <td>MRW</td>
        <td><b>mhomcounter19</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB14</b></td>
        <td>MRW</td>
        <td><b>mhomcounter20</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB15</b></td>
        <td>MRW</td>
        <td><b>mhomcounter21</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB16</b></td>
        <td>MRW</td>
        <td><b>mhomcounter22</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB17</b></td>
        <td>MRW</td>
        <td><b>mhomcounter23</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB18</b></td>
        <td>MRW</td>
        <td><b>mhomcounter24</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB19</b></td>
        <td>MRW</td>
        <td><b>mhomcounter25</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB1A</b></td>
        <td>MRW</td>
        <td><b>mhomcounter26</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB1B</b></td>
        <td>MRW</td>
        <td><b>mhomcounter27</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB1C</b></td>
        <td>MRW</td>
        <td><b>mhomcounter28</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB1D</b></td>
        <td>MRW</td>
        <td><b>mhomcounter29</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB1E</b></td>
        <td>MRW</td>
        <td><b>mhomcounter30</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xB1F</b></td>
        <td>MRW</td>
        <td><b>mhomcounter31</b></td>
        <td>Machine performance-counter</td>
    </tr>
    <tr>
        <td><b>0xC00</b></td>
        <td>URO</td>
        <td><b>cycle</b></td>
        <td>Cycle counter for RDCYCLE instruction</td>
    </tr>
    <tr>
        <td><b>0xC01</b></td>
        <td>URO</td>
        <td><b>time</b></td>
        <td>Timer for RDTIME instruction</td>
    </tr>
    <tr>
        <td><b>0xC02</b></td>
        <td>URO</td>
        <td><b>instret</b></td>
        <td>Instruction-retired counter for RDINSTRET instruction</td>
    </tr>
    <tr>
        <td><b>0xC03</b></td>
        <td>URO</td>
        <td><b>hpmcounter3</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC04</b></td>
        <td>URO</td>
        <td><b>hpmcounter4</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC05</b></td>
        <td>URO</td>
        <td><b>hpmcounter5</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC06</b></td>
        <td>URO</td>
        <td><b>hpmcounter6</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC07</b></td>
        <td>URO</td>
        <td><b>hpmcounter7</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC08</b></td>
        <td>URO</td>
        <td><b>hpmcounter8</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC09</b></td>
        <td>URO</td>
        <td><b>hpmcounter9</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC0A</b></td>
        <td>URO</td>
        <td><b>hpmcounter10</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC0B</b></td>
        <td>URO</td>
        <td><b>hpmcounter11</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC0C</b></td>
        <td>URO</td>
        <td><b>hpmcounter12</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC0D</b></td>
        <td>URO</td>
        <td><b>hpmcounter13</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC0E</b></td>
        <td>URO</td>
        <td><b>hpmcounter14</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC0F</b></td>
        <td>URO</td>
        <td><b>hpmcounter15</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC10</b></td>
        <td>URO</td>
        <td><b>hpmcounter16</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC11</b></td>
        <td>URO</td>
        <td><b>hpmcounter17</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC12</b></td>
        <td>URO</td>
        <td><b>hpmcounter18</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC13</b></td>
        <td>URO</td>
        <td><b>hpmcounter19</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC14</b></td>
        <td>URO</td>
        <td><b>hpmcounter20</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC15</b></td>
        <td>URO</td>
        <td><b>hpmcounter21</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC16</b></td>
        <td>URO</td>
        <td><b>hpmcounter22</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC17</b></td>
        <td>URO</td>
        <td><b>hpmcounter23</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC18</b></td>
        <td>URO</td>
        <td><b>hpmcounter24</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC19</b></td>
        <td>URO</td>
        <td><b>hpmcounter25</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC1A</b></td>
        <td>URO</td>
        <td><b>hpmcounter26</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC1B</b></td>
        <td>URO</td>
        <td><b>hpmcounter27</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC1C</b></td>
        <td>URO</td>
        <td><b>hpmcounter28</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC1D</b></td>
        <td>URO</td>
        <td><b>hpmcounter29</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC1E</b></td>
        <td>URO</td>
        <td><b>hpmcounter30</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC1F</b></td>
        <td>URO</td>
        <td><b>hpmcounter31</b></td>
        <td>Performance-monitoring counter</td>
    </tr>
    <tr>
        <td><b>0xC20</b></td>
        <td>URO</td>
        <td><b>vl</b></td>
        <td>Vector length</td>
    </tr>
    <tr>
        <td><b>0xC21</b></td>
        <td>URO</td>
        <td><b>vtype</b></td>
        <td>Vector data type register</td>
    </tr>
    <tr>
        <td><b>0xC22</b></td>
        <td>URO</td>
        <td><b>vlenb</b></td>
        <td>Vector register length in bytes</td>
    </tr>
    <tr>
        <td><b>0xDA0</b></td>
        <td>SRO</td>
        <td><b>scountovf</b></td>
        <td>Supervisor count overflow</td>
    </tr>
    <tr>
        <td><b>0xE12</b></td>
        <td>HRO</td>
        <td><b>hgeip</b></td>
        <td>Hypervisor guest external interrupt pending</td>
    </tr>
    <tr>
        <td><b>0xF11</b></td>
        <td>MRO</td>
        <td><b>mvendorid</b></td>
        <td>Vendor ID</td>
    </tr>
    <tr>
        <td><b>0xF12</b></td>
        <td>MRO</td>
        <td><b>marchid</b></td>
        <td>Architecture ID</td>
    </tr>
    <tr>
        <td><b>0xF13</b></td>
        <td>MRO</td>
        <td><b>mimpid</b></td>
        <td>Implementation ID</td>
    </tr>
    <tr>
        <td><b>0xF14</b></td>
        <td>MRO</td>
        <td><b>mhartid</b></td>
        <td>Hardware thread ID</td>
    </tr>
    <tr>
        <td><b>0xF15</b></td>
        <td>MRO</td>
        <td><b>mconfigptr</b></td>
        <td>Pointer to configuration data structure</td>
    </tr>
</table>