# VNC server for Remarkable 2 tablet

The goal of this project was to gain enough experience with Ghidra to reverse
engineer the software running on the Remarkable 2 tablet (`xochitl`) to be able
to write a VNC server for it.

This works! But some of the code might be specific to the [version 2.7 of the
software](https://support.remarkable.com/s/article/Software-release-2-7-May-2021),
which got released in May 2021. For example, the address of the screen memory
is hard-coded in `ScreenMemory::new`.

