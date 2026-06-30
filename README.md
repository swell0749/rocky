<p align="center">
  <img src="https://github.com/swell0749/rocky/blob/main/gh-assets/full.png?raw=true" alt="rocky full logo" height="256"/>
</p>

<p align="center">
  <a href="https://stardance.hackclub.com/projects/27175">stardance project page</a>
</p>

---

## what is rocky??

rocky is a toolkit made to make your life easier, with some basic tools aswell as a custom plugins system...

> [!WARNING]
> it's not gonna be good!! at all!!! it was made in little time for [stardance](https://stardance.hackclub.com/)!!!

however, if you DO want to use it:

## how do i build rocky??

firstly, if you haven't already, you gotta install cargo!!! the preferred route is via [rustup](https://rustup.rs/)!

then, you gotta clone the repository!!

```
git clone https://github.com/swell0749/rocky.git --depth=1
```

the `--depth=1` is important because it reduces file sizes and download times by only downloading the current copy of the repository.

## how do i run rocky??

firstly, if you want debug logging, set the `RUST_LOG` environment variable to any of these:

- `debug`
- `info`
- `warn`
- `error`

but `error` is the default.

you gotta use cargo to run the code:

```
cd rocky
cargo run
```

