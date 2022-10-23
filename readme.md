# TokenTrade

Can transfer SOL to token and token to SOL

## Local Installation

```bash
solana-test-validator --reset
yarn install
yarn build
yarn deploy
```

## Usage

Test Transfer 1 SOL to 0.1 Token

```bash
yarn start 1
```

Test Transfer 1 Token to 0.1 SOL

```bash
yarn start 2
```


## DevNet test

Program address: https://explorer.solana.com/address/5tegohCx6eNHJmKGEXYDttfNtsG5BgVrAkrqKPMYngkp?cluster=devnet

### Transfer 0.1 SOL to 1 Token

```bash
yarn start 1
```

### Result:

https://explorer.solana.com/tx/7BLFyzoGX6cpWykcHTQhiESdyak8M64w8cLncW8WuWTXGpmX69snDtNwT2F9kTRahTPkmWwkUzQCRqwJn3Tadd7?cluster=devnet
