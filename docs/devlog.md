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
In other terms: We want our OS' compilation (I think that's a correct way to put it)
to be independent from our environment (the OS we're using).
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

### Am I really learning?

Because of all the dog-water of Rust being too strict (which is good,
we just don't want to admit it), I feel like every part of making an OS
with that language is 13% of implementation and 102% of trying to satisfy
Rust.
Yes; satisfying Rust means satisfying the computer with *safety*.
But **ARGH**!

*\*thinks\**

As a matter of fact, maybe it's just me who's not good enough at Rust yet (fact).
But I gotta express that our guy Philipp uses way, WAY too much crates.
It's a few, yes. But for me & my goals it feels like a lot, and it's pretty annoying.
I do know that if I try implementing even the simplest thing in Rust by myself
I would break something. Thus using a crate is just a mean to an end which is:
making an Operating System.

...

Why did I want to use Rust again? Ah yeah, right: the safety and the forced "Clean Code".
Well, I'm the genius who decided to start an <u>**Operating System**</u> in **Rust**--a language
I do not master or even am proficient/efficient with.

And all that talk from the OSDev Wiki just saying:
"You know it's pretty hard"
"If you're not ready don't start such a project"
"Actually, your OS-project is stoopid".

There are a some stuff said on the wiki that feels pretty honest: the fact that
making an OS is no easy task--and I did know that, and I just told myself
"meh who cares I'll improve in the process".

But on this same wiki, there are grains of... gate-keeping.
It sometimes feel like some OSDev folks just don't want you to touch OSDev.
- I'm gonna break something? Well, as long as I don't damage any computer: who care!?
- I'm gonna write the worst code ever written by someone? As long as the people working
  with you are fine or you work alone: who cares?!

I'm not planning on making the next Linux or Windows!

### ALRIGHT

Alright, it's like, the 27th of December. It's 2AM. I guess I'll continue
to look into this OSDev stuff some other time.

So far, my tasks are:
- Finding the Rust skills I lack.
- Learn more about OSDev and the beginning of an OS' start
  so I don't rely too much on the Phil-Opp blog.
- Find out if it's even worth it writing this OS in Rust!
  * Especially because of how essential crates look in this project
    (I refuse to! Maybe, I'll use and give up to those crates.
    But I want my own software written!).
- Maybe finding another kind of project to work on resembling this kind.
- List out the OS' stuff I must work on.
- Find a way to convert any OS
  knowledge I gain online into code--not relying on some already written
  code with some explanation then calling it a day saying "Alright I get it"
  when I don't.
- Prioritize understanding & explanation!

... I already feel lazy to continue this project...
I HAVEN'T EVEN IMPLEMENTED A WAY TO PRINT TEXT YET! I HAVEN'T EVEN
WRITTEN A "Hello, World!" YET! AND I AM ALREADY FEELING LIKE:
"this is annoying and boring"!

> I literally had to wait a week--or 7 days to work on this project.
  I was so excited to **START** working on it!

### (literally 4 minutes later when I was finally going to bed)

You know, I just re-read a few bits of the Phil-Opp blog and...
It really, REALLY feels like Rust is living in the shadow of C
in matters of OSDev!
Like: "we extern this to imitate C", "we use C calling convention",
"Rust relays on the C-runtime", "We add `repr(C)` to imitate a C struc-"
**AAAAAARRRRRRGHHHHHHHH!!!!!!! STOP! ENOUGH!**

**YES! YES RUST ISN'T AS MATURE AS C! YES C IS LITERALLY THE FOUNDATION
OF COMPUTERS, SOFTWARES N' ALL THAT!**

**BUT ARGH!<br/>ARGH!**<br/>
**"USE THOSE CRATES", "DON'T WRITE THERE", "ACTUALLY, LET's ASK
BABA C"**


## [27/12/2025]

### Dear me: rely on yourself!

So, I have tried compiling my OS to print some text to the screen by writing
to the VGA text buffer at `0xb8000`. **BUT** (there is always a but):
the blog I'm following recommends using the `bootloader` package/crate/idk,
and install `bootimage` for cargo to be able to compile our OS by sticking a bootloader
to our compiled kernel.

Now: `bootloader` definitely relies on `std`. Because everytime I try compiling
I have these, sweet, error messages:
```
error: could not compile `serde_core` (lib) due to 5829 previous errors
```

Excuse me? 64 packages? Nuh uh. Our dear `serde_core` doesn't want to let us
compile peacefully. What is it? It's some kind of JSON library.
It's probably used to read some config, maybe.
But for now my issue is sticking a bootloader to my kernel.

Now that I think about it... I *could* just write something simple in Assembly, but:
1.  Writing a bootloader can be a pain in the neck.
2.  I will have to code from scratch the transition from 16-bit real-mode to 32-bit
    protected-mode, THEN try to enter 64-bit long-mode.

At the same time, having your kernel rely on code **YOU** wrote is: ***ahhhh***.

Our other solution would be looking for a way to tinker with an already existing
bootloader and try linking it to our kernel. I think the second option option could
be a win for me. Maybe I could just look into those `bootimage` and `bootloader`
and try to find a way to satisfy them and their dependencies.

### Let's break things down

The best way to understand a problem is understanding what we're really trying to do
then trying to understand what we have coded. This concept, if you have a rubber-duck
on your desk, is called: *"Rubber Ducking"*.
It means talking about almost anything that's related to our current problem or code
in order to achieve some goal(s) like understanding the codebase better and tracking down
a bug or problem.

So let's do something similar: **what are we doing?**

First of all, we have abandoned the `main` function and `std`--Rust's standard library.
Because Rust relies on the C-runtime-zero (crt0), our usual user-space Rust programs
are started by the C-runtime which initializes some stuff like the stack and all,
then starts Rust's runtime stuff (which is minimal) like backtrace on panic,
stack overflow guards...etc.
C starts the Rust part by calling a function called `start`.
We have implemented this function and called it `_start`.
We made sure it's name wasn't changed or replaced by a randomly generated
unique one by using `#[unsafe(no_mangle)]`.
We then made this function use the C calling convention (some stuff to be able
to call a function from other languages and binaries, I think. Like some kind
tunnel between languages in terms of functions through its names or idk).

That `_start` function has the return type of `!` which means the function
should never be returned from. See the C-runtime calling `start` as a `goto`--just
entering the function, not expecting anything in return.

> It's definitely different from the `()` type in Rust. I should check about that.

Rust requires a "panic" function which should be run when, you guessed it: panicking.
That's why we replace Rust's default panic handler with ours.
At the time I'm writing this, my panic handler doesn't do anything. It just serves
as code Rust should go to when something goes wrong.
When I'll be able to write some text to the screen, I'll be able to output the error
or backtrace which would be much more appropriate for a panic handler.

We have made modifications to our Rust code to specify how we are coding.
But `cargo` doesn't know anything about us coding an OS! Meaning it doesn't know
we are in a bare-metal environment! Thus, we must specify that **we** are going to handle panic.
We also must configure `cargo` at `.cargo/config.toml` to specify our target and the libraries/crates
we must recompile to our target.
For example: my current cargo configuration specifies that we must recompile `core` and its dependencies
from `compiler_builtins`.
This requires the <u>nightly</u> build of Rust.

Our target's specifications are in `specs/x86_64-amr_os.json`.
I don't quite understand it all yet so I'll skip it for now.

Finally, the thing we're stuck at; the bootloader a.k.a how to boot our kernel.
Because so far, we have just coded a really minimal kernel!
I could write my own bootloader in Assembly and all, but I might do it some other time.
For now, I'd like to rely on a good external bootloader that would be linked to our kernel,
allowing our OS to boot:
I could write my own bootloader in Assembly and all, but I might do it some other time.
For now, I'd like to rely on a good external bootloader that would be linked to our kernel,
allowing our OS to boot.

If I look back at the Phil-Opp blog; the only thing I'm not respecting is the version of
the `bootloader` crate/package and `bootimage`.

### It was the version.

So, uh, heheh, *cough **cough***. So, uhhh, I installed the latest version version of `bootloader`
(at the time I'm writing its `0.11`). But getting the version `0.9` fixes <u>**EVERYTHING**</u>.

Anyways, the OS now compiles and I can run it in QEMU. AND I can run my OS by simply doing
`cargo run`.
Everything is cool so far!


## [28/12/2025]

### VGA Buffer Stuff

Yesterday I wrote some print and VGA-Buffer manipulation functions that I forgot
to log here (oops). I now have some very useful functions! Like `write_byte` to
write a specific byte to a specific spot, a wrapper for `write_byte` called
`print_char_at`, and other functions like `print_string_at` and `print_string`.

My VGA "interface" (i don't know if I can call it so) works pretty simply:
We have a `Writer` type which is responsible for handling interactions with
the VGA-Buffer (all the functions I mentioned previously are implementations
of this struct). It needs to be initialized in order to be used.

`Writer` is composed of three values which consist of its column position,
color scheme to use on characters to print, and finally: the buffer.
When initialized, the writer's buffer address is set to `0xB8000` which is
the address of the VGA buffer.
Setting the buffer's address is unsafe, as Rust cannot predict what lies at
this address, nor what happens around it, or anything else.
So we <u>must</u> use the `unsafe` keyword.

---

Previously, the writer's `Buffer` looked like this:
```rust
struct Buffer {
    chars: [[CharCell; BUFFER_WIDTH]; BUFFER_HEIGHT]
}
```

It now looks like this:

```rust
struct Buffer {
    chars: [[Volatile<CharCell>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}
```

Notice the difference in the `Volatile` we added. Apparently, Rust could become
too excited and try to optimize our OS as much as possible.
Noticing that we're only writing to the buffer and not reading from it,
Rust will assume this variable is useless. Thus removing any code involving
writing to that buffer.
But it's logical for us to not read from it--it's a video buffer! We're displaying
to the screen!
I don't see a situation in which I would like to extract what's written in the buffer
which I, the developer, didn't explicitly intend to print.

### Implementing macros for `Writer`

Being able to use `print_string` is pretty cool. But the real purpose of such a function
is displaying in formation! And a lot of information can't be pre-wrote (i.e wrote before
compilation) cuz there are dynamic infos!

Ok enough nonsense: What I mean is that I should be able to print formatted strings that can
display values and variables' values. I could code a `printf` function similar to what I did
in my previous OS attempts in C. Or I could just Yank an open-source `printf` implementation
for embedded-systems or freestanding/bare-metal software from GitHub.
But our guy Phil-Opp suggested just adding an implementation of `fmt::Write` for `Writer`.
I do **not** understand how "implementation for" works in Rust:

```rust
impl <implemenation> for <idk> {
  /* ... */
}
```

Oh, well. At least I can write in a formatted way now using the `write!` macro.
Though, it's annoying having to stick a `.unwrap()` at the end of each use.
It's literally said in the blog:

***"The `write!` call returns a `Result` which causes a warning if not used,
so we call the `unwrap` function on it, which panics if an error occurs.
This isn’t a problem in our case, since writes to the VGA buffer never fail."***

Thus, I think I might just implement some wrapper for `write!` which doesn't
expect `Result` as a return type.

### Handling new lines

So far, a new line just means incrementing by 1 the row position (vertical) and reset
the column position (horizontal) to 0.
I'd like to be able to support returning to line in a fancier way (probably useless, but fancy, maybe).
For now, what my `Writer` sees as a new line is really just a fusion betweenthe new-line character `\n`
and the carriage-return character `\r`.

By fancy line-returns I mean this: Imagine you're using the `print_string_at` function
and you're printing at, let's say, column 12 and row 5.
The characters should start printing from left to right starting from the 12th column.
When encountering a new-line character `\n`, the cursor sets it position to the 12th column
and the 6th row. Get the picture? My current implementation would just return to
first column in the screen.
Or imagine you're using the character printing functions (`print_char_at` and `print_char`) and you'd like*
to do something like this:

```rust
print_char_at(b'S', 15, 10);
print_char(b'\n');
print_char(b'A');
```

What I'd like to happen is the 'S' printing at the 15th column and the 10th row,
then returning to the 15th column and go to the 11th row.
But instead, what will happen with the current code is going to the 16th column
if the 11th row.

I could just modify the `write_byte` function to increment the row position by one
and to not advance the cursor horizontally.
But this would lead to issues in the string printing functions, in which a new-line
with this suggested implementation would result in the new line start at the printed string's
last letter's position minus 1.

> Note to myself: I should implement that thing when some text overflows to the right
  so in order to not overwrite the line below it we instead push the lines below
  like terminals do.

### Too much fancying...

I think I have spent way too much time trying to make cool printing functions which
I'll probably rarely or never use. Like, seriously; writing a function to efficiently
display a text at the left, center or right of the screen, all while respecting lines-alignment
and trying to center the text based on the length of the longest line.

If I **EVER** have to do that, it's the user-space program that will handle that--my VGA interface's
only job is displaying text with a few helper functions. That's it.

So I will stop stupid-ing around and get some real stuff going.
