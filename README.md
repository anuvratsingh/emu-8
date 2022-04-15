[CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) sub implementation in Rust


### OP codes
|- [x]|Opcode|Type|Explaination|
|-|-|-|-|
|-[ ]|`0NNN`|`Call`|Calls machine code routine (RCA 1802 for COSMAC VIP) at address NNN. Not necessary for most ROMs.| 
|- [x]|`00EE`|`Flow`|Returns from a subroutine.|
|- [x]|`2NNN`|`Flow`|Calls subroutine at NNN.|
|- [x]|`8XY4`|`Math`|Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not.|


