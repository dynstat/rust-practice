	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
@feat.00 = 0
	.file	"demo.dc3ed5978db21c9e-cgu.0"
	.def	_ZN37_$LT$f64$u20$as$u20$demo..MyTrait$GT$7do_work17h9f6619c23d305c8eE;
	.scl	3;
	.type	32;
	.endef
	.globl	__real@c1e0000000000000
	.section	.rdata,"dr",discard,__real@c1e0000000000000
	.p2align	3, 0x0
__real@c1e0000000000000:
	.quad	0xc1e0000000000000
	.globl	__real@41dfffffffc00000
	.section	.rdata,"dr",discard,__real@41dfffffffc00000
	.p2align	3, 0x0
__real@41dfffffffc00000:
	.quad	0x41dfffffffc00000
	.section	.text,"xr",one_only,_ZN37_$LT$f64$u20$as$u20$demo..MyTrait$GT$7do_work17h9f6619c23d305c8eE,unique,0
_ZN37_$LT$f64$u20$as$u20$demo..MyTrait$GT$7do_work17h9f6619c23d305c8eE:
	movsd	(%rcx), %xmm0
	xorl	%eax, %eax
	ucomisd	%xmm0, %xmm0
	maxsd	__real@c1e0000000000000(%rip), %xmm0
	minsd	__real@41dfffffffc00000(%rip), %xmm0
	cvttsd2si	%xmm0, %ecx
	cmovnpl	%ecx, %eax
	leal	(%rax,%rax,2), %eax
	retq

	.def	_ZN37_$LT$i32$u20$as$u20$demo..MyTrait$GT$7do_work17he9351de4a1244c50E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN37_$LT$i32$u20$as$u20$demo..MyTrait$GT$7do_work17he9351de4a1244c50E,unique,1
_ZN37_$LT$i32$u20$as$u20$demo..MyTrait$GT$7do_work17he9351de4a1244c50E:
	movl	(%rcx), %eax
	addl	%eax, %eax
	retq

	.def	_ZN3std2rt10lang_start17hcda5c77054204648E;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std2rt10lang_start17hcda5c77054204648E,unique,2
	.globl	_ZN3std2rt10lang_start17hcda5c77054204648E
_ZN3std2rt10lang_start17hcda5c77054204648E:
.seh_proc _ZN3std2rt10lang_start17hcda5c77054204648E
	subq	$56, %rsp
	.seh_stackalloc 56
	.seh_endprologue
	movq	%r8, %rax
	movq	%rdx, %r8
	leaq	48(%rsp), %r10
	movq	%rcx, (%r10)
	movb	%r9b, 32(%rsp)
	leaq	vtable.0(%rip), %rdx
	movq	%r10, %rcx
	movq	%rax, %r9
	callq	_ZN3std2rt19lang_start_internal17hf9be512cb4d4567aE
	nop
	.seh_startepilogue
	addq	$56, %rsp
	.seh_endepilogue
	retq
	.seh_endproc

	.def	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he9cdc59753bd0c21E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he9cdc59753bd0c21E,unique,3
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he9cdc59753bd0c21E:
.seh_proc _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he9cdc59753bd0c21E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h70142d768e703d77E
	xorl	%eax, %eax
	.seh_startepilogue
	addq	$40, %rsp
	.seh_endepilogue
	retq
	.seh_endproc

	.def	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h70142d768e703d77E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h70142d768e703d77E,unique,4
_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h70142d768e703d77E:
.seh_proc _ZN3std3sys9backtrace28__rust_begin_short_backtrace17h70142d768e703d77E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	callq	*%rcx
	#APP
	#NO_APP
	nop
	.seh_startepilogue
	addq	$40, %rsp
	.seh_endepilogue
	retq
	.seh_endproc

	.def	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h711c153b3c24c427E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h711c153b3c24c427E,unique,5
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h711c153b3c24c427E:
.seh_proc _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h711c153b3c24c427E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h70142d768e703d77E
	xorl	%eax, %eax
	.seh_startepilogue
	addq	$40, %rsp
	.seh_endepilogue
	retq
	.seh_endproc

	.def	_ZN4demo15process_dynamic17hc7706987329013a9E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4demo15process_dynamic17hc7706987329013a9E,unique,6
