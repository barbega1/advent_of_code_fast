use std::fmt::Display;
use std::arch::naked_asm;
use std::intrinsics::unchecked_div;

pub fn part1(s: &str) -> impl Display {
    unsafe { inner1(s) }
}

#[naked]
unsafe fn inner1(s: &str) -> usize {
    // Register allocation 
    // - r8 will be used to accumulate the sum
    // - rsi will be used to track how much of the string has been processed
    // - rdi will be used to point to the next string char to be processed
    // - small numbers will be pushed onto the stack alongside the target number
    // - r9 will be used to store the operations currently being performed
    // - r10 will be used to store the index of small numbers
    // - r11 will be used to store the target number


    naked_asm!(
        "add rsi, rdi",             // Add the string pointer to the string length

        "21:",                      // 21 process the target number
        "xor r11, r11",             // Clear the target number
        "xor r10, r10",             // Clear the number of small numbers

        "22:",                      // 22 process a target digit
        "mov al, byte ptr [rdi]",   // Load the next character from the string
        "cmp al, 0x3a",             // Check if the character is a colon
        "je 23f",                   // If it is a colon move on to next part
        "sub al, 0x30",             // Convert the character to a number
        "imul r11, r11, 10",        // Multiply the target number by 10
        "add r11, rax",             // Add the new digit to the target number
        "add rdi, 1",               // Move to the next character in the string
        "jmp short 22b",            // Loop back to process the next digit

        "23:",                      // 23 process small numbers
        "add rdi, 2",               // Move by 2 to skip the colon and space
        "xor rcx, rcx",             // Clear the small number temp register
        "jmp 25f",                  // Move on to the next stage

        "24:",                      // 24 process a small number
        "add rdi, 1",               // Move by 1 to skip the space
        "add r10, 1",               // Increment the number of small numbers
        "push rcx",                 // Push the small number onto the stack
        "xor rcx, rcx",             // Clear the small number temp register

        "25:",                      // 25 process a small number digit
        "mov al, byte ptr [rdi]",   // Load the next character from the string
        "sub al, 0x30",             // Convert the character to a number
        "js 26f",                   // If not a digit see why
        "imul rcx, rcx, 10",        // Multiply the target number by 10
        "add rcx, rax",             // Add the new digit to the small number
        "add rdi, 1",               // Move to the next character in the string
        "jmp short 25b",            // Loop back to process the next digit

        "26:",                      // 30 process the operations
        "cmp al, 0xf0",             // Check if the character is a space or newline
        "je 24b",                   // If it is a space move on to next small number

        "add rdi, 1",               // Move by 1 to skip the newline
        "add r10, 1",               // Increment the number of small numbers
        "push rcx",                 // Push the small number onto the stack

        "mov r9, 0",                // Clear the operations register
        "mov r11, rax",             // Move the target number to the main register for division

        "27:",                      // 27 clear the stack
        "pop rcx",                  // Pop the first number from the stack
        "xor rdx, rdx",             // Clear the remainder register
        "div rcx",                  // Divide the target number by the small number
        "cmp rdx, 0",               // Check if the division was successful
        "je 28f",                   // If the division was successful move on to the next small number

        "sub r10, 1",               // Decrement the number of small numbers
        "cmp r10, 0",               // Check if there are any small numbers left
        "jne 27b",                  // Go back and pop them
        "jmp 29f",                  // Move on to the next stage

        "28:",                      // 28 multiply the target number by the small number
        "sub r10, 1",               // Decrement the number of small numbers
        "cmp r10, 0",               // Check if there are any small numbers left
        "jne 27b",                  // Go back and pop them

        "29:",

        "cmp rdi, rsi ",            // Check if end of file has been reached
        "jne 21b",                  // If the end of the file has been reached move on to the next stage
        "mov rax, rcx",             // Move the last small number to the return register
        "ret",                      // Return from the function
    )
}

