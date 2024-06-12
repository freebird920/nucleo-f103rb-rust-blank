use cortex_m::asm::nop;

pub fn delay_sys_clk_ms(ms: u32) {
    // 클럭 주파수 및 지연 루프 보정
    for _ in 0..100 * 8 * 4 * ms {
        nop();
    }
}

pub fn delay_sys_clk_10us(us_10: u32) {
    // 클럭 주파수 및 지연 루프 보정
    for _ in 0.. 8 * 4 * us_10 {
        nop();
    }
}
