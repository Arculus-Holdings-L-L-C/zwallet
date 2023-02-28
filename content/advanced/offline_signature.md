---
title: Offline Signature Tutorial
parent: Advanced Usage
weight: 5
---

In this tutorial, we are going to demonstrate how to setup an offline 
account on an air-gapped device and make a transaction from a second
device that does not have the account secret key.

This workflow isolates the secret key material from any network device
and Internet connection while allowing the user to perform multiple
transactions.

You will need two devices that support YWallet. One of them must be
connected to the Internet. The second one *should* remain disconnected
after YWallet is installed.

We will call the Internet-enabled device the watch-only wallet (WO).
The second device will be called the signing device (SG).

You will also need a way to transfer files from one device to the other.
It can be a USB drive or phone cameras. In this tutorial, we'll use the
former.

## Setup

Go the Settings page and switch to Advanced Mode.
Then reopen the Settings page and **turn off Use QR for offline signing**.

## Account Setup

On the SG device, create a new account. Write down the seed phrase
somewhere safe. Then get the Unified Viewing Key.

To transfer the key to the WO device, we have several options:
- write the key in a text file and put that file on the USB drive
- show the key as a QR code and scan it on the WO device

Now we need to create an identical account on the WO device by
importing the unified viewing key.

At this point, you should have both device with the same address.
But only SG has the seed phrase.

## Fund the account

Send some funds to the account and wait for a few (3) confirmations.

The WO device should show the account balance. The SG device 
*does not* show the account balance because it has no Internet access
and therefore cannot scan the blockchain.

## Let's spend these funds

The account on your WO device behaves just like a regular account.
You can send funds, do a pool transfer, etc., anything that ultimately
leads to a transaction.

For example, let's unshield some funds. To do this, send some funds to
a Transparent address.

{{< img "Screenshot_20230228-232040.jpg" >}}

Then, initiate the transaction by tapping the "Send" button.

You will see the Transaction Plan page that gives a summary of transaction.

{{< img "Screenshot_20230228-232051.jpg" >}}

Take note of the Inputs/Outputs, Net Change in Sapling/Orchard, etc.

It is worth noting that this transaction has *LOW* privacy because it
sends funds to a transparent receiver.

Since everything looks in order, we can confirm the transaction.

Now, with a normal account, the transaction would be built, signed and 
broadcast to the network. However, in our case, we do not have
the seed phrase, and therefore we cannot sign.

Instead, the wallet prepared a file `tx.json` and shows a File Save dialog
(or Share on mobile devices). This is your **unsigned** transaction file.

Save it to your USB drive.

## Sign

Plug the USB drive into the SG device. Then go the Advanced Menu and
choose "Sign".

It will now open the load file dialog and ask you to choose the `tx.json` 
file from earlier.

At this point, you will be presented with the transaction summary again.
But this time it is on the SG device. Everything should be the same.

{{< img "2023-02-28_23-23-52.png" >}}

Check that the transaction details are what you expect. If everything
is fine, click "Send".

The SG device signs the transaction and builds the transaction bytes.
Since it is not connected to the network, it cannot broadcast them.

We need to perform a final step.

## Broadcast

The SG device should now show you the "Save" dialog. This time we
are saving the signed transaction.

Let's save it to our USB drive and move the file `tx.raw` to the
WO wallet.

Finally, use the command Advanced/Broadcast to publish our signed transaction.

{{< img "Screenshot_20230228-232636.jpg" >}}

## With Phone Cameras

The process can be greatly simplified if both devices are phones with 
cameras.

Check the option to "Use QR for offline signing" then instead of
saving, moving and loading the unsigned transaction file, the
WO wallet will show an animated series of QR codes.

On the SG device, choose "Advanced/Sign", point the phone camera
and continuously scan the QR codes. Then continue like earlier
and finish up signing the transaction.

Then, the SG device will show a series of QR codes that encodes
the *signed* transaction. Select "Broadcast" on the WO device
and scan these codes.

The process can be completed in a minute or two and remains 
air-gapped.
