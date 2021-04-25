use crate::mem::Mem;
use crate::utils::*;
use crate::bus::*;
struct Register {}

impl Register {
    pub const A: usize = 0;
    pub const B: usize = 1;
    pub const D: usize = 2;
    pub const H: usize = 3;
    pub const F: usize = 4;
    pub const C: usize = 5;
    pub const E: usize = 6;
    pub const L: usize = 7;
    //I m not sure ebout these one
    //pub const R :usize =8;
    //pub const I :usize =9;
    pub const IX_L: usize = 10;
    pub const IX_H: usize = 11;
    pub const IY_L: usize = 12;
    pub const IY_H: usize = 13;
    pub const SP_L: usize = 14;
    pub const SP_H: usize = 15;
    pub const WZ_H: usize = 16;
    pub const WZ_L: usize = 17;

    //define R as register 
    pub const R_A : u8 = 7;
    pub const R_B : u8 = 0;
    pub const R_C : u8 = 1;
    pub const R_D : u8 = 2;
    pub const R_E : u8 = 3;
    pub const R_H : u8 = 4;
    pub const R_L : u8 = 5;

   
    pub const R_BC : u8 = 0;
    pub const R_DE : u8 = 1;
    pub const R_HL : u8 = 2;
    pub const R_AF : u8 = 3;

    pub const F_C: u8 = 0;
    pub const F_N: u8 = 1;
    pub const F_PV: u8 = 2;
    pub const F_X: u8 = 3;
    //REUSED AS H
    //pub const F_N2: u8 = 4;
    //REUSED AS H
    pub const F_H: u8 = 4;
    pub const F_X2: u8 = 5;
    pub const F_Z: u8 = 6;
    pub const F_S: u8 = 7;


}
const TOTAL_REGISTER: usize = 18;

