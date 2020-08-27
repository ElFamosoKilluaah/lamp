use crate::base::vm::VM;

#[test]
pub fn vm_add_test() {
    let bin = vec![
        // LOAD 13, 15, 15: Put in the register 13 the u16 represented by 15 and 15
        15, 13, 15, 15,
        // LOAD 14, 16, 16: Put in the register 14 the u16 represented by 16 and 16
        15, 14, 16, 16,
        // ADD 13, 14, 15: Put in the register 15 the result of the register 13 + the register 14
        1, 13, 14, 15,
    ];

    let mut vm = VM::new(bin);
    let _ = vm.run();
    let expected_value = ((15 << 8) | 15) + ((16 << 8) | 16);
    assert_eq!(*vm.get_register(15), expected_value);
}

#[test]
pub fn vm_sub_test() {
    let bin = vec![
        // LOAD 13, 15, 15: Put in the register 13 the u16 represented by 15 and 15
        15, 13, 15, 15,
        // LOAD 14, 16, 16: Put in the register 14 the u16 represented by 16 and 16
        15, 14, 16, 16,
        // SUB 13, 14, 15: Put in the register 15 the result of the register 13 - the register 14
        2, 13, 14, 15,
    ];

    let mut vm = VM::new(bin);
    let _ = vm.run();
    let expected_value = ((15 << 8) | 15) - ((16 << 8) | 16);
    assert_eq!(*vm.get_register(15), expected_value);
}

#[cfg(test)]
pub fn registers_dump(vm: &VM) {
    for i in 0..32 {
        println!("Register {}: {}", &i, vm.get_register(i));
    }
}

#[test]
pub fn vm_mul_test() {
    let bin = vec![
        // LOAD 13, 15, 15: Put in the register 13 the u16 represented by 15 and 15
        15, 13, 15, 15,
        // LOAD 14, 16, 16: Put in the register 14 the u16 represented by 16 and 16
        15, 14, 16, 16,
        // MUL 13, 14, 15: Put in the register 15 the result of the register 13 * the register 14
        3, 13, 14, 15,
    ];

    let mut vm = VM::new(bin);
    let _ = vm.run();
    let expected_value = ((15 << 8) | 15) * ((16 << 8) | 16);
    assert_eq!(*vm.get_register(15), expected_value);
}

#[test]
pub fn vm_mod_test() {
    let bin = vec![
        // LOAD 13, 15, 15: Put in the register 13 the u16 represented by 15 and 15
        15, 13, 15, 15,
        // LOAD 14, 16, 16: Put in the register 14 the u16 represented by 16 and 16
        15, 14, 16, 16,
        // MOD 13, 14, 15: Put in the register 15 the result of the register 13 / the register 14. The remainder goes into a special register
        4, 13, 14, 15,
    ];

    let mut vm = VM::new(bin);
    let _ = vm.run();
    let expected_value = ((15 << 8) | 15) / ((16 << 8) | 16);
    assert_eq!(*vm.get_register(15), expected_value);
}

#[test]
pub fn vm_load_test() {
    let bin = vec![
        // LOAD 13, 15, 15: Put in the register 13 the u16 represented by 15 and 15
        15, 13, 15, 15,
    ];
    let mut vm = VM::new(bin);
    let _ = vm.run();
    let expected_value = (15 << 8) | 15;
    assert_eq!(*vm.get_register(13), expected_value);
}
