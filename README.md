# select.rs [![Build Status](https://travis-ci.org/utkarshkukreti/select.rs.svg?branch=master)](https://travis-ci.org/utkarshkukreti/select.rs)

> A library to extract useful data from HTML documents, suitable for web scraping.

Note: All the API is currently unstable and will change as I use this library
more in real world projects. If you have any suggestions or feedback, please
open an issue or send me an email.

## Examples

### from `examples/stackoverflow.rs`

```rust
extern crate select;
use select::document::Document;
use select::predicate::*;

pub fn main() {
    // stackoverflow.html was fetched from
    // http://stackoverflow.com/questions/tagged/rust?sort=votes&pageSize=50 on
    // Aug 10, 2015.
    let document = Document::from_str(include_str!("stackoverflow.html"));

    println!("# Menu");
    for node in document.find(Attr("id", "hmenus")).find(Name("a")).iter() {
        println!("{} ({:?})", node.text(), node.attr("href").unwrap());
    }
    println!("");

    println!("# Top 5 Questions");
    for node in document.find(Class("question-summary")).iter().take(5) {
        let question = node.find(Class("question-hyperlink")).first().unwrap();
        let votes = node.find(Class("vote-count-post")).first().unwrap().text();
        let answers = node.find(Class("status")).find(Name("strong")).first().unwrap().text();
        let tags = node.find(Class("post-tag")).iter().map(|tag| tag.text()).collect::<Vec<_>>();
        let asked_on = node.find(Class("relativetime")).first().unwrap().text();
        let asker = node.find(Class("user-details")).find(Name("a")).first().unwrap().text();
        println!(" Question: {}", question.text());
        println!("  Answers: {}", answers);
        println!("    Votes: {}", votes);
        println!("   Tagged: {}", tags.join(", "));
        println!(" Asked on: {}", asked_on);
        println!("    Asker: {}", asker);
        println!("Permalink: http://stackoverflow.com{}", question.attr("href").unwrap());
        println!("");
    }

    println!("# Top 10 Related Tags");
    for node in document.find(Attr("id", "h-related-tags")).parent().find(Name("div")).iter().take(10) {
        let tag = node.find(Name("a")).first().unwrap().text();
        let count = node.find(Class("item-multiplier-count")).first().unwrap().text();
        println!("{} ({})", tag, count);
    }
}
```

prints

```
# Menu
Questions ("/questions")
Tags ("/tags")
Users ("/users")
Badges ("/help/badges")
Unanswered ("/unanswered")
Ask Question ("/questions/ask")

# Top 5 Questions
 Question: Applications and libraries written in Rust [closed]
  Answers: 8
    Votes: 67
   Tagged: rust
 Asked on: Feb 19 '12 at 14:39
    Asker: Atom
Permalink: http://stackoverflow.com/questions/9350125/applications-and-libraries-written-in-rust

 Question: How to debug Rust programs? [closed]
  Answers: 6
    Votes: 52
   Tagged: rust
 Asked on: Apr 8 '13 at 5:30
    Asker: macropas
Permalink: http://stackoverflow.com/questions/15871885/how-to-debug-rust-programs

 Question: How to access command line parameters?
  Answers: 9
    Votes: 51
   Tagged: rust
 Asked on: Mar 25 '13 at 15:59
    Asker: shutefan
Permalink: http://stackoverflow.com/questions/15619320/how-to-access-command-line-parameters

 Question: Why are explicit lifetimes needed in Rust?
  Answers: 6
    Votes: 48
   Tagged: pointers, rust, static-analysis, lifetime
 Asked on: Jul 24 at 11:15
    Asker: jco
Permalink: http://stackoverflow.com/questions/31609137/why-are-explicit-lifetimes-needed-in-rust

 Question: What is the difference between traits in Rust and typeclasses in Haskell?
  Answers: 1
    Votes: 46
   Tagged: haskell, rust
 Asked on: Jan 24 at 7:50
    Asker: LogicChains
Permalink: http://stackoverflow.com/questions/28123453/what-is-the-difference-between-traits-in-rust-and-typeclasses-in-haskell

# Top 10 Related Tags
lifetime (165)
traits (83)
rust-cargo (79)
string (76)
ffi (62)
iterator (58)
multithreading (50)
generics (50)
arrays (49)
borrow-checker (47)
```

## License

MIT
