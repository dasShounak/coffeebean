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

---

# Usage
1. To begin with, you must have a `*.cb` file (actually, any plaintext file will do, but the extension makes it look awesome, for the time being). Write a C-like return statement:  
    ```c
    return 20;
    ```
2. Pass this file as an argument.  
    ```sh
    cargo run <file.cb>
    ```
3. It will create an `out.asm` file. In order to create the binary, run this command (you might need to install `nasm` and `ld` packages first):  
   ```sh
   nasm -felf64 out.asm && ld out.o -o out
   ```
4. Now run the binary  
   ```sh
   ./out
   ```
5. It won't print anything as it calls only the `exit` syscall with a return value (20 in this case). To check the return value, run:  
   ```sh
   echo $?
   ```
   It should print 20.

