bits 16
org 0x7C00

%include "build/kernel_sectors.inc"

start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00

    ; Guarda o drive de boot
    mov [boot_drive], dl

    ; Carrega o kernel para 0x1000
    mov bx, 0x1000
    mov ah, 0x02
    mov al, KERNEL_SECTORS ; Sectors number
    mov ch, 0
    mov cl, 2
    mov dh, 0
    mov dl, [boot_drive]
    int 0x13
    jc disk_error

    ; Carrega GDT
    lgdt [gdt_descriptor]

    ; Entra em protected mode
    mov eax, cr0
    or eax, 1
    mov cr0, eax

    jmp CODE_SEL:protected_mode


disk_error:
    mov si, disk_error_msg

.print:
    lodsb
    test al, al
    jz .hang

    mov ah, 0x0E
    mov bh, 0
    int 0x10

    jmp .print

.hang:
    cli
    hlt
    jmp .hang


; ============================================================
; 32-bit Protected Mode
; ============================================================

bits 32

protected_mode:
    mov ax, DATA_SEL
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax

    mov esp, 0x90000

    ; --------------------------------------------------------
    ; Cria as tabelas de páginas
    ; --------------------------------------------------------

    ; PML4 = 0x10000
    ; PDP  = 0x11000
    ; PD   = 0x12000

    ; Limpa as tabelas
    mov edi, 0x10000
    mov ecx, 0x3000
    xor eax, eax
    rep stosb

    ; PML4[0] -> PDP
    mov eax, 0x11003
    mov [0x10000], eax

    ; PDP[0] -> PD
    mov eax, 0x12003
    mov [0x11000], eax

    ; PD[0] -> 2 MiB page
    ; Present + Writable + Huge Page
    mov eax, 0x00000083
    mov [0x12000], eax

    ; --------------------------------------------------------
    ; Ativa PAE
    ; --------------------------------------------------------

    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; --------------------------------------------------------
    ; Ativa long mode via EFER
    ; --------------------------------------------------------

    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; --------------------------------------------------------
    ; Ativa paginação
    ; --------------------------------------------------------

    mov eax, 0x10000
    mov cr3, eax

    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ; --------------------------------------------------------
    ; Entra em Long Mode
    ; --------------------------------------------------------

    jmp CODE64_SEL:long_mode


; ============================================================
; 64-bit Long Mode
; ============================================================

bits 64

long_mode:
    ; Segmentos de dados
    mov ax, DATA64_SEL
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax

    ; Stack 64-bit
    mov rsp, 0x90000

    ; Kernel está em 0x1000
    jmp 0x1000


; ============================================================
; GDT
; ============================================================

gdt_start:

    ; Null descriptor
    dq 0

; 32-bit code
gdt_code:
    dw 0xFFFF
    dw 0
    db 0
    db 10011010b
    db 11001111b
    db 0

; 32-bit data
gdt_data:
    dw 0xFFFF
    dw 0
    db 0
    db 10010010b
    db 11001111b
    db 0

; 64-bit code
gdt_code64:
    dw 0
    dw 0
    db 0
    db 10011010b
    db 00100000b
    db 0

; 64-bit data
gdt_data64:
    dw 0
    dw 0
    db 0
    db 10010010b
    db 00000000b
    db 0

gdt_end:


gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start


CODE_SEL equ gdt_code - gdt_start
DATA_SEL equ gdt_data - gdt_start

CODE64_SEL equ gdt_code64 - gdt_start
DATA64_SEL equ gdt_data64 - gdt_start


; ============================================================
; Boot data
; ============================================================

boot_drive db 0
disk_error_msg db "Disk read error", 0


times 510-($-$$) db 0
dw 0xAA55
