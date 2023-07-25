use std::borrow::Cow;
pub struct Peer{
    pub username: String,
    pub ip_addr: String,
    pub port: u16,
}

//peer implements Into<Cow<'_, str>> so that it can be used as a key in a HashMap
impl Peer{
    pub fn new(username: String, ip_addr: String, port: u16) -> Peer{
        Peer{
            username,
            ip_addr,
            port,
        }
    }

    pub fn split_at(&self, index: usize) -> (&Peer, &Peer){
        let mut first = self.clone();
        let mut second = self.clone();
        first.username = self.username[..index].to_string();
        second.username = self.username[index..].to_string();
        (first,second)
    }
}