
use crate::mem::Mem;
use crate::utils::*;
#[derive(Debug)]
struct Regs {
    //Acumulator
    a: u8,
    b: u8,
    d: u8,
    h: u8,
    //flag
    f: u8,
    c: u8,
    e: u8,
    l: u8,
    //memory refresh
    r: u8,
    //interrupt vector
    i : u8,
    ix: u16,
    //index register
    iy: u16,
    //stack pointer
    sp: u16,
    //program counter
    pc: u16,
    
}

impl Regs {
    fn new() -> Regs {
        Regs {
        a:0,
        b:0,
        d:0,
        h:0,
          //flag
        f: 0,
        c: 0,
        e: 0,
        l: 0,
        //memory refresh
        r: 0,
        //interrupt vector
        i : 0,
        ix: 0,
        //index register
        iy: 0,
        //stack pointer
        sp: 0,
        //program counter
        pc: 0,
        }
    }

}
pub struct Cpu<M: Mem> {
    pub c: u64,
    regs: Regs,
    pub mem: M,
    pub debug: bool
}

impl<M: Mem> Mem for Cpu<M>{
    fn load(&mut self, addr: u16) -> u8 {
        self.mem.load(addr)
    }

    fn store(&mut self, addr: u16, val: u8) {
        self.mem.store(addr,val);
    }
}

impl <M: Mem> Cpu<M>{

    pub fn new(mem: M,debug :bool) -> Cpu<M> {
        Cpu {
            c: 0,
            regs: Regs::new(),
            mem: mem,
            debug: debug,
        }
    }

    pub fn trace(&mut self){
        if self.debug{
            println!("PC : {:x} {:?}",self.regs.pc,self.regs)
        }
            
    }
       

    fn decode_op(&mut self,addr :u16) {
        let op = self.mem.load(addr);
        match op {
            
            0x01 =>{
                let n = self.load(addr+1);
                let n2 = self.load(addr+2);
                self.regs.b =n;
                self.regs.c =n2;
                self.regs.pc+=3;
            }
            //dec bc
            0x0B => {
                let (b,c) = get_2_u8_from_u16(get_u16_from_2_u8(self.regs.b,self.regs.c) - 1);
                self.regs.b = b;
                self.regs.c = b;
                self.regs.pc+=1;
            }
            //LD H,v
            0x26 => {
                let n = self.load(addr+1);
                self.regs.h =n;
                self.regs.pc+=2;
            }
            //JP
            0xC3 => {
                let next_pc = self.load(addr+1) as u16 | (self.load(addr+2)as u16)<<8;
                self.regs.pc = next_pc;
            },
            //DI
            0xF3=> {
                //TODO
                    println!("Disable interupt");
                    self.regs.pc= self.regs.pc+1
            }
            
            _ => panic!("unknow opcode {:x}",op)   
        }
    }

    pub fn step(&mut self){
        self.trace();
        
        let pc = self.regs.pc;
        self.decode_op(pc);
    }
}