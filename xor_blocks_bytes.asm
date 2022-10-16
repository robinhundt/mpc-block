example::xor_blocks_bytes:
        cmp     rsi, rcx
        cmova   rsi, rcx
        test    rsi, rsi
        je      .LBB1_7
        cmp     rsi, 1
        jne     .LBB1_3
        xor     eax, eax
        jmp     .LBB1_5
.LBB1_3:
        mov     r8, rsi
        and     r8, -2
        mov     ecx, 16
        xor     eax, eax
.LBB1_4:
        movaps  xmm0, xmmword ptr [rdi + rcx - 16]
        xorps   xmm0, xmmword ptr [rdx + rcx - 16]
        movaps  xmm1, xmmword ptr [rdi + rcx]
        movaps  xmmword ptr [rdi + rcx - 16], xmm0
        xorps   xmm1, xmmword ptr [rdx + rcx]
        add     rax, 2
        movaps  xmmword ptr [rdi + rcx], xmm1
        add     rcx, 32
        cmp     r8, rax
        jne     .LBB1_4
.LBB1_5:
        test    sil, 1
        je      .LBB1_7
        shl     rax, 4
        movaps  xmm0, xmmword ptr [rdi + rax]
        xorps   xmm0, xmmword ptr [rdx + rax]
        movaps  xmmword ptr [rdi + rax], xmm0
.LBB1_7:
        ret