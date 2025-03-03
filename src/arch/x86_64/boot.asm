section .multiboot_header
header_start:
    ; Magic number for multiboot2
    dd 0xe85250d6                ; Multiboot2 magic number
    dd 0                         ; Architecture (0 = i386/x86)
    dd header_end - header_start ; Header length
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start)) ; Checksum

    ; Required end tag
    dw 0    ; Type
    dw 0    ; Flags
    dd 8    ; Size
header_end:

section .text
global _start
bits 32

_start:
    ; Set up stack
    mov esp, stack_top
    
    ; Call Rust kernel
    extern _start
    call _start
    
    ; Halt if we return
    cli
.hang:
    hlt
    jmp .hang

section .bss
stack_bottom:
    resb 64*1024  ; 64 KiB stack
stack_top: 