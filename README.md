[CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) sub implementation in Rust

### OP codes

| ✓   | Opcode | Type      | Function            | Test | Explaination                                                                             |
| --- | ------ | --------- | ------------------- | ---- | ---------------------------------------------------------------------------------------- |
| ✗   | `0NNN` | `Call`    |                     | ✗    | Calls machine code routine at address NNN.                                               |
| ✗   | `00E0` | `Display` |                     | ✗    | Clears the screen.                                                                       |
| ✓   | `00EE` | `Flow`    | `ret()`             | ✗    | Returns from a subroutine.                                                               |
| ✓   | `1NNN` | `Flow`    | `jump()`            | ✗    | Jumps to address NNN.                                                                    |
| ✓   | `2NNN` | `Flow`    | `call()`            | ✗    | Calls subroutine at NNN.                                                                 |
| ✓   | `3XKK` | `Cond`    | `skip_eq`           | ✓    | Skips the next instruction if VX equals KK.                                              |
| ✓   | `4XKK` | `Cond`    | `skip_ne`           | ✓    | Skips the next instruction if VX does not equal KK                                       |
| ✓   | `5XY0` | `Cond`    | `skip_eq`           | ✓    | Skips the next instruction if VX equals VY.                                              |
| ✓   | `6XKK` | `Const`   | `set()`             | ✓    | Sets VX to NN.                                                                           |
| ✓   | `7XKK` | `Const`   | `add()`             | ✓    | Adds NN to VX.                                                                           |
| ✓   | `8XY0` | `Assig`   | `set()`             | ✓    | Sets VX to the value of VY.                                                              |
| ✓   | `8XY1` | `BitOp`   | `or_bitwise_set()`  | ✓    | Sets VX to VX or VY                                                                      |
| ✓   | `8XY2` | `BitOP`   | `add_bitwise_set()` | ✓    | Sets VX to VX and VY.                                                                    |
| ✓   | `8XY3` | `BitOp`   | `xor_bitwise_set()` | ✓    | Sets VX to VX xor VY.                                                                    |
| ✓   | `8XY4` | `Math`    | `add_xy()`          | ✓    | Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not.          |
| ✓   | `8XY5` | `Math`    | `sub_xy()`          | ✓    | VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not. |
| ✓   | `9XY0` | `Cond`    | `skip_ne()`         | ✓    | VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not. |
