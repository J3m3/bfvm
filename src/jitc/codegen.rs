use super::AARCH64_INST_SIZE;

pub fn nop() -> [u8; AARCH64_INST_SIZE] {
    [0x1f, 0x20, 0x03, 0xd5]
}

pub fn ret() -> [u8; AARCH64_INST_SIZE] {
    [0xc0, 0x03, 0x5f, 0xd6]
}

pub fn sub_x19_x19_x8() -> [u8; AARCH64_INST_SIZE] {
    [0x73, 0x02, 0x08, 0xcb]
}

pub fn add_x19_x19_x8() -> [u8; AARCH64_INST_SIZE] {
    [0x73, 0x02, 0x08, 0x8b]
}

pub fn sub_w9_w9_w8() -> [u8; AARCH64_INST_SIZE] {
    [0x29, 0x01, 0x08, 0x4b]
}

pub fn str_w9_addrx19() -> [u8; AARCH64_INST_SIZE] {
    [0x69, 0x02, 0x00, 0xb9]
}

pub fn add_w9_w9_w8() -> [u8; AARCH64_INST_SIZE] {
    [0x29, 0x01, 0x08, 0x0b]
}

pub fn ldr_w9_addrx19() -> [u8; AARCH64_INST_SIZE] {
    [0x69, 0x02, 0x40, 0xb9]
}

fn movk_xn_immd16(xn: u8, immd16: u16, lsl: u8) -> [u8; AARCH64_INST_SIZE] {
    assert!(xn < 32);
    assert!(lsl == 0 || lsl == 16 || lsl == 32 || lsl == 48);
    let lsl = match lsl {
        0 => 0,
        16 => 1,
        32 => 2,
        48 => 3,
        _ => panic!("invalid lsl field in movk"),
    };
    // N-filled bits are a placeholder for immd16
    // 11110010 100NNNNN NNNNNNNN NNN01000
    let base = 0xf2800000; // big endian version of `movk xn, #0x0000, lsl #0`
    let instruction = base | xn as u32 | ((immd16 as u32) << 5) | ((lsl as u32) << 16 + 5);
    instruction.to_le_bytes() // reverse byte order to ensure that it's little endian (because it's aarch64!)
}

fn mov_xn_immd16(xn: u8, immd16: u16) -> [u8; AARCH64_INST_SIZE] {
    assert!(xn < 32);

    // N-filled bits are a placeholder for immd16
    // 11010010 100NNNNN NNNNNNNN NNN01000
    let base = 0xd2800000; // big endian version of `mov xn, #0x0000`
    let instruction = base | xn as u32 | ((immd16 as u32) << 5);
    instruction.to_le_bytes() // reverse byte order to ensure that it's little endian (because it's aarch64!)
}

pub fn mov_x8_i32operand(operand: i32) -> [u8; AARCH64_INST_SIZE * 2] {
    const SZ: usize = AARCH64_INST_SIZE;

    let immd0 = (operand & 0x0000ffff) as u16;
    let immd1 = ((operand >> 16) & 0xffff) as u16;

    let mov_inst: [u8; AARCH64_INST_SIZE] = mov_xn_immd16(8, immd0);
    let movk_inst: [u8; AARCH64_INST_SIZE] = movk_xn_immd16(8, immd1, 16);

    let mut result: [u8; SZ * 2] = [0; SZ * 2];
    result[..SZ].copy_from_slice(&mov_inst);
    result[SZ..SZ * 2].copy_from_slice(&movk_inst);
    result
}

pub fn mov_x19_u64operand(operand: u64) -> [u8; AARCH64_INST_SIZE * 4] {
    const SZ: usize = AARCH64_INST_SIZE;

    let immd0 = (operand & 0xffff) as u16;
    let immd1 = ((operand >> 16) & 0xffff) as u16;
    let immd2 = ((operand >> 32) & 0xffff) as u16;
    let immd3 = ((operand >> 48) & 0xffff) as u16;

    let mov_inst0 = mov_xn_immd16(19, immd0);
    let movk_inst1 = movk_xn_immd16(19, immd1, 16);
    let movk_inst2 = movk_xn_immd16(19, immd2, 32);
    let movk_inst3 = movk_xn_immd16(19, immd3, 48);

    let mut result = [0; SZ * 4];
    result[..SZ].copy_from_slice(&mov_inst0);
    result[SZ..SZ * 2].copy_from_slice(&movk_inst1);
    result[SZ * 2..SZ * 3].copy_from_slice(&movk_inst2);
    result[SZ * 3..SZ * 4].copy_from_slice(&movk_inst3);
    result
}

