# key-recovery

This is a very simple utility to recover private keys from the obsolete `Namada Extension`.

This work is based on the [a444534](https://github.com/anoma/namada-interface/tree/a444534f181f48a93f8ffda1ea65bc7b41d310b6/) of `namada-interface`.

## Usage

```bash
# Optionally, run `cargo build` and set your path to the corresponding binary in `target/`
cargo run
```

- When prompted, enter your mnemonic
- When prompted, enter derivation path, e.g., `m/44'/877'/0'/0/0`
- Confirm the resulting `Address` and `Public Key` in the output, then copy the `Private Key`
- In the current Namada Keychain, go to `Add Keys`, then choose the `Private Key` tab, and paste the key here
  - You should see the same Address & Public Key displayed here. You can now sign with this key in the current extension!

Derivation paths to try that could have been used in this early wallet:

- `m/44'/877'/0'`
- `m/44'/877'/0'/0/0`
- `m/44'/877'/0'/0'/0'`

## Tests

To run unit test, issue the following command:

```bash
cargo test
```