pub fn part2(s: &str) -> impl Display {
    unsafe { inner2(s) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner2(s: &str) -> impl Display {
    let mut part2: i64 = 0;
    let bytes = s.as_bytes();

    let mut numbers = [0; 12];
    let mut original_target: i64 = 0;
    let mut number_index = 0;
    let mut temp_num: i64 = 0;
    let mut tries = 0;
    for c in bytes {
        match c {
            b':' => {
                original_target = temp_num;
                temp_num = 0;
            }
            b' ' => {
                if temp_num != 0 {
                    numbers[number_index] = temp_num as u16;
                    number_index += 1;
                    temp_num = 0;
                }
            }
            b'\n' => {
                numbers[number_index] = temp_num as u16;
                number_index += 1;
                temp_num = 0;
                let mut operators = [0; 12];
                let mut target = original_target;
                let mut i = number_index - 1;
                loop {
                    while i > 0 {
                        let small = *numbers.get_unchecked(i) as i64;
                        if small < 10 && (unchecked_div(target, 10) * 10) + small == target {
                            target = unchecked_div(target, 10) as i64;
                            *operators.get_unchecked_mut(i) = 2;
                        } else if small < 100
                            && (unchecked_div(target, 100) * 100) + small == target
                        {
                            target = unchecked_div(target, 100) as i64;
                            *operators.get_unchecked_mut(i) = 2;
                        } else if (unchecked_div(target, 1000) * 1000) + small == target {
                            target = unchecked_div(target, 1000) as i64;
                            *operators.get_unchecked_mut(i) = 2;
                        } else if (unchecked_div(target, small) as i64) * small as i64 == target {
                            target = unchecked_div(target, small) as i64;
                        } else {
                            *operators.get_unchecked_mut(i) = 1;
                            target = target - small as i64;
                            if target < 0 {
                                for j in i..number_index {
                                    if *operators.get_unchecked_mut(j) == 2 {
                                        *operators.get_unchecked_mut(j) = 0;
                                        if *numbers.get_unchecked(j) < 10 {
                                            target = target * 10 + *numbers.get_unchecked(j) as i64;
                                        } else if *numbers.get_unchecked(j) < 100 {
                                            target =
                                                target * 100 + *numbers.get_unchecked(j) as i64;
                                        } else {
                                            target =
                                                target * 1000 + *numbers.get_unchecked(j) as i64;
                                        }
                                        if (unchecked_div(target, *numbers.get_unchecked(j) as i64)
                                            as i64)
                                            * *numbers.get_unchecked(j) as i64
                                            == target
                                        {
                                            target = unchecked_div(
                                                target,
                                                *numbers.get_unchecked(j) as i64,
                                            )
                                                as i64;
                                            *operators.get_unchecked_mut(j) = 0;
                                        } else {
                                            target -= *numbers.get_unchecked(j) as i64;
                                            *operators.get_unchecked_mut(j) = 1;
                                        }
                                        i = j;
                                        break;
                                    } else if *operators.get_unchecked_mut(j) == 1 {
                                        *operators.get_unchecked_mut(j) = 0;
                                        target += *numbers.get_unchecked(j) as i64;
                                    } else {
                                        target *= *numbers.get_unchecked(j) as i64;
                                        target -= *numbers.get_unchecked(j) as i64;
                                        *operators.get_unchecked_mut(j) = 1;
                                        i = j;
                                        break;
                                    }
                                }
                                tries += 1;
                            }
                        }
                        i -= 1
                    }
                    if *numbers.get_unchecked(0) as i64 == target {
                        part2 += original_target;
                        break;
                    } else {
                        i = 100;
                        for j in 1..number_index {
                            if *operators.get_unchecked_mut(j) == 2 {
                                if *numbers.get_unchecked(j) < 10 {
                                    target = target * 10 + *numbers.get_unchecked(j) as i64;
                                } else if *numbers.get_unchecked(j) < 100 {
                                    target = target * 100 + *numbers.get_unchecked(j) as i64;
                                } else {
                                    target = target * 1000 + *numbers.get_unchecked(j) as i64;
                                }
                                if (unchecked_div(target, *numbers.get_unchecked(j) as i64) as i64)
                                    * *numbers.get_unchecked(j) as i64
                                    == target
                                {
                                    target = unchecked_div(target, *numbers.get_unchecked(j) as i64)
                                        as i64;
                                    *operators.get_unchecked_mut(j) = 0;
                                } else {
                                    target -= *numbers.get_unchecked(j) as i64;
                                    *operators.get_unchecked_mut(j) = 1;
                                }
                                i = j - 1;
                                break;
                            } else if *operators.get_unchecked_mut(j) == 1 {
                                *operators.get_unchecked_mut(j) = 0;
                                target += *numbers.get_unchecked(j) as i64;
                            } else {
                                target *= *numbers.get_unchecked(j) as i64;
                                target -= *numbers.get_unchecked(j) as i64;
                                *operators.get_unchecked_mut(j) = 1;
                                i = j - 1;
                                break;
                            }
                        }
                        tries += 1;
                        if i == 100 {
                            break;
                        }
                    }
                }
                number_index = 0;
            }
            _ => {
                temp_num = temp_num * 10 + (c - b'0') as i64;
            }
        }
    }
    println!("Tries: {}", tries);
    part2
}
