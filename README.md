# cpu-cache-test

.text:0000000140001184                 cmp     i, 100000000
.text:000000014000118A                 jz      short loc_1400011A7
.text:000000014000118C                 cmp     length, index
.text:000000014000118F                 jbe     loc_140001525
.text:0000000140001195                 lea     rdi, [i+1]      ; rax=i
.text:0000000140001199                 add     i, [base+index*8] ; rdx = index
.text:000000014000119D                 xor     edx, edx        ; set edx to zero
.text:000000014000119F                 div     max             ; reminder in rdx
.text:00000001400011A2                 mov     i, rdi
.text:00000001400011A5                 jmp     short loc_140001184
.text:00000001400011A7 ; --------------------------------------------