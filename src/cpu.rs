
use crate::mem::Mem;

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
}

impl<M: Mem> Mem for Cpu<M>{
    fn load(&self, addr: u16) -> u8 {
        self.mem.load(addr)
    }

    fn store(&mut self, addr: u16, val: u8) {
        self.mem.store(addr,val);
    }
}

impl <M: Mem> Cpu<M>{

    pub fn new(mem: M) -> Cpu<M> {
        Cpu {
            c: 0,
            regs: Regs::new(),
            mem: mem,
        }
    }
}