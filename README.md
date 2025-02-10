# key-recovery

This tool allows those keys to be recovered using a mnemonic phrase, or by decrypting secrets from a Namada `wallet.toml`.

The inital _alpha_ version of the Namada extension used a different derivation path, along with a different library for deriving
private keys.

This work is based on the [a444534](https://github.com/anoma/namada-interface/tree/a444534f181f48a93f8ffda1ea65bc7b41d310b6/) of `namada-interface`.

## Usage

**NOTE**: In the process of implementing sub-commands, one for deriving from mnemonic, one for attempting to
recover from a `wallet.toml` file.

To run the key-recoverer, simply run the following in a Linux or macOS terminal:

1. Copy your `wallet.toml` to the root directory of this repo
2. Run the following:

```bash
# Optionally, run `cargo build` and set your path to the corresponding binary in `target/`
cargo run
```

**TODO**: Run `cargo run export`

3. When prompted for an account to recover, enter the `alias` of the account you wish to recover, then when
   prompted, enter your password.

### Recovery from mnemonic using obsolete derivation

**NOTE**: For now, uncomment the line `// cli::hdkeys::derive();`, then `cargo run`:

**TODO**: Run `cargo run derive`

- When prompted, enter your mnemonic
- When prompted, enter derivation path, e.g., `m/44'/877'/0'/0/0`
- Confirm the resulting `Address` and `Public Key` in the output, then copy the `Private Key`
- In the current Namada Keychain, go to `Add Keys`, then choose the `Private Key` tab, and paste the key here
  - You should see the same Address & Public Key displayed here. You can now sign with this key in the current extension!

### TODO:

Enable sub-command to run the mnemonic derivation (currently commented out in `main.rs`):

## Tests

To run unit test, issue the following command:

```bash
cargo test
```

## Notes

- Original PR that updated derivation to match CLI: [#434](https://github.com/anoma/namada-interface/pull/434)
  - Update Mnemonic to use `tiny-bip39` instead of `bip0039`
  - Update HDWallet to use `slip10_ed25519` instead of `ed25519_dalek`
- Example of original derivation: [keyring.ts](https://github.com/anoma/namada-interface/blob/d0a9da882943925d5b8f88af8a894a99d9e49a13/apps/extension/src/background/keyring/keyring.ts#L234)
- CoinType already `877` as of this version of [chains/namada.ts](https://github.com/anoma/namada-interface/blob/d0a9da882943925d5b8f88af8a894a99d9e49a13/packages/chains/src/chains/namada.ts#L21)
  - Default path on Mnemonic import was: `m/44'/877'/0'/0`
