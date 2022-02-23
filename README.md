# Macro OSC
This is an implementation of an [OSC (Open Sound Control)](https://opensoundcontrol.stanford.edu/) formater using rust macros.

This allows the code to boil down to simple move commands, omitting all bound checks and branches.

`osc_format!("/foo", 1000, -1, "hello", 1.234, 5.678)`

compiles down to

```
mov     dword ptr [rax], 1869571631
mov     dword ptr [rax + 8], 1936288044
mov     word ptr [rax + 12], 26214
movabs  rcx, -2617245696
mov     qword ptr [rax + 16], rcx
mov     ecx, dword ptr [rip + "hello"]
mov     dword ptr [rax + 24], ecx
mov     cl, byte ptr [rip + "hello"+4]
mov     byte ptr [rax + 28], cl
movabs  rcx, 3292893567113207103
mov     qword ptr [rax + 32], rcx
```
