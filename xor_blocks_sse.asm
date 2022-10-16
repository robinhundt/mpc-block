example::xor_blocks_sse:
        cmp     rsi, rcx
        cmova   rsi, rcx
        test    rsi, rsi
        je      .LBB0_7
        cmp     rsi, 1
        jne     .LBB0_3
        xor     eax, eax
        jmp     .LBB0_5
.LBB0_3:
        mov     r8, rsi
        and     r8, -2
        mov     ecx, 16
        xor     eax, eax
.LBB0_4:
        movaps  xmm0, xmmword ptr [rdx + rcx - 16]
        xorps   xmm0, xmmword ptr [rdi + rcx - 16]
        movaps  xmmword ptr [rdi + rcx - 16], xmm0
        movaps  xmm0, xmmword ptr [rdx + rcx]
        xorps   xmm0, xmmword ptr [rdi + rcx]
        add     rax, 2
        movaps  xmmword ptr [rdi + rcx], xmm0
        add     rcx, 32
        cmp     r8, rax
        jne     .LBB0_4
.LBB0_5:
        test    sil, 1
        je      .LBB0_7
        shl     rax, 4
        movaps  xmm0, xmmword ptr [rdx + rax]
        xorps   xmm0, xmmword ptr [rdi + rax]
        movaps  xmmword ptr [rdi + rax], xmm0
.LBB0_7:
        ret