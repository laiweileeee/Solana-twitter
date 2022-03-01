# solana-twitter

solana-tweeter.netlify.app/#/

![](https://github.com/laiweileeee/solana-twitter/blob/main/solana-tweeter.gif)

Things I learnt
- [x] Solana architecture
- [x] Rust basics
- [x] Anchor framwork
- [x] Vue.js framwork
- [x] Solana tool suite 

### Pre-requisites
1. Node.js - Install Node using either nvm or fnm

2. Solana Tool Suite - Installation instructions [here](https://docs.solana.com/cli/install-solana-cli-tools). M1 Mac users troubleshoot info [here](https://github.com/project-serum/anchor/issues/95#issuecomment-913090162) 

3. Anchor - Installation instructions [here](https://project-serum.github.io/anchor/getting-started/installation.html).

4. Solana browser wallet - I use [Phantom](https://phantom.app/)


### To build
Clone the repo
```
git clone https://github.com/laiweileeee/solana-twitter.git
```
Change into the project directory you'd like to run

Install the dependencies

```
yarn install
```
Start a local Solana node
```
solana-test-validator
```
Build the anchor project
```
anchor build
```
Fetch the project ID for the build:
```
solana address -k target/deploy/<programname>-keypair.json
```
Update the project ID in the Rust program located at solana-twitter/programs/solana-twitter/src/lib.rs with the output from above.

Run the tests

```
anchor test
```
Change into the app directory and install the dependencies:
```
cd app && yarn install
```
Run the client-side app
```
yarn start
```
