pub struct PrimPr {
    pub title: String,
    pub url: String,
    pub author: String,
    pub comments: u8,
    pub reviews: u8,
    pub age: String,
    pub mergeable: bool,
    pub draft: bool,
    pub bot: bool,
}

impl std::fmt::Display for PrimPr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "**{}**", self.title)?;
        writeln!(f, "{}", self.url)?;
        writeln!(
            f,
            "{}, {}",
            plural(self.reviews, "review"),
            plural(self.comments, "comment")
        )?;
        writeln!(f, "Created by {} {}", self.author, self.age)?;
        Ok(())
    }
}

pub struct PrimPrList(pub Vec<PrimPr>);

impl std::fmt::Display for PrimPrList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        println!("PRs:");
        println!("");
        for pr in self.0.iter() {
            if pr.draft == false && pr.bot == false {
                writeln!(f, "{}", pr)?;
            }
        }
        println!("Bot PRs:");
        println!("");
        for pr in self.0.iter() {
            if pr.bot == true {
                writeln!(f, "{}", pr)?;
            }
        }

        println!("Draft Prs:");
        println!("");
        for pr in self.0.iter() {
            if pr.draft == true {
                writeln!(f, "{}", pr)?;
            }
        }
        Ok(())
    }
}

fn plural(n: u8, s: &str) -> String {
    if n == 1 {
        return format!("{} {}", n, s);
    }
    return format!("{} {}s", n, s);
}
