use crate::dns::question;

pub struct Name {
    length: usize,
    content: Vec<u8>,

}
pub struct DnsQuestion {
    pub names: Vec<Name>,
    pub the_type: u16,
    pub class: u16
}

impl DnsQuestion {
      pub fn parse(data: &[u8]) -> (Self, usize, Vec<u8>) {
        let mut names: Vec<Name> = Vec::new();
        let mut pos = 12;
        let question_start = pos;
        loop {
            let length =data[pos] as usize; 

            if length == 0 {
                pos += 1; // skip null terminator
                break
            }
            let content = &data[pos + 1 .. pos + 1 + length];
            names.push(Name { length: length, content: content.to_vec() });
            pos += length + 1;
        }

           let the_type =  u16::from_be_bytes([data[pos], data[pos + 1]]);
           pos += 2;
           let class = u16::from_be_bytes([data[pos], data[pos + 1]]);
       

           // raw question slice
           let raw_quesiton = data[question_start .. pos].to_vec();

    (  Self { names , the_type, class }, pos, raw_quesiton)
    }
}