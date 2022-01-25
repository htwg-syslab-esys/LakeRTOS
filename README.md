# LakeRTOS

[![Cortex M Types](https://img.shields.io/badge/target-thumbv7em--none--eabihf-green)](https://docs.rust-embedded.org/cortex-m-quickstart/cortex_m_quickstart/) 

[![Status](https://img.shields.io/badge/Version-0.1%20Alpha-green)]()


## Description
Welcome to the LakeRTOS repository. This work was commissioned by the team project of the HTWG Konstanz in the winter semester 21/22,
planned and developed by [Benjamin Wilhelm] and [Lukas Kaluscha] .
LakeRTOS is a minimal implementation of a real-time operating system written entirely in the rust programming language.
During development, we aimed to make the system as light-weight as possible, so we completely dispensed with external crates.
In addition to the actual functionality, this project should also serve as a teaching object, so that there is an accompanying [Tutorial] in addition to this repository, which makes it easier to understand the written source code.

## Implemented Features
* Multiprocess Round Robin Scheduling up to N Tasks
* Basic User-/Kernelspace separating using Cortex M4 Handler-/Threadmode feature
* Basic access to GPIO Device
* Basic UART setup to print information on a host terminal
* ARM Semihosting

[Benjamin Wilhelm]: https://github.com/wolfbiker1 (Benjamin Wilhelm)
[Lukas Kaluscha]: https://github.com/turboka11e (Lukas Kaluscha)
[Tutorial]: https://app.gitbook.com/o/-M8jXJtMZDrQtS3wV0kr/s/qidxytTgteUUYfPcTiVZ/ (GitBook)