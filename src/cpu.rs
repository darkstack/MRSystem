
// use crate::mem::Mem;
// use crate::utils::*;
// #[derive(Debug)]
// struct Regs {
//     //Acumulator
//     a: u8,
//     b: u8,
//     d: u8,
//     h: u8,
//     //flag
//     f: u8,
//     c: u8,
//     e: u8,
//     l: u8,
//     //memory refresh
//     r: u8,
//     //interrupt vector
//     i : u8,
//     ix: u16,
//     //index register
//     iy: u16,
//     //stack pointer
//     sp: u16,
//     //program counter
//     wz: u16,
//     pc: u16,
    
// }

// enum Register {
//     //Acumulator
//     a,
//     b,
//     d,
//     h,
//     f,
//     c,
//     e,
//     l,
//     r,
//     i,
//     ix,
//     iy,
//     sp,
//     wz,
//     pc,
    
// }

// impl Regs {
//     fn new() -> Regs {
//         Regs {
//         a:0,
//         b:0,
//         d:0,
//         h:0,
//           //flag
//         f: 0,
//         c: 0,
//         e: 0,
//         l: 0,
//         //memory refresh
//         r: 0,
//         //interrupt vector
//         i : 0,
//         ix: 0,
//         //index register
//         iy: 0,
//         //stack pointer
//         sp: 0,
//         //program counter
//         wz: 0,
//         pc: 0,
//         }
//     }

//     fn set_carry_flag(&mut self){
//         self.f |= 1<<0;
//     }
//     fn get_carry_flag(&mut self) -> bool{
//         self.f & 1<<0 == 0
//     }
//     fn clear_carry_flag(&mut self)
//     {
//         self.f &= !(1 << 0)
//     }
//     fn set_zero_flag(&mut self){
//         self.f |= 1<<6;
//     }
//     fn get_zero_flag(&mut self) -> bool{
//         self.f & 1<<6 == 0
//     }
//     fn clear_zero_flag(&mut self)
//     {
//         self.f &= !(1 << 6)
//     }


// }
// pub struct Cpu<M: Mem> {
//     pub c: u64,
//     regs: Box<Regs>,
//     pub mem: M,
//     pub debug: bool
// }

// impl<M: Mem> Mem for Cpu<M>{
//     fn load(&mut self, addr: u16) -> u8 {
//         self.mem.load(addr)
//     }

//     fn store(&mut self, addr: u16, val: u8) {
//         self.mem.store(addr,val);
//     }
// }

// impl <M: Mem> Cpu<M>{

//     pub fn new(mem: M,debug :bool) -> Cpu<M> {
//         Cpu {
//             c: 0,
//             regs: Box::new(Regs::new()),
//             mem: mem,
//             debug: debug,
//         }
//     }

//     pub fn trace(&mut self){
//         if self.debug{
//             println!("PC : {:x} {:?}",self.regs.pc,self.regs)
//         }
            
//     }
//     fn instruction_jp_cc_nn(&mut self,cc:bool,addr:u16){
//         if cc {
//             self.regs.pc = addr;
//         }
//         else{
//             self.regs.pc += 3;
//         }
//     }
//     fn instruction_jr_cc_v(&mut self,cc:bool,v:i8){
//         if cc {
//             if v.is_negative(){
//                 let abs = v.wrapping_abs() as u16;
//                 self.regs.pc -= abs-2 //we fetched the next operand first so +2 :S
//             }
//             else{
//                 self.regs.pc +=  v as u16 +2;
//             }
//         }
//         else{
//             self.regs.pc += 2;
//         }
//     }
//     fn instruction_or_a_r(&mut self,r:u8){
//         let x =self.regs.a | r;
//         self.regs.clear_carry_flag();
//         if x==0 {
//             self.regs.set_zero_flag()
//         } else{
//             self.regs.clear_zero_flag();
//         }
//         self.regs.a = x;
//         self.regs.pc += 1
//     }

