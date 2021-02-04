use std::env;
use std::fs;
use std::process;

use xmas_elf::ElfFile;

use bad64::{disassemble, Operand, Reg};

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        eprintln!("Usage: {} <elf>", argv[0]);
        process::exit(1);
    }

    let buf = fs::read(&argv[1]).unwrap();

    let elf = ElfFile::new(&buf).unwrap();

    let text_section = elf.find_section_by_name(".text").unwrap();

    let base = text_section.address();
    let size = text_section.size();
    let bytes = text_section.raw_data(&elf);

    println!("disassembling {} bytes from .text @ {:#x}", size, base);

    for decoded in disassemble(bytes, base).filter_map(Result::ok) {
        let ops = decoded.operands();

        match ops {
            &[Operand::Reg { reg: Reg::XZR, .. }, ..] => println!(
                "64bit zero reg as first operand @ {:x} in {:?}",
                decoded.address(),
                decoded.operation()
            ),
            _ => (),
        }
    }
}