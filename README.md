# homectl_sign

This command-line tool allows you to sign systemd [JSON User Records](https://systemd.io/USER_RECORD/)
without the need to recreate your entire home directory.

This solves the not so unusual problem where you transfer your home directory
(be it a LUKS file or a directory) and you forget to also transfer
the system keys associated with it.

## Usage

With long options:

```bash
homectl_sign --identity path/to/identity.json --signing-key path/to/key.private
```

Otherwise:

```bash
homectl_sign -i path/to/identity.json -s path/to/key.private
```

It gives the signed identify file as the output.

So, for example, if you want to sign with your identity file your local key
you can just:

```bash
sudo homectl_sign -i ~/.identity -s /var/lib/systemd/home/local.private > ~/.identity
```

If you are not currently logged in as the interested user,
make sure to change '~' with the appropriate home directory
(or mountpoint in case of a LUKS file).
