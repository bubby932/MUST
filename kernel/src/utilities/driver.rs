use core::arch::asm;

pub unsafe fn outb(port: u16, value: u8) {
	asm!("out dx, al", in("dx") port, in("al") value);
}

pub unsafe fn inb(port: u16) -> u8 {
	let value: u8;
	asm!("in al, dx", in("dx") port, out("al") value);
	value
}

pub unsafe fn outw(port: u16, value: u16) {
	asm!("out dx, ax", in("dx") port, in("ax") value);
}

pub unsafe fn inw(port: u16) -> u16 {
	let value: u16;
	asm!("in ax, dx", in("dx") port, out("ax") value);
	value
}

pub unsafe fn outl(port: u16, value: u32) {
	asm!("out dx, eax", in("dx") port, in("eax") value);
}

pub unsafe fn inl(port: u16) -> u32 {
	let value: u32;
	asm!("in eax, dx", in("dx") port, out("eax") value);
	value
}

pub unsafe fn outq(port: u16, value: u64) {
	asm!("out dx, rax", in("dx") port, in("rax") value);
}

pub unsafe fn inq(port: u16) -> u64 {
	let value: u64;
	asm!("in rax, dx", in("dx") port, out("rax") value);
	value
}