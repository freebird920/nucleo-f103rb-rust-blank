use cortex_m::asm::nop;

pub fn delay_ms(ms: u32) {
    // 클럭 주파수 및 지연 루프 보정
    let cycles_per_ms = 32; // 1ms당 NOP 실행 횟수 (보정 필요)
    for _ in 0..(ms * cycles_per_ms) {
        nop();
    }
}
// pub fn delay_tik(tiks: u32) {
//     // 클럭 주파수 및 지연 루프 보정
//     let cycles_per_tiks = 1; // 1ms당 NOP 실행 횟수 (보정 필요)
//     for _ in 0..(tiks * cycles_per_tiks) {
//         nop();
//     }
// }