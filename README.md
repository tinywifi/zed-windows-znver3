# Unofficial nightly builds of Zed for Windows

## Warning

This is a personal repo built for my CPU with `-C target-cpu=alderlake`. You should not use this repo or its builds. Use the parent repo as it is generalized for any CPU.

But if you're interested in getting nightly builds customized to your specific CPU, then fork this repo and run this cmd in your terminal:

`rustc --target=x86_64-pc-windows-msvc --print target-cpus`

At the top you'll see something like
```
Available CPUs for this target:
    native                  - Select the CPU of the current host (currently alderlake).
```
In my case it was `alderlake`. In [build.yml](https://github.com/MolotovCherry/zed-windows-builds/blob/main/.github/workflows/build.yml), change the `alderlake` part in the `-C target-cpu=alderlake` to your specific CPU.

## Is it safe?

This repository is just a [simple GitHub workflow](./.github/workflows/build.yml) that builds Zed from `main` and publishes a release every night at UTC+0000. (Additionally on push for testing).

See the [Zed homepage](https://zed.dev/) or [official repository](https://github.com/zed-industries/zed) for more details.
