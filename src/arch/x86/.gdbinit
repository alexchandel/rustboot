target remote localhost:1234
symbol-file boot/kernel.elf

set disassembly-flavor intel

b debug
c
