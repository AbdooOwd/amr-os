# Devlog

You could also call this a "Diary" for this OS project.

> Note: date format = (DD/MM/YYYY)

## [26/12/2025]

### Some info for OS start in Rust

> These were comments for `src/main.rs` at `19:10PM`

When a Rust program starts, it uses the C-runtime (crt0)
to prepare some stuff like the stack or idk
and then looks for a `start` function
which is like: "Alright, Rust, I finished my runtime
work. It's your turn to do whatever you're gonna do".
Then Rust, by its turn,will look for 
a `main` function--the usual function we start our 
rust code in!

`no_mangle` means we don't wanna generate some unique
function name for the `_start` function.

`extern "C"` specifies the use of the C calling convention
(there is some thing VERY similar in C++).

### Returning from the entry func?

The `!` return type means a function from which we should
NEVER return. Like, see it as just "entering the function"
rather than "calling the function" (like a `goto` in C?).

Instead of returning, the end of the function
is "what happens when the OS stops?".
Logically: power-off, maybe.

### I should try to understand...!

I've already tried making an OS before--3 or 4 times, as a matter of fact.
But I'm a different developer now! And before,
I used to either yoink ctrl+c ctrl+v copy-paste some random stuff
I found in tutorials, forums and anything else
then expected it to work.

It did sometimes work, but I think it kind of ruins
the purpose of the project; getting our hands dirty! Learn!

So, this time, I will try to understand as much as possible of what
I am reading and learning!
And the first step is searching for **every single thing I didn't understand from
the Rust OS blog I'm following:
[Writing an OS in Rust from Philipp Oppermann's blog](https://os.phil-opp.com)**

### Our target is bare-metal! (i guess)

When running `cargo build`, even with our sweet `#![no_std]`,
cargo thinks we're building for our platfrom/target.
In my case: `x86_64-unknown-linux-gnu`. The architecture is `x86_64`,
`linux` is the system, the vendor is unknown, and the ABI is `gnu`.
But our OS... can't compile on top of... an OS? Like,
making an OS--an independent component of the computer--that relies on
Linux? Another OS? It's like saying crafting wood out of wood.
How would it be an OS? The core/brain of our system!
In other terms: We want our OS' compilation (I think that's a correct way to put it) to be independent from our environment (the OS we're using).
In my case, `cargo` will try building to Linux in the x86_64.
(again: running an OS from an OS?)

So instead, we'll use a target that doesn't use or depend on any OS,
which is the one proposed by Philipp Oppermann in his RustOS blog:
`thumbv7em-none-eabihf`. I don't know what this is, but I know two thing:
I should search about it, and it just doesn't have an underlying OS.

### Rust-Analyzer shall be happy!

Because of a setting for RustAnalyzer (`cargo.allTargets`), Cargo
thinks the "package" (our project) *could* be a library OR a binary
(a "package" target?).
And the real reason RustAnalyzer wasn't too happy for some code I wrote
in a `no_std` environment is that it thinks I will use benching and testing
--thus we disable them in `Cargo.toml`.

### Huh? Custom target?

So, we were using that other target that didn't depend on an OS.
But, we would like to be able to specify how this target behaves
or give its specifications.
Thus, I ~~copy pasted from os.phil-opp.com~~ wrote the specifications
of an average x86_64 OS (i think) and just called it
`x86_64-amr_os.json`.

I have also switched my Rust compiler's build to `nightly` to access some
features we require, like building some `std` stuff like `core` for our
new target. A new file, `.cargo/config.toml`, was required to configure
that std-recompilation stuff.

We need to recompile `core` and its dependencies from `compiler_builtins`
in order to use the essentials of Rust (the `Result` type, `PanicHandler`...).
We also enable some feature called `compiler-builtins-mem` which enable
some memory functions that should be included in the C-runtime by default,
like `memset`, `memcpy`, `memcmp`.
They are already provided by `compiler_builtins`, but disabled by default
to avoid any conflict with the C-runtime (not our case).
All of this because Rust assumes it can use these memory functions,
plus some features use them.
We **could** code our own memory functions and stick a `#[unsafe(no_manlge)]`,
but this isn't very safe, nor smart. Like, imagine using something
that uses a memory function while we're trying to write a memory function.

Finally, instead of doing `cargo build --target specs/x86_64-amr_os.json`
every time I want to use my target, which I'll use, like, <u>**always**</u>;
we'll set it as our default target.
