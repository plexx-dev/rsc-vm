[![wakatime](https://wakatime.com/badge/user/034bfb19-0ab5-462c-93f5-fb78ecd01042/project/27affd8d-7819-4d2b-b83a-6fea6305a256.svg)](https://wakatime.com/badge/user/034bfb19-0ab5-462c-93f5-fb78ecd01042/project/27affd8d-7819-4d2b-b83a-6fea6305a256) [![Build & Test](https://github.com/plexx-dev/rsc-vm/actions/workflows/rust.yml/badge.svg)](https://github.com/plexx-dev/rsc-vm/actions/workflows/rust.yml)

# About

This is a VM to execute the binary file format `.rsbf` from the [RaychelScript](https://github.com/Weckyy702/RaychelScript) Scripting Language. The VM is designed to be as much light weight as possible to maximize performance.
This version currently only supports Version 4 of RaychelScript.

# Features

- Works with perfectly fine Files   ✔
- CLI Arguments   ✔

###

- Any Error Handling inside the execution Loop (to be implemented)
- Any Advanced File Reading Error Handling (to be implemented)
- In General any Real Error Handling when sth stupid happens inside the VM (to be implemented)

# Usage

First clone the GitHub repo 
```
git clone https://github.com/plexx-dev/rsc-vm
```

Then if you want change the given File Input in the 
```
src/main.rs
``` 

Then run the actual VM with
```
cargo run --release <YOUR INPUT ARGUMENTS SEPERATED WITH A WHITESPACE CHARACTER>
```

## Example:

```
cargo run --release 2 3
```

# Important

If you think you can expect any masterpiece or in general in any way a decently optimized Code here you're wrong. This is just to brag about my friend [Weckyy702's](https://github.com/weckyy702) that Rust is or can be faster than C++.

Somehow this is faster by a order of magnitude than the original Version of the RaychelScript VM which it's original author [Weckyy702's](https://github.com/weckyy702) wrote in C++.
Even though iam completly new to Rust and barley take advantage of Rust's full potential.

So i guess we all can assume Rust > C++.