#[derive(Debug)]
pub struct Registers {
    //GENERAL REG
    reg: [u8; TOTAL_REGISTER],
    pc: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            reg: [0; TOTAL_REGISTER],
            pc: 0,
        }
    }

    pub fn a(&self) -> u8 {
        self.reg[Register::A]
    }
    pub fn set_a(&mut self, v: u8) {
        self.reg[Register::A] = v;
    }

    pub fn b(&self) -> u8 {
        self.reg[Register::B]
    }
    pub fn set_b(&mut self, v: u8) {
        self.reg[Register::B] = v;
    }

    pub fn d(&self) -> u8 {
        self.reg[Register::D]
    }
    pub fn set_d(&mut self, v: u8) {
        self.reg[Register::D] = v;
    }

    pub fn h(&self) -> u8 {
        self.reg[Register::H]
    }
    pub fn set_h(&mut self, v: u8) {
        self.reg[Register::H] = v;
    }

    pub fn f(&self) -> u8 {
        self.reg[Register::F]
    }
    pub fn set_f(&mut self, v: u8) {
        self.reg[Register::F] = v;
    }

    pub fn c(&self) -> u8 {
        self.reg[Register::C]
    }
    pub fn set_c(&mut self, v: u8) {
        self.reg[Register::C] = v;
    }

    pub fn e(&self) -> u8 {
        self.reg[Register::E]
    }
    pub fn set_e(&mut self, v: u8) {
        self.reg[Register::E] = v;
    }

    pub fn l(&self) -> u8 {
        self.reg[Register::L]
    }
    pub fn set_l(&mut self, v: u8) {
        self.reg[Register::L] = v;
    }

    pub fn bc(&self) -> u16 {
        //COULD u16 from index
        get_u16_from_2_u8(self.reg[Register::B], self.reg[Register::C])
    }
    pub fn set_bc(&mut self, v: u16) {
        //COULD u16 from index
        let (b, c) = get_2_u8_from_u16(v);
        self.reg[Register::B] = b;
        self.reg[Register::C] = c;
    }
    pub fn de(&self) -> u16 {
        //COULD u16 from index
        get_u16_from_2_u8(self.reg[Register::D], self.reg[Register::E])
    }
    pub fn set_de(&mut self, v: u16) {
        //COULD u16 from index
        let (d, e) = get_2_u8_from_u16(v);
        self.reg[Register::D] = d;
        self.reg[Register::E] = e;
    }

    pub fn hl(&self) -> u16 {
        //COULD u16 from index
        get_u16_from_2_u8(self.reg[Register::H], self.reg[Register::L])
    }

    pub fn set_hl(&mut self, v: u16) {
        //COULD u16 from index
        let (h, l) = get_2_u8_from_u16(v);
        self.reg[Register::H] = h;
        self.reg[Register::L] = l;
    }

    pub fn sp(&self) -> u16 {
        //COULD u16 from index
        get_u16_from_2_u8(self.reg[Register::SP_H], self.reg[Register::SP_L])
    }
    pub fn set_sp(&mut self, v: u16) {
        //COULD u16 from index
        let (s, p) = get_2_u8_from_u16(v);
        self.reg[Register::SP_H] = s;
        self.reg[Register::SP_L] = p;
    }
    pub fn wz(&self) -> u16 {
        //COULD u16 from index
        get_u16_from_2_u8(self.reg[Register::WZ_H], self.reg[Register::WZ_L])
    }
    pub fn set_wz(&mut self, v: u16) {
        //COULD u16 from index
        let (s, p) = get_2_u8_from_u16(v);
        self.reg[Register::WZ_H] = s;
        self.reg[Register::WZ_L] = p;
    }
    //FLAG
    fn set_carry_flag(&mut self) {
        self.reg[Register::F] |= 1 << Register::F_C;
    }
    fn get_carry_flag(&mut self) -> bool {
        self.reg[Register::F] & 1 << Register::F_C == 0
    }
    fn clear_carry_flag(&mut self) {
        self.reg[Register::F] &= !(1 << Register::F_C)
    }
    //Do we need to set N2 will see.
    fn set_addsubstract_flag(&mut self) {
        self.reg[Register::F] |= 1 << Register::F_N;
    }
    fn get_addsubstract_flag(&mut self) -> bool {
        self.reg[Register::F] & 1 << Register::F_N == 0
    }
    fn clear_addsubstract_flag(&mut self) {
        self.reg[Register::F] &= !(1 << Register::F_N)
    }

    fn set_parity_overflow_flag(&mut self) {
        self.reg[Register::F] |= 1 << Register::F_PV;
    }
    fn get_parity_overflow_flag(&mut self) -> bool {
        self.reg[Register::F] & 1 << Register::F_PV == 0
    }
    fn clear_parity_overflow_flag(&mut self) {
        self.reg[Register::F] &= !(1 << Register::F_PV)
    }

    fn set_half_carry_flag(&mut self) {
        self.reg[Register::F] |= 1 << Register::F_H;
    }
    fn get_half_carry_flag(&mut self) -> bool {
        self.reg[Register::F] & 1 << Register::F_H == 0
    }
    fn clear_half_carry_flag(&mut self) {
        self.reg[Register::F] &= !(1 << Register::F_H)
    }

    fn set_zero_flag(&mut self) {
        self.reg[Register::F] |= 1 << Register::F_Z;
    }
    fn get_zero_flag(&mut self) -> bool {
        self.reg[Register::F] & 1 << Register::F_Z == 0
    }
    fn clear_zero_flag(&mut self) {
        self.reg[Register::F] &= !(1 << Register::F_Z)
    }
    
    fn set_sign_flag(&mut self) {
        self.reg[Register::F] |= 1 << Register::F_S;
    }
    fn get_sign_flag(&mut self) -> bool {
        self.reg[Register::F] & 1 << Register::F_S == 0
    }
    fn clear_sign_flag(&mut self) {
        self.reg[Register::F] &= !(1 << Register::F_S)
    }
    // fn set_carry_flag(&mut self) {
    //     self.reg[Register::F] |= Register::F_C << 0;
    // }
    // fn get_carry_flag(&mut self) -> bool {
    //     self.reg[Register::F] & Register::F_C << 0 == 0
    // }
    // fn clear_carry_flag(&mut self) {
    //     self.reg[Register::F] &= !(Register::F_C << 0)
    // }
    
}

pub struct Cpu<M: Mem> {
    pub c: u64,
    regs: Registers,
    pub mem: M,
    pub debug: bool,
    pub cycles: u128,

}


impl<M: BusSpace + Mem> Cpu<M> {
    pub fn new(mem: M, debug: bool) -> Cpu<M> {
        Cpu {
            c: 0,
            regs: Registers::new(),
            mem: mem,
            debug: debug,
            cycles: 0,
        }
    }

    pub fn push(&mut self,v:u16){
        self.mem.storew(self.regs.sp(), v);
        self.regs.set_sp(self.regs.sp()-2);
    }
    pub fn trace(&self) {
        if self.debug {
            println!("PC : {:08X} | SP : {:04X} | {:?}", self.regs.pc,self.regs.sp(), self.regs)
        }
    }
    pub fn trace_op(&self,op:u8,x:u8,y:u8,z:u8) {
        if self.debug {
            println!("OPERAND : {:2X} | {:08b} | x:{:b} y:{:b} z:{:b} | ({},{},{})", op,op, x, y, z, x, y, z);
        }
    }


