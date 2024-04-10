	.section .text.entry
	.globl _start # 入口地址依据链接脚本放在BASE_ADDRESS处
_start:
	la sp, boot_stack_top # 将sp设置为栈空间的栈顶
	call rust_main # 函数调用rust_main

	.section .bss.stack # 栈空间命名
	.globl boot_stack
boot_stack_lower_bound: # 栈底地址全局符号标识
	.space 4096 * 16 # 预留了一块大小为4096 * 16字节，也就是64KiB的空间
	.globl boot_stack_top
boot_stack_top: # 栈顶地址全局符号标识