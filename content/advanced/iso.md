---
title: Standalone YWallet
weight: 50
---

In high security/value scenarios where you need a fully isolated
deployment of YWallet, you can use [Ywallet.iso](https://github.com/hhanh00/zwallet/releases/download/v1.3.5%2B398/ywallet.iso)
a live CD/USB with a barebone Linux and YWallet pre-installed.

[Ywallet.iso](https://github.com/hhanh00/zwallet/releases/download/v1.3.5%2B398/ywallet.iso)
is a ISO image that you can burn to a CD or install on a USB drive,
and then boot from it. It is meant to be used as a **offline signer**.
As such, there is no networking capabilities and no persistent state.

{{% notice warning %}}
When you shutdown the machine, your accounts are deleted.
{{% /notice %}}

## Requirements
- x86 64 bits only
