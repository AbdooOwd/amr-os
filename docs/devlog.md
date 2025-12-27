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
