
use std::default::Default;
use std::str::FromStr;
use std::string::ToString;

mod tag;
pub use self::tag::Tag;


#[derive(Debug)]
pub struct Playlist {
    tags: Vec<Tag>
}

impl Default for Playlist {
    fn default() -> Playlist {
        let mut tags: Vec<Tag> = Vec::new();
        tags.push(Tag::M3U);
        tags.push(Tag::VERSION(3));
        Playlist {tags: tags}
    }
}

impl ToString for Playlist {
    fn to_string(&self) -> String {
        self.tags.iter().map(|tag: &Tag| -> String {
            tag.to_string() + "\n"
        }).collect::<Vec<String>>().join("\n")
    }
}

impl FromStr for Playlist {
    type Err = ();
    fn from_str(s: &str) -> Result<Playlist, ()> {
        let mut lines: Vec<&str> = s.split("#").collect();
        lines.remove(0);
        let mut tags: Vec<Tag> = Vec::new();
        for line in lines.iter(){
            match Tag::from_str(line.trim()) {
                Ok(tag) => tags.push(tag),
                Err(e)  => println!("[Playlist] Parse fail ({:?})", e)
            };
        }
        // TODO: verify tags
        if tags.len() == 0 {
            Err(())
        } else {
            Ok(Playlist{tags: tags})
        }
    }
}

impl Playlist {
    #[allow(unused_variables)]
    pub fn new (duration: usize) -> Playlist {
        let tags: Vec<Tag> = Vec::new();
        Playlist{tags: tags}
    }
    pub fn append(&self) -> bool {
        true
    }
    pub fn tags(&self) -> &[Tag] {
        self.tags.as_slice()
    }
}
