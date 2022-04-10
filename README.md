# shape_chesss_in_rust
Yet another shape chess game in Rust.

# Why the implementation is so slow?

The main reason is performance of Vector iteration is very poor.

Consider the following code:

let tail = &play_tree[play_tree.len()-1];

for t in tail { }

The nearly do-nothing code which only takes Vector iteration consumes 69 seconds to finish for a ~700000 Vector.