    fn decode_op(&mut self, addr: u16) {
        let op = self.mem.load(addr);

        
        //Z80 seems to be split in multiple type

        let (x, y, z) = (op >> 6, (op >> 2 & 0x0F), op & 0b111);
        if self.debug {
            self.trace_op(op, x, y, z);
        }
        
        let cycle = match (x, y, z) {
            //LD BC
            (0, 0, 1) => {
                let n1 = self.mem.load(addr + 1);
                let n2 = self.mem.load(addr + 2);
                self.regs.set_bc(get_u16_from_2_u8(n2, n1));
                self.regs.pc += 3;
                0
            }
            //DEC BC
            (0, 2, 3) => {
                self.regs.set_bc(self.regs.bc() - 1);
                self.regs.pc += 1;
                1
            }
            //DEC H
            (0, 9, 5) => {
                self.regs.set_h(self.regs.h() - 1);
                self.regs.pc += 1;
                1
            }
            // JR NZ
            (0, 8, 0) => {
                //Should implement CC
                //HOW MUCH
                let o = self.mem.load(addr + 1) as i8;
                let nz = self.regs.get_zero_flag();
                if nz {
                    if o.is_negative() {
                        let abs = o.wrapping_abs() as u16;
                        self.regs.pc -= abs - 2 //we fetched the next operand first so +2 :S
                    } else {
                        self.regs.pc += o as u16 + 2;
                    }
                } else {
                    self.regs.pc += 2;
                }
                1
            }
            //LD h,r
            (0, 9, 6) => {
                let n = self.mem.load(addr + 1);
                self.regs.set_h(n);
                self.regs.pc += 2;
                0
            }
            // it try to init stack p
            // LD dd,nn
            (0,12,1) => {
                let n=self.mem.load(addr + 1);
                let n2= self.mem.load(addr + 2);
                let nn = get_u16_from_2_u8(n,n2);
                self.regs.set_sp(nn);
                self.regs.pc+=3;
                2
            }
            //LD nn, A
            (0,12,2) => {
                let n=self.mem.load(addr + 1);
                let n2= self.mem.load(addr + 2);
                let nn = get_u16_from_2_u8(n,n2);
                self.mem.store(nn, self.regs.a());
                self.regs.pc += 3;
                4
            }
            
            //LD A,n
            (0,15,6) => {
                let n = self.mem.load(addr + 1);
                self.regs.set_a(n);
                self.regs.pc += 2;
                2
            }
            
            //LD A,B
            (1, 14, 0) => {
                self.regs.set_a(self.regs.b());
                self.regs.pc += 1;
                1
            }
            //OR C
            (2, 12, 1) => {
                let x = self.regs.a() | self.regs.c();
                self.regs.set_a(x);
                if x == 0 {
                    self.regs.set_zero_flag()
                } else {
                    self.regs.clear_zero_flag();
                }
                self.regs.pc += 1;
                2
            }
            
            //JP <- nn
            (3, 0, 3) => {
                let n1 = self.mem.load(addr + 1);
                let n2 = self.mem.load(addr + 2);
                self.regs.pc = get_u16_from_2_u8(n1, n2);
                3
            }
             //PUSH BC
            (3,1,5) =>{
                let x = self.regs.bc();
                self.push(self.regs.bc());
                self.regs.pc += 1;
                3
            }
            //CALL
            (3,3,5) =>{
                let c_pc = self.regs.pc;

                let n=self.mem.load(addr + 1);
                let n2= self.mem.load(addr + 2);
                let nn = get_u16_from_2_u8(n,n2);
                self.push(c_pc);
                self.regs.set_wz(nn);
                self.regs.pc = nn;
                println!("call {:04X}",nn);
                17
            }
            //OUT nn A
            (3,4,3) =>{ 
                let read=self.mem.load(addr + 1);
                //self.out(addr as u16, self.regs.a());
                self.mem.bus_out(read, self.regs.a());
                self.regs.pc+=2;
                1
            }
            //PUSH HL
            (3,9,5) =>{
                let x = self.regs.hl();
                self.push(self.regs.hl());
                self.regs.pc += 1;
                3
            }
            (3, 12, 3) => {
                //TODO
                println!("Disable interupt");
                self.regs.pc += 1;
                1
            }
            
            _ => {
                println!("ERROR OP CODE :{:X} ({},{},{})",op , x, y, z);
                println!("ERROR NEXT CODE :{:X}", self.mem.load(addr + 1));
                
                panic!("OP ERROR 0x{:X}", op)
            },
        };
    }

    pub fn step(&mut self) {
        self.trace();

        let pc = self.regs.pc;
        self.decode_op(pc);
    }
}
