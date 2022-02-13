# solana-twitter

Site can be found in the description 
- [x] Solana architecture
- [x] Rust basics
- [x] Anchor framwork
- [x] Vue.js framwork
- [x] Solana tool suite 

## To build
Clone the repo
> git clone git@github.com:dabit3/complete-guide-to-full-stack-solana.git
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
```anchor build```
Fetch the project ID for the build:
```solana address -k target/deploy/<programname>-keypair.json```
Update the project ID in the Rust program located at projectname/programs/src/programname.rs with the output from above.

Run the tests

```anchor test```
Change into the app directory and install the dependencies:
```cd app && yarn install```
Run the client-side app
```yarn start```
