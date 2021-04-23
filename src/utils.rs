pub fn  get_u16_from_2_u8 (a:u8 ,b:u8) -> u16{
    (a as u16) | (b as u16) <<8
}

pub fn get_2_u8_from_u16(v:u16) -> (u8,u8){
   let a = (v & 0x00FF) as u8;
   let b = (v & 0xFF00) as u8;
   (a,b)
}