//     //THIS IS SO UGLY
//     fn instruction_dec_r(&mut self,r:Register) {
//         let mut c=0u8;
//         match r {
//             Register::a => {self.regs.a -= 1; c = self.regs.a }
//             Register::b => {self.regs.b -= 1; c = self.regs.b }
//             Register::d => {self.regs.c -= 1; c = self.regs.c }
//             Register::h => {self.regs.h -= 1; c = self.regs.h }
//             Register::f => {self.regs.f -= 1; c = self.regs.f }
//             Register::c => {self.regs.c -= 1; c = self.regs.c }
//             Register::e => {self.regs.e -= 1; c = self.regs.e }
//             Register::l => {self.regs.l -= 1; c = self.regs.l }
//             Register::r => {self.regs.r -= 1; c = self.regs.r }
//             Register::i => {self.regs.i -= 1; c = self.regs.i }
//             // Register::ix => {}
//             // Register::iy => {}
//             _ => panic!("WAT?")
//         };
//         self.regs.clear_carry_flag();
//         if c==0 {
//             self.regs.set_zero_flag()
//         } else{
//             self.regs.clear_zero_flag();
//         }
//         self.regs.pc += 1;
//     }

  
//     fn decode_op(&mut self,addr :u16) {
//         let op = self.mem.load(addr);
        
//         match op {
//             //
//             0x01 =>{
//                 let n = self.load(addr+1);
//                 let n2 = self.load(addr+2);
//                 self.regs.b =n2;
//                 self.regs.c =n;
//                 self.regs.pc+=3;
//             }
//             //dec bc
//             0x0B => 
//             {
//                 let mut bc = get_u16_from_2_u8(self.regs.b,self.regs.c);
//                 bc -= 1;
//                 let (b,c) = get_2_u8_from_u16(bc);
//                 self.regs.b = b;
//                 self.regs.c = c;
//                 self.regs.pc+=1;
//             }
//             //JR NZ
//             0x20 =>{
                
//                 let o = self.load(addr+1) as i8;
//                 let nz = self.regs.get_zero_flag();
//                 self.instruction_jr_cc_v(nz,o);
//             }
//             //DEC H
//             0x25 => {
//                 self.instruction_dec_r(Register::h);
//             }
//             //LD H,v
//             0x26 => {
//                 let n = self.load(addr+1);
//                 self.regs.h =n;
//                 self.regs.pc+=2;
//             }
//             //LD A,B
//             0x78 =>{
               
//                 self.regs.a = self.regs.b;
//                 self.regs.pc+=1;
               
//             }
//             0xB0=>{
               
//                 self.instruction_or_a_r(self.regs.b);
               
//             }
//             0xB1=>{
//                 self.instruction_or_a_r(self.regs.c)
//             }       
//             0xB2=>{
//                 self.instruction_or_a_r(self.regs.d)
//             }
//             0xB3=>{
//                 self.instruction_or_a_r(self.regs.e)
//             }
//             // 0xB4=>{
//             //     self.instruction_or_a_r(self.regs.h)
//             // }
//             // 0xB5=>{
//             //     self.instruction_or_a_r(self.regs.l)
//             // }
//             // 0xB6=>{
//             //     self.instruction_or_a_r(self.regs.hl)
//             // }
//             0xB7=>{
//                 self.instruction_or_a_r(self.regs.a)
//             }

//             //JP
//             0xC3 => {
//                 let next_pc = self.load(addr+1) as u16 | (self.load(addr+2)as u16)<<8;
//                 self.regs.pc = next_pc;
//             },
//             //DI
//             0xF3=> {
//                 //TODO
//                     println!("Disable interupt");
//                     self.regs.pc= self.regs.pc+1
//             }
            
//             _ => panic!("unknow opcode {:X}",op)   
//         }
//     }

//     pub fn step(&mut self){
//         self.trace();
        
//         let pc = self.regs.pc;
//         self.decode_op(pc);
//     }
// }