_ZN4demo15process_dynamic17hc7706987329013a9E:
	rex64 jmpq	*24(%rdx)

	.def	_ZN4demo15process_generic17h1c4798a783bb26edE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4demo15process_generic17h1c4798a783bb26edE,unique,7
_ZN4demo15process_generic17h1c4798a783bb26edE:
	movl	$30, %eax
	retq

	.def	_ZN4demo15process_generic17h8dcd10b1d07c4b7fE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4demo15process_generic17h8dcd10b1d07c4b7fE,unique,8
_ZN4demo15process_generic17h8dcd10b1d07c4b7fE:
	movl	$20, %eax
	retq

	.def	_ZN4demo4main17h193265a56c645843E;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4demo4main17h193265a56c645843E,unique,9
	.globl	_ZN4demo4main17h193265a56c645843E
_ZN4demo4main17h193265a56c645843E:
.seh_proc _ZN4demo4main17h193265a56c645843E
	pushq	%rsi
	.seh_pushreg %rsi
	pushq	%rdi
	.seh_pushreg %rdi
	pushq	%rbx
	.seh_pushreg %rbx
	subq	$48, %rsp
	.seh_stackalloc 48
	.seh_endprologue
	callq	_ZN4demo15process_generic17h8dcd10b1d07c4b7fE
	movl	%eax, %esi
	callq	_ZN4demo15process_generic17h1c4798a783bb26edE
	movl	%eax, %edi
	leaq	alloc_85b5ec2b6dd9d9b3c047fa3c0ea49204(%rip), %rcx
	leaq	vtable.1(%rip), %rdx
	callq	_ZN4demo15process_dynamic17hc7706987329013a9E
	movl	%eax, %ebx
	leaq	alloc_28677088aabd366d68223ae0630d6486(%rip), %rcx
	leaq	vtable.2(%rip), %rdx
	callq	_ZN4demo15process_dynamic17hc7706987329013a9E
	leaq	32(%rsp), %rcx
	movl	%esi, (%rcx)
	movl	%edi, 4(%rcx)
	movl	%ebx, 8(%rcx)
	movl	%eax, 12(%rcx)
	#APP
	#NO_APP
	.seh_startepilogue
	addq	$48, %rsp
	popq	%rbx
	popq	%rdi
	popq	%rsi
	.seh_endepilogue
	retq
	.seh_endproc

	.def	main;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,main,unique,10
	.globl	main
	.p2align	4
main:
	movq	%rdx, %r8
	movslq	%ecx, %rdx
	leaq	_ZN4demo4main17h193265a56c645843E(%rip), %rcx
	xorl	%r9d, %r9d
	jmp	_ZN3std2rt10lang_start17hcda5c77054204648E

	.section	.rdata,"dr",one_only,vtable.0,unique,11
	.p2align	3, 0x0
vtable.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h711c153b3c24c427E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he9cdc59753bd0c21E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17he9cdc59753bd0c21E

	.section	.rdata,"dr",one_only,alloc_85b5ec2b6dd9d9b3c047fa3c0ea49204,unique,12
	.p2align	2, 0x0
alloc_85b5ec2b6dd9d9b3c047fa3c0ea49204:
	.asciz	"\n\000\000"

	.section	.rdata,"dr",one_only,vtable.1,unique,13
	.p2align	3, 0x0
vtable.1:
	.asciz	"\000\000\000\000\000\000\000\000\004\000\000\000\000\000\000\000\004\000\000\000\000\000\000"
	.quad	_ZN37_$LT$i32$u20$as$u20$demo..MyTrait$GT$7do_work17he9351de4a1244c50E

	.section	.rdata,"dr",one_only,alloc_28677088aabd366d68223ae0630d6486,unique,14
	.p2align	3, 0x0
alloc_28677088aabd366d68223ae0630d6486:
	.ascii	"\000\000\000\000\000\000$@"

	.section	.rdata,"dr",one_only,vtable.2,unique,15
	.p2align	3, 0x0
vtable.2:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN37_$LT$f64$u20$as$u20$demo..MyTrait$GT$7do_work17h9f6619c23d305c8eE

	.globl	_fltused
