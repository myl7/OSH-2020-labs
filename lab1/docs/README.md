# Lab1

## Linux kernel

Download Linux 5.4.22 src from [The Linux Kernel Archives][linux-src].

Extract the `.tar.xz` file:

```bash
tar xf linux-5.4.22.tar.xz
```

Get into the src folder and compile the kernel.

```bash
cd linux-5.4.22
make defconfig  # Use default config according to current OS and hardware environment.
make xconfig  # As I am using KDE, use QT-based GUI config windows.
# Remove some unused functions, such as network, sound and virtualization.
make -j6  # I am usng i5-8250U, which is a 4C8T low-voltage CPU.
# Then we can found the `bzImage` file in `arch/x86/boot`, which is 2113 kB large.
```

## initrd and init

### Simple initrd

Create `init.c` with respective code.

Compile and staticly link it with `gcc`.

Create the gzip initrd with `cpio` and `gzip`.

Boot the kernel with the initrd with:

```bash
qemu-system-x86_64 -kernel bzImage -initrd initrd.cpio.gz
```

We will see:

![initrd1-qemu](./images/initrd1-qemu.png)

Which has no "Hello, Linux!".

But this is because we try to kill the init process, which caused kernel panic.
The error info full the VGA memory-mapped screen so we can not see the required output.
Later in initrd2 we will see it.

## x86 bare metal MBR program

## Questions

[linux-src]: https://www.kernel.org/