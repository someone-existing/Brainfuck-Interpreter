use std::{char, io, io::Read};

const MAX_MEM: usize = 100;

fn find_matching(c_ind: usize, src_vec: &Vec<char>) -> usize {
    if src_vec[c_ind] == '[' {
        let mut i = c_ind + 1;
        let mut count = 0;
        while i < src_vec.len() {
            match src_vec[i] {
                '[' => count += 1,
                ']' => {
                    if count == 0 {
                        return i;
                    }
                    count -= 1;
                }
                _ => (),
            }
            i += 1;
        }
    } else if src_vec[c_ind] == ']' {
        let mut i = c_ind - 1;
        let mut count = 0;
        while i > 0 {
            match src_vec[i] {
                ']' => count += 1,
                '[' => {
                    if count == 0 {
                        return i;
                    }
                    count -= 1;
                }
                _ => (),
            }
            i -= 1;
        }
    }
    return 0;
}

fn main() {
    loop {
        let mut src = String::new();
        io::stdin()
            .read_line(&mut src)
            .expect("Couldn't read source");
        let mut mem: [u8; MAX_MEM] = [0; MAX_MEM];
        let mut ptr_address = 0;
        let src_vec: Vec<char> = src.chars().collect();
        let mut c_ind = 0; // Current index
        print!("OUTPUT: ");
        while c_ind < src_vec.len() {
            match src_vec[c_ind] {
                '+' => {
                    // increment
                    mem[ptr_address] += 1;
                }
                '-' => {
                    // decrement, unless if decrementing makes the value negative
                    if mem[ptr_address] > 0 {
                        mem[ptr_address] -= 1;
                    }
                }
                '>' => {
                    // move right, as long as it stays within the bounds of memory
                    if ptr_address < MAX_MEM {
                        ptr_address += 1;
                    }
                }
                '<' => {
                    // move left, unless if moving left goes negative
                    if ptr_address > 0 {
                        ptr_address -= 1;
                    }
                }
                '.' => {
                    // output
                    if mem[ptr_address] < 10 {
                        print!("{}", mem[ptr_address]);
                    } else {
                        print!("{}", mem[ptr_address] as char)
                    }
                }
                ',' => {
                    // input
                    let mut input: [u8; 1] = [0; 1];
                    io::stdin()
                        .read(&mut input)
                        .expect("Couldn't read from stdin");
                    mem[ptr_address] = input[0];
                }
                '[' => {
                    // Jump past matching ] if the cell at the current index is 0
                    if mem[ptr_address] == 0 {
                        c_ind = find_matching(c_ind, &src_vec) + 1;
                    }
                }
                ']' => {
                    // Jump back to matching [ if the cell at the current index is nonzero
                    if mem[ptr_address] != 0 {
                        c_ind = find_matching(c_ind, &src_vec);
                    }
                }

                _ => (), // skip all other characters
            }
            c_ind += 1;
        }
        println!("\n--- PROGRAM FINISHED ---");
    }
}
