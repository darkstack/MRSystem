use crate::mem::Mem;
use crate::utils::*;

struct Register{ }

impl Register{

    pub const A :usize = 0;
    pub const B :usize =1;
    pub const D :usize =2;
    pub const H :usize =3;
    pub const F :usize =4;
    pub const C :usize =5;
    pub const E :usize =6;
    pub const L :usize =7;
    //I m not sure ebout these one
    //pub const R :usize =8;
    //pub const I :usize =9;
    pub const IX_L :usize =10;
    pub const IX_H :usize =11;
    pub const IY_L :usize =12;
    pub const IY_H :usize =13;
    pub const SP_L :usize =14;
    pub const SP_H :usize =15;
    pub const WZ_H :usize =16;
    pub const WZ_L :usize =17;
}
const TOTAL_REGISTER : usize = 18;

#[derive(Debug)]
pub struct Registers {
    //GENERAL REG
    reg : [u8;TOTAL_REGISTER],
    pc : u16,
}

impl Registers{
    pub fn new() -> Registers {
        Registers{
            reg: [0;TOTAL_REGISTER],
            pc: 0,
        }
    }

    pub fn a(&self) -> u8 {
        self.reg[Register::A]
    }
    pub fn set_a(&mut self, v : u8) {
        self.reg[Register::A] = v;
    }

    pub fn b(&self) -> u8 {
        self.reg[Register::B]
    }
    pub fn set_b(&mut self, v : u8) {
        self.reg[Register::B] = v;
    }

    pub fn d(&self) -> u8 {
        self.reg[Register::D]
    }
    pub fn set_d(&mut self, v : u8) {
        self.reg[Register::D] = v;
    }

    pub fn h(&self) -> u8 {
        self.reg[Register::H]
    }
    pub fn set_h(&mut self, v : u8) {
        self.reg[Register::H] = v;
    }

    pub fn f(&self) -> u8 {
        self.reg[Register::F]
    }
    pub fn set_f(&mut self, v : u8) {
        self.reg[Register::F] = v;
    }

    pub fn c(&self) -> u8 {
        self.reg[Register::C]
    }
    pub fn set_c(&mut self, v : u8) {
        self.reg[Register::C] = v;
    }

    pub fn e(&self) -> u8 {
        self.reg[Register::E]
    }
    pub fn set_e(&mut self, v : u8) {
        self.reg[Register::E] = v;
    }

    pub fn l(&self) -> u8 {
        self.reg[Register::L]
    }
    pub fn set_l(&mut self, v : u8) {
        self.reg[Register::L] = v;
    }
    
    pub fn bc(&self)-> u16 {
        //COULD u16 from index
        get_u16_from_2_u8(self.reg[Register::B],self.reg[Register::C])
    }
    pub fn set_bc(&mut self,v:u16){
        //COULD u16 from index
        let (b,c) = get_2_u8_from_u16(v);
        self.reg[Register::B] = b;
        self.reg[Register::C] = c;
    }








    //FLAG
    fn set_carry_flag(&mut self){
        self.reg[Register::F] |= 1<<0;
    }
    fn get_carry_flag(&mut self) -> bool{
        self.reg[Register::F] & 1<<0 == 0
    }
    fn clear_carry_flag(&mut self)
    {
        self.reg[Register::F] &= !(1 << 0)
    }
    fn set_zero_flag(&mut self){
        self.reg[Register::F] |= 1<<6;
    }
    fn get_zero_flag(&mut self) -> bool{
        self.reg[Register::F] & 1<<6 == 0
    }
    fn clear_zero_flag(&mut self)
    {
        self.reg[Register::F] &= !(1 << 6)
    }
}


pub struct Cpu2<M: Mem> {
    pub c: u64,
    regs: Registers,
    pub mem: M,
    pub debug: bool,
    pub cycles: u128,
}

impl <M: Mem> Cpu2<M>{

    pub fn new(mem: M,debug :bool) -> Cpu2<M> {
        Cpu2 {
            c: 0,
            regs: Registers::new(),
            mem: mem,
            debug: debug,
            cycles: 0,
        }
    }

    pub fn trace(&mut self){
        if self.debug{
            println!("PC : {:x} {:?}",self.regs.pc,self.regs)
        }
            
    }

    fn decode_op(&mut self,addr :u16) {
        let op = self.mem.load(addr);
        println!("Decode op : 0x{:X}",op);
        //Z80 seems to be split in multiple type

        let (x,y,z) = (op >> 6 , (op>> 2 & 0x0F) , op & 0b111);
        println!("OPERAND : {:08b}",op);
        println!("x:{:b} y:{:b} z:{:b}",x,y,z);
        println!("x:{} y:{} z:{}",x,y,z);

        let cycle = match (x,y,z){
            //LD BC
            (0,0,1) =>{
                let n1 = self.mem.load(addr+1);
                let n2 = self.mem.load(addr+2);
                self.regs.set_bc(get_u16_from_2_u8(n2, n1));
                self.regs.pc+=3;
                0
            }
            //DEC BC
            (0,2,3) =>{
                self.regs.set_bc(self.regs.bc()-1);
                self.regs.pc+=1;
                1
            }
            //DEC H 
            (0 ,9 ,5) =>{
                self.regs.set_h(self.regs.h()-1);
                self.regs.pc+=1;
                1
            }
            // JR NZ 
            (0,8,0)=>{
                //Should implement CC 
                //HOW MUCH
                let o = self.mem.load(addr+1) as i8;
                let nz = self.regs.get_zero_flag();
                if nz {
                    if o.is_negative(){
                        let abs = o.wrapping_abs() as u16;
                        self.regs.pc -= abs-2 //we fetched the next operand first so +2 :S
                    }
                    else{
                        self.regs.pc +=  o as u16 +2;
                    }
                }
                else{
                    self.regs.pc += 2;
                }
                1
            }
            //LD h,r
            (0,9,6) =>{
                let n = self.mem.load(addr+1);
                self.regs.set_h(n);
                self.regs.pc+=2;
                0
            }
            //LD A,B
            (1,14,0)=>{
                self.regs.set_a(self.regs.b());
                self.regs.pc+=1;
                1
            }
            //OR C
            (2,12,1) => {
                let x =self.regs.a() | self.regs.c();
                self.regs.set_a(x);
                if x==0 {
                    self.regs.set_zero_flag()
                } else{
                    self.regs.clear_zero_flag();
                }
                self.regs.pc += 1;
                2
            }
            //JP <- nn
            (3,0,3) =>{
                let n1 = self.mem.load(addr+1);
                let n2 = self.mem.load(addr+2);
                self.regs.pc = get_u16_from_2_u8(n1,n2);
                3
            }
            (3,12,3)=> {
                //TODO
                    println!("Disable interupt");
                    self.regs.pc+=1;
                    1
            }
            _ => panic!("OP ERROR 0x{:X}",op),
        };
         
    }

    pub fn step(&mut self){
        self.trace();
        
        let pc = self.regs.pc;
        self.decode_op(pc);
    }
}