The first kind of loop is called `loop`. 

```rust
let mut x = 0;
loop {
    if x == 5 {
        break;
    }
    println!("x={}", x);
    x += 1;
}
```

- It is flexible: we can have break considition anywhere in the loop's body
- It can be used to implement infinite loops