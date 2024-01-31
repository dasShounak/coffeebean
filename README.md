# ☕ coffeebean
A compiler written in Rust.

## What the heck is this?
This is just a (small?) project I'm working on. I don't have a clear goal at this point. I'll try to update this repo on a weekly basis.

## Why??
Well it all started with this:
> *"There are two kinds of programmers — those who have written compilers and those who haven't."  
> -Terry A. Davis*

No, actually...  
I am learning compiler design at college this semester so I thought this might be a good time to try building a compiler from scratch. It took me 4 days to write a simple comiler that can convert...
```c
return 70;
```
...this, to...
```asm
global _start
_start:
    mov rax, 60
    mov rdi, 70
    syscall
```
...this.

## But why rust?
Because I don't know Rust and I felt this might be a good chance to learn the language 🤷‍♂️

## Coffeebean????
I <s>like</s> love coffee ♥