pub fn syscall_write() -> [u8; AARCH64_INST_SIZE * 5] {
    const SZ: usize = AARCH64_INST_SIZE;
    let mut result = [0; SZ * 5];

    // mov x0, #1
    // mov x1, x19
    // mov x2, #1    /* this is possible because aarch64 uses little-endian (though I'm not 100% sure) */
    // mov x8, #64
    // svc #0
    result[..SZ].copy_from_slice(&[0x20, 0x00, 0x80, 0xd2]);
    result[SZ..SZ * 2].copy_from_slice(&[0xe1, 0x03, 0x13, 0xaa]);
    result[SZ * 2..SZ * 3].copy_from_slice(&[0x22, 0x00, 0x80, 0xd2]);
    result[SZ * 3..SZ * 4].copy_from_slice(&[0x08, 0x08, 0x80, 0xd2]);
    result[SZ * 4..SZ * 5].copy_from_slice(&[0x01, 0x00, 0x00, 0xd4]);
    result
}

pub fn syscall_read() -> [u8; AARCH64_INST_SIZE * 5] {
    const SZ: usize = AARCH64_INST_SIZE;
    let mut result = [0; SZ * 5];

    // mov x0, #0
    // mov x1, x19
    // mov x2, #1
    // mov x8, #63
    // svc #0
    result[..SZ].copy_from_slice(&[0x00, 0x00, 0x80, 0xd2]);
    result[SZ..SZ * 2].copy_from_slice(&[0xe1, 0x03, 0x13, 0xaa]);
    result[SZ * 2..SZ * 3].copy_from_slice(&[0x22, 0x00, 0x80, 0xd2]);
    result[SZ * 3..SZ * 4].copy_from_slice(&[0xe8, 0x07, 0x80, 0xd2]);
    result[SZ * 4..SZ * 5].copy_from_slice(&[0x01, 0x00, 0x00, 0xd4]);
    result
}

fn cbz_xn_immd19(xn: u8, immd19: i32) -> [u8; AARCH64_INST_SIZE] {
    assert!(xn < 32);

    let base = 0xb4000000u32; // big-endian version of `cbz xn, #immd19`
    let instruction = base | xn as u32 | ((immd19 as u32) << 5); // immd19 is always positive
    instruction.to_le_bytes()
}

fn cbnz_xn_immd19(xn: u8, immd19: i32) -> [u8; AARCH64_INST_SIZE] {
    assert!(xn < 32);

    let base = 0xb5000000u32; // big-endian version of `cbnz xn, #immd19`
    let sign_shrunk = (immd19 & 0x0007ffff) as u32; // immd19 is always negative
    let instruction = base | xn as u32 | (sign_shrunk << 5);
    instruction.to_le_bytes()
}

pub type CondNearBranch = [u8; AARCH64_INST_SIZE * 3];

pub fn cbz_x9_addrx19_immd19(operand: i32) -> CondNearBranch {
    const SZ: usize = AARCH64_INST_SIZE;
    let mut result = [0; SZ * 3];

    // ldr x9, [x19]
    // nop
    // cbz x9, #immd19
    result[..SZ].copy_from_slice(&ldr_w9_addrx19());
    result[SZ..SZ * 2].copy_from_slice(&nop());
    result[SZ * 2..SZ * 3].copy_from_slice(&cbz_xn_immd19(9, operand));
    result
}

pub fn cbnz_x9_addrx19_immd19(operand: i32) -> CondNearBranch {
    const SZ: usize = AARCH64_INST_SIZE;
    let mut result = [0; SZ * 3];

    // ldr x9, [x19]
    // nop
    // cbnz x9, #immd19
    result[..SZ].copy_from_slice(&ldr_w9_addrx19());
    result[SZ..SZ * 2].copy_from_slice(&nop());
    result[SZ * 2..SZ * 3].copy_from_slice(&cbnz_xn_immd19(9, operand));
    result
}
