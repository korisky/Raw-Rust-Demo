## Why using reference would not pass the ownership in Rust

The image below showing how reference works

- without passing the ownership, we need to pass refernce
- when we pass s1's reference to s, we actually make it point to s1
- s1 is a pointer, the real value is still in the heap

![show_image](https://doc.rust-lang.org/book/img/trpl04-05.svg)