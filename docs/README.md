# Notes and journals

## Day 5 (Dec 4, 2022)
Learned something new today. For day 5's question, I have a list of structs, and one of the operations requires that two elements from that list are mutated in a single function call:

```rust
struct Stack {
    crates: VecDeque<char>,
}

impl Stack {
    fn move_crates(&mut self, to: &mut Stack, n: i32, preserve_ordering: bool) {
        let mut buffer: VecDeque<char> = VecDeque::new();

        for _ in 0..n {
            // safely assume that all operations are legal
            buffer.push_back(self.crates.pop_front().unwrap());
        }

        for _ in 0..n {
            if preserve_ordering {
                to.crates.push_front(buffer.pop_back().unwrap());
            } else {
                to.crates.push_front(buffer.pop_front().unwrap());
            }
        }
    }
}
```

On my first try, this is what I wrote

```rust
/// Generate and return the list of stacks
fn generate_stacks() -> Vec<Stack> {
    ...
}

fn solve() {
    let mut stacks = generate_stacks();

    ...
    let from = stacks.get_mut(from_i).unwrap();  // &mut Stack
    let to = stacks.get_mut(to_i).unwrap();  // &mut Stack
    from.move_crates(to, n, true);
}
```

However, this will not compile, since `from` and `to` are both mutable references to `stacks`, and using them in a single function call violates the borrow-checker's rule that only a single mutable reference exists at a time.

My second attempt is to use the interior mutability pattern:

```rust
/// Generate and return the list of stacks
fn generate_stacks() -> Vec<Stack> {
    ...
}

fn solve() {
    let stacks = Rc::new(RefCell::(generate_stacks()));
    let from_stacks = Rc::clone(&stacks);
    let to_stacks = Rc::clone(&stacks);

    ...
    let from = from_stacks.borrow_mut().get_mut(from_i).unwrap();
    let to = to_stacks.borrow_mut().get_mut(to_i).unwrap();
    from.move_crates(&mut *to, n, true);
}
```

However, this will panic for the same reason: both `from` and `to` hold `RefMut` to the list of `Stack`'s underlying `stacks`, only this time the borrow-checker rule was enforced at runtime, so the program will compile, but it will panic.

My third attempt was successful, this time we are not working with "list of structs", but "list of mutable references to structs":

```rust
/// Generate and return the list of stacks
fn generate_stacks() -> Vec<RefCell<Stack>> {
    ...
}

fn solve() {
    let stacks = generate_stacks();

    let from = stacks.get(from_i).unwrap();  // RefCell<Stack>
    let to = stacks.get(to_i).unwrap();  // RefCell<Stack>

    from.borrow_mut()  // MutRef<Stack>
        .move_crates(
            &mut *to.borrow_mut(),  // &mut Stack
            n
        );
}
```

Again I had to use this awkward contortionist "deferencing then borrowing pattern", where my function takes a mutable borrow `&mut Stack`, but my data is a mutable reference `RefCell<Stack>`, so I have to deference the mutable reference first, then borrow the deferenced value:

```rust
let to = stacks.get(to_i).unwrap();
let to = to.borrow_mut();  // MutRef<Stack>
let to = *to;  // Stack
let to = &mut to; // &mut STack
```

This doesn't seem elegant, but at least it works, I guess.

## Day 1 (Nov 30, 2022)
Because the puzzles were released at midnight on Eastern Time and I was living in the west coast, I got my hands on the puzzles at 9 pm on November 30. This was the first time I was participating in Advent of Code, or any other online coding events at all. While waiting for the puzzle to be released, I became surprisingly anxious, fearing that I would not perform as well as I expected, especially given that I chose to use Rust (instead of Python, which I am more familiar with), a language that I started learning at the end of September. Fortunately the first puzzle was logically trivial, and I was able to solve it in a few lines of code. In the process I learned to use `std::collections::BinaryHeap`, which was a fundamental data structure that "the book" did not cover.

While there was not much technical details to discuss, I wanted to write down some thoughts on doing Advent of Code. The truth is that after finishing reading "The Rust Programming Language," I was feeling lost about where to go next. It turned out that the language itself was not as hard to learn as many Youtube videos claimed, it was the kind of problems that Rust was used to solve that was really difficult. Compilers, operating systems, databases, security, networking, and other advanced topics all seemed to require a pretty big leap of faith that I simply didn't feel ready to make the jump across. Of course, sooner or later I will have make a jump, and there is simply no telling whether my first jump will land me in the place I want to go. As Steve Jobs said in the Stanford commencement speech:

> you cannot connect the dots forward; you can only connect them looking backward. So you have to trust that the dots will connect in your future.

Meanwhile, I will use Advent of Code as an opportunity to get more familiar with the Rust language. This means that my participation will be rather casual: I will only work on puzzles for 2 hours every day, and if I couldn't solve the puzzle for the day, I will just give up and come back to it later.