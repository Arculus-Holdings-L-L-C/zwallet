---
title: Database Encryption
weight: 40
---

## Desktop Only

If you use the database encryption feature, all the database files are
encrypted with 256 bit AES encryption in CBC mode.

The encryption key is derived from your password/passphrase
using PBKDF2. For more details, refer to [SQLCipher Design](https://www.zetetic.net/sqlcipher/design/).

By encrypting your database files, your accounts are protected in case
malware exfiltrates them our of your computer.

{{% notice note %}}
This feature is only offered on the desktop versions because mobile 
devices have file system encryption and application isolation.
Their security model is stronger than desktop OS. The later typically
only provides permissions at the user level.
On mobile devices, the "Protect Open" feature provides the same 
functionality.
{{% /notice %}}

## Encrypt/Decrypt

Choose the "Advanced/Encrypt Database" menu item.
{{< img 2023-03-06_16-57-27.png >}}

If your database is encrypted, you will need to provide the
current database password.

Input the new password and repeat it for confirmation.

{{< img 2023-03-06_17-10-25.png >}}

To remove database encryption, leave the new password empty.

## Startup

{{< img 2023-03-08_14-38-22.png >}}

At startup, you will be required to enter the database password
if you have set one. 

{{% notice warning %}}
If you forget your database password, you will have to recover
your accounts from seed or from batch backup!
{{% /notice %}}

## Backup / Restore

Backing up an encrypted database is not supported. Remove the encryption
before making a batch backup.
After restoring a batch backup, the database is unencrypted.

## Forgotten Password

If you need to reset the state of the application because you forgot
the database password, you have to reset the app.

{{< img 2023-03-08_14-38-41.png >}}

{{% notice warning %}}
Resetting the database **DELETES** every account.
{{% /notice %}}
