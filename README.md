# CasperSwap - Uniswap V2 Core for the Casper Blockchain
Implementation of `ERC20 Token`, `Pair` ,`Factory`, `Flash Swapper`, and `WCSPR` Contract for the CasperLabs platform.

## Security Review Status
![QuantstampSecured](https://s3-us-west-1.amazonaws.com/qsp-www-images/certificate-gh-badge.svg)

[View Report - commit 801eaaedf4a5fe106cebdc157c82ac801fc04caa](RengoLabs-Report.pdf)

## Contents
There are 5 contracts in this repo
1) ERC20 Token Contract
2) Pair Contract
3) Factory Contract
4) FLASH SWAPPER Contract
5) WCSPR Contract

## Table of contents

- [Interacting with the contract](#interacting-with-the-contract)
  - [Install the prerequisites](#install-the-prerequisites)
  - [Creating Keys](#creating-keys)
  - [Usage](#usage)
    - [Install](#install)
    - [Build Individual Smart Contract](#build-individual-smart-contract)
    - [Build All Smart Contracts](#build-all-smart-contracts)
    - [Individual Test Cases](#individual-test-cases)
    - [All Test Cases](#all-test-cases)
  - [Known contract hashes](#known-contract-hashes)
- [Deploying ERC20 contract manually](#deploying-erc20-contract-manually)
  - [Entry Point methods](#erc20-entry-point-methods)
    - [```transfer```](#erc20-transfer)
    - [```transfer_from```](#erc20-transfer-from)
    - [```permit```](#erc20-permit)
    - [```approve```](#erc20-approve)
    - [```balance_of```](#erc20-balance_of)
    - [```nonce```](#erc20-nonce)
    - [```allowance```](#erc20-allowance)
    - [```total_supply```](#erc20-total-supply)
    - [```mint```](#erc20-mint)
    - [```burn```](#erc20-burn)
    - [```name```](#erc20-name)
    - [```symbol```](#erc20-symbol)
- [Deploying WCSPR contract manually](#deploying-wcspr-contract-manually)
  - [Entry Point methods](#wcspr-entry-point-methods)
    - [```transfer```](#wcspr-transfer)
    - [```transfer_from```](#wcspr-transfer-from)
    - [```approve```](#wcspr-approve)
    - [```balance_of```](#wcspr-balance_of)
    - [```allowance```](#wcspr-allowance)
    - [```total_supply```](#wcspr-total_supply)
    - [```deposit```](#wcspr-deposit)
    - [```withdraw```](#wcspr-withdraw)
    - [```name```](#wcspr-name)
    - [```symbol```](#wcspr-symbol)
- [Deploying PAIR contract manually](#deploying-pair-contract-manually)
  - [Manual Deployment](#pair-manual-deployment)
  - [Entry Point methods](#pair-entry-point-methods)
    - [```transfer```](#pair-transfer)
    - [```transfer_from```](#pair-transfer-from)
    - [```swap```](#pair-swap)
    - [```skim```](#pair-skim)
    - [```sync```](#pair-sync)
    - [```permit```](#pair-permit)
    - [```approve```](#pair-approve)
    - [```balance_of```](#pair-balance_of)
    - [```nonce```](#pair-nonce)
    - [```allowance```](#pair-allowance)
    - [```total_supply```](#pair-total_supply)
    - [```mint```](#pair-mint)
    - [```burn```](#pair-burn)
    - [```treasury_fee```](#pair-treasury-fee)
    - [```set_treasury_fee_percent```](#pair-treasury-fee-percent)
    - [```token0```](#pair-token0)
    - [```token1```](#pair-token1)
    - [```initilize```](#pair-initialize)
    - [```get_reserves```](#pair-get-reserves)
    - [```erc20_mint```](#pair-erc20-mint)
- [Deploying FACTORY contract manually](#deploying-factory-contract-manually)
  - [Entry Point methods](#factory-entry-point-methods)
    - [```create_pair```](#factory-create-pair)
    - [```get_pair```](#factory-get-pair)
    - [```fee_to```](#factory-fee-to)
    - [```fee_to_setter```](#factory-fee-to-setter)
    - [```all_pairs```](#factory-all-pairs)
    - [```all_pairs_length```](#factory-all-pairs)
    - [```set_fee_to```](#factory-set-fee-to)
    - [```set_fee_to_setter```](#factory-set-fee-to-setter)
- [Deploying FLASH SWAPPER contract manually](#deploying-flash-swapper-contract-manually)
  - [Manual Deployment](#flash-swapper-manual-deployment)
  - [Entry Point methods](#flash-swapper-entry-point-methods)
    - [```start_swap```](#flash-swapper-start-swap)
    - [```uniswap_v2_call```](#flash-swapper-uniswap-v2-call)



## Interacting with the contract
You need to have `casper-client` and `jq` installed on your system to run the examples. The instructions have been tested on Ubuntu 20.04.2 LTS.

### Install the prerequisites

You can install the required software by issuing the following commands. If you are on an up-to-date Casper node, you probably already have all of the prerequisites installed so you can skip this step.

```bash
# Update package repositories
sudo apt update

# Install the command-line JSON processor
sudo apt install jq -y

# Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#Install the nightly version (by default stable toolchain is installed)
rustup install nightly

#Check that nightly toolchain version is installed(this will list stable and nightly versions)
rustup toolchain list

#Set rust nightly as default
rustup default nightly

# Install wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown

#rust Version
rustup --version

#Install Cmake
 sudo apt-get -y install cmake

Note:https://cgold.readthedocs.io/en/latest/first-step/installation.html

#cmake Version
cmake --version

#Installing the Casper Crates
cargo install cargo-casper

# Add Casper repository
echo "deb https://repo.casperlabs.io/releases" bionic main | sudo tee -a /etc/apt/sources.list.d/casper.list
curl -O https://repo.casperlabs.io/casper-repo-pubkey.asc
sudo apt-key add casper-repo-pubkey.ascr
sudo apt update

# Install the Casper client software
Install Casper-client

cargo +nightly install casper-client

# To check Casper Client Version
Casper-client --version

# Commands for help
casper-client --help

casper-client <command> --help

```
### Creating Keys

```bash
# Create keys
casper-client keygen <TARGET DIRECTORY>
```

### Usage
To run the Contracts make sure you are in the folder of your required contract.
#### Install
Make sure `wasm32-unknown-unknown` is installed.
```
make prepare
```

It's also recommended to have [wasm-strip](https://github.com/WebAssembly/wabt)
available in your PATH to reduce the size of compiled Wasm.

#### Build Individual Smart Contract
Run this command to build Smart Contract.
```
make build-contract
```
<br>**Note:** User needs to be in the desired project folder to build contracts and User needs to run `make build-contract` in every project to make wasms to avoid errors

#### Build All Smart Contracts
Run this command in main folder to build all Smart Contract.
```
make all
```

#### Individual Test Cases
Run this command to run Test Cases.
```
make test
```
<br>**Note:** User needs to be in the desired project folder to run test cases

#### All Test Cases
Run this command in main folder to run all contract's Test Cases.
```
make test
```

### Known contract hashes

All contracts have already being deployed. Inorder to interact with the specific contract you need to call it by its hash. The table below contains the contract hash (without the `hash-` prefix) for all the contracts on public Casper networks:

Network| Contract Name | Account info contract hash | Contract owner
---|---|---|---
Testnet| ERC20 | `hash-279445c140615fd511759dfb96c610dee212769913f61a57b0f9dde42d6a8d10` | Casper
Association
Testnet| WCSPR | `hash-4f2d1b772147b9ce3706919fe0750af6964249b0931e2115045f97e1e135e80b` | Casper
Association
Testnet| FLASHSWAPPER | ` hash-1c23f9e89033e5c2d2a21a6926411b2645c000cf43fc0db495821633da2aed6e` | Casper
Association
Testnet| PAIR | `hash-de6ba94b699dad44e12bf98e35c1122eed7dba9eed8af6d8952875afaec8c7dd` | Casper
Association
Testnet| FACTORY | `hash-13cc83616c3fb4e6ea22ead5e61eb6319d728783ed02eab51b1f442085e605a7` | Casper
Association


### Deploying ERC20 contract manually

If you need to deploy the `ERC20 contract` manually you need to pass the some parameters. Following is the command to deploy the `ERC20 contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:string='token-name'" \
    --session-arg="symbol:string='token-symbol'" \
    --session-arg="decimals:u8='unsigned integer value'" \
    --session-arg="initial_supply:u256='unsigned integer value'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="erc20-entry-point-methods"></a>

Following are the ERC20's entry point methods.

- #### transfer <a id="erc20-transfer"></a>
Lets ` self.get_caller() ` send pool tokens to a recipient hash.

Following is the table of parameters.

Parameter Name | Type
---|---
recipient | Key
amount | U256


This method **returns** nothing.

- #### transfer_from <a id="erc20-transfer-from"></a>
Sends pool tokens from one hash to another.
<br>User needs to call approve method before calling the ` tranfer_from `.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key
recipient | Key
amount | U256


This method **returns** nothing.
<br>**Recommendation:**
The exploit is mitigated through use of functions that increase/decrease the allowance relative to its current value, such as `increaseAllowance()` and `decreaseAllowance()`,
Pending community agreement on an ERC standard that would protect against this exploit, we recommend that developers of applications dependent on approve() / transferFrom()
should keep in mind that they have to set allowance to 0 first and verify if it was used before setting the new value.
<br>**Note:**  Teams who decide to wait for such a standard should make these
recommendations to app developers who work with their token contract.

- #### permit <a id="erc20-permit"></a>
Sets the allowance for a spender where approval is granted via a signature.

Following is the table of parameters.

Parameter Name | Type
---|---
public | String
signature | String
owner | Key
spender | Key
value | U256
deadline | u64


This method **returns** nothing.


- #### approve <a id="erc20-approve"></a>
Lets ` self.get_caller() ` set their allowance for a spender.
<br>user needs to call this `approve` method before calling the `transfer_from` method.

Following is the table of parameters.

Parameter Name | Type
---|---
spender | Key
amount | U256

This method **returns** nothing.
<br>**Recommendation:**
The exploit is mitigated through use of functions that increase/decrease the allowance relative to its current value, such as `increaseAllowance()` and `decreaseAllowance()`,
Pending community agreement on an ERC standard that would protect against this exploit, we recommend that developers of applications dependent on approve() / transferFrom()
should keep in mind that they have to set allowance to 0 first and verify if it was used before setting the new value.
<br>**Note:**  Teams who decide to wait for such a standard should make these
recommendations to app developers who work with their token contract.

- #### balance_of <a id="erc20-balance-of"></a>
This method will return the balance of owner in `ERC20 Contract`.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key


This method **returns** U256.


- #### nonce <a id="erc20-nonce"></a>
Returns the current `nonce` for an address for use in ` permit `.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key


This method **returns** U256.


- #### allowance <a id="erc20-allowance"></a>
Returns the amount of liquidity tokens owned by an hash that a spender is allowed to transfer via ` transfer_from `.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key
spender | Key


This method **returns** U256.


- #### total_supply <a id="erc20-total-supply"></a>
Returns the total amount of pool tokens for a pair.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** U256.


- #### mint <a id="erc20-mint"></a>
This method mints the number of tokens provided by user against the hash provided by user.

Following is the table of parameters.

Parameter Name | Type
---|---
to | Key
amount | U256

This method **returns** nothing.


- #### burn <a id="erc20-burn"></a>
This method burns the number of tokens provided by user against the hash provided by user.

Following is the table of parameters.

Parameter Name | Type
---|---
from | Key
amount | U256

This method **returns** nothing.
<br>**Note:** To `burn` the tokens against the hash provided by user, User needs to `mint` tokens first in `ERC20`.

- #### name <a id="erc20-name"></a>
Returns the `name` of tokens for a pair.

Following is the table of parameters.

Parameter Name | Type
---|---

This method **returns** String.

- #### symbol <a id="erc20-symbol"></a>
Returns the `symbol` of tokens for a pair.

Following is the table of parameters.

Parameter Name | Type
---|---

This method **returns** String.


### Deploying WCSPR contract manually

If you need to deploy the `WCSPR contract` manually you need to pass the some parameters. Following is the command to deploy the `WCSPR contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:string='token-name'" \
    --session-arg="symbol:string='token-symbol'" \
    --session-arg="decimals:u8='unsigned integer value'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="wcspr-entry-point-methods"></a>

Following are the WCSPR's entry point methods.

- #### transfer <a id="wcspr-transfer"></a>
Lets ` self.get_caller() ` send pool tokens to a recipient hash.

Following is the table of parameters.

Parameter Name | Type
---|---
recipient | Key
amount | U256


This method **returns** nothing.


- #### transfer_from <a id="wcspr-transfer-from"></a>
Sends pool tokens from one hash to another.
<br>User needs to call `approve` method before calling the `tranfer_from`.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key
recipient | Key
amount | U256


This method **returns** nothing.
<br>**Recommendation:**
The exploit is mitigated through use of functions that increase/decrease the allowance relative to its current value, such as `increaseAllowance()` and `decreaseAllowance()`,
Pending community agreement on an ERC standard that would protect against this exploit, we recommend that developers of applications dependent on approve() / transferFrom()
should keep in mind that they have to set allowance to 0 first and verify if it was used before setting the new value.
<br>**Note:**  Teams who decide to wait for such a standard should make these
recommendations to app developers who work with their token contract.


- #### approve <a id="wcspr-approve"></a>
Lets `self.get_caller()` set their allowance for a spender.
<br>user needs to call this `approve` method before calling the `transfer_from` method.

Following is the table of parameters.

Parameter Name | Type
---|---
spender | Key
amount | U256

This method **returns** nothing.
<br>**Recommendation:**
The exploit is mitigated through use of functions that increase/decrease the allowance relative to its current value, such as `increaseAllowance()` and `decreaseAllowance()`,
Pending community agreement on an ERC standard that would protect against this exploit, we recommend that developers of applications dependent on approve() / transferFrom()
should keep in mind that they have to set allowance to 0 first and verify if it was used before setting the new value.
<br>**Note:**  Teams who decide to wait for such a standard should make these
recommendations to app developers who work with their token contract.

- #### balance_of <a id="wcspr-balance-of"></a>
This method will return the balance of owner in `WCSPR Contract` .

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key

This method **returns** U256.

- #### allowance <a id="wcspr-allowance"></a>
Returns the amount of liquidity tokens owned by an hash that a spender is allowed to transfer via ` transfer_from `.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key
spender | Key

This method **returns** U256.

- #### total_supply<a id="wcspr-total-supply"></a>
Returns the total amount of pool tokens for a pair.

This method **returns** U256.


- #### deposit <a id="wcspr-deposit"></a>
This method deposits the number of tokens provided by user against the hash provided by user.

Following is the table of parameters.

Parameter Name | Type
---|---
to | Key
purse | URef

This method **returns** nothing.


- #### withdraw <a id="wcspr-withdraw"></a>
This method withdraws the number of tokens provided by user against the hash provided by user.

Following is the table of parameters.

Parameter Name | Type
---|---
to | Key
amount | U256

This method **returns** nothing.
<br>**Note:** To `withdraw` the tokens against the hash provided by user, User needs to `deposit` tokens first in `WCSPR`.

- #### name <a id="wcspr-name"></a>
Returns the `name` of tokens for a pair.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** String.


- #### symbol <a id="wcspr-symbol"></a>
Returns the `symbol` of tokens for a pair.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** String.


### Deploying PAIR contract manually

If you need to deploy the `PAIR contract` manually you need to pass the hashes of the other contracts as parameter. Following is the command to deploy the `PAIR contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:string='token-name'" \
    --session-arg="symbol:string='token-symbol'" \
    --session-arg="decimals:u8='unsigned integer value'" \
    --session-arg="initial_supply:u256='unsigned integer value'" \
    --session-arg="contract_name:string='contract_name'"
    --session-arg="factory_hash:Key='Hash of factory Contract'" \
    --session-arg="callee_contract_hash:Key='Flash Swapper Contract Hash'" \
```

Before deploying `PAIR Contract`, you would need to deploy other contracts first and pass hashes of these contracts to the respective parameters above. We have already deployed these contracts and the tables belows displays the hashes of the contracts.

Name | Network | Account info contract hash | Contract owner
---|---|---|---
Factory | Testnet | `hash-13cc83616c3fb4e6ea22ead5e61eb6319d728783ed02eab51b1f442085e605a7` | Casper Association
Flash Swapper | Testnet | ` hash-1c23f9e89033e5c2d2a21a6926411b2645c000cf43fc0db495821633da2aed6e` | Casper Association



### Manual Deployment <a id="pair-manual-deployment"></a>

For manual deployments of these contracts, following are the commands.

#### Factory
```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="fee_to_setter:Key='Hash of fee-to-setter Contract'" \
    --session-arg="contract_name:string='contract_name'"
```

#### Flash Swapper
```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="uniswap_v2_factory:Key='Hash of factory Contract'" \
    --session-arg="wcspr:Key='Hash of WCSPR Contract'" \
    --session-arg="dai:Key='Hash of DAI Contract'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="pair-entry-point-methods"></a>

Following are the PAIR's entry point methods.

- #### transfer <a id="pair-transfer"></a>
Lets ` self.get_caller ` send pool tokens to a recipient hash.

Following is the table of parameters.

Parameter Name | Type
---|---
recipient | Key
amount | U256


This method **returns** nothing.


- #### transfer_from <a id="pair-transfer-from"></a>
Sends pool tokens from one hash to another.
<br>**Note:** User needs to call `approve` method before calling the `tranfer_from` .

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key
recipient | Key
amount | U256


This method **returns** nothing.

- #### swap <a id="pair-swap"></a>
Swaps tokens. For regular swaps, ` data.length ` must be ` 0 `.
<br> **Note:** To call this method explicitly, User needs to deploy a `Factory contract` first and call a method `create_pair` which invokes the `initialize` methods of `Pair contract` that's how the `Pair contract` can access the `token0` and `token1` after this user needs to mint `token0` and `token1` by calling an `erc20_mint` method in `pair contract` or you can transfer some tokens to it, so they have some balance in them. To call the `swap` method the user needs to have some balance in `reserve0` and `reserve1`.

Following is the table of parameters.

Parameter Name | Type
---|---
amount0_out | U256
amount1_out | U256
to | Key
data | String


This method **returns** nothing.

- #### skim <a id="pair-skim"></a>
<br>**Note:** To call this method explicitly, User needs to deploy a `Factory contract` first and call a method `create_pair` which invokes the `initialize` methods of `Pair contract` that's how the `Pair contract` can access the `token0` and `token1` after this user needs to mint `token0` and `token1` by calling an `erc20_mint` method in `Pair contract` or you can transfer some tokens to it, so they have some balance in them. To call the `skim` method the user needs to have some balance in `reserve0` and `reserve1`.

Following is the table of parameters.

Parameter Name | Type
---|---
to | Key


This method **returns** nothing.


- #### sync <a id="pair-sync"></a>
<br>**Note:** To call this method explicitly, User needs to deploy a `Factory contract` first and call a method `create_pair` which invokes the `initialize` methods of `Pair contract` that's how the `Pair contract` can access the `token0` and `token1` after this user needs to mint `token0` and `token1` by calling an `erc20_mint` method in `Pair contract` or you can transfer some tokens to it, so they have some balance in them.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** nothing.


- #### permit  <a id="pair-permit"></a>
Sets the allowance for a spender where approval is granted via a signature.

Following is the table of parameters.

Parameter Name | Type
---|---
public | String
signature | String
owner | Key
spender | Key
value | U256
deadline | u64


This method **returns** nothing.


- #### approve <a id="pair-approve"></a>
Lets ` self.get_caller() ` set their allowance for a spender.
<br>user needs to call this `approve` method before calling the `transfer_from` method.

Following is the table of parameters.

Parameter Name | Type
---|---
spender | Key
amount | U256


This method **returns** nothing.
<br>**Recommendation:**
The exploit is mitigated through use of functions that increase/decrease the allowance relative to its current value, such as `increaseAllowance()` and `decreaseAllowance()`,
Pending community agreement on an ERC standard that would protect against this exploit, we recommend that developers of applications dependent on approve() / transferFrom()
should keep in mind that they have to set allowance to 0 first and verify if it was used before setting the new value.
<br>**Note:**  Teams who decide to wait for such a standard should make these
recommendations to app developers who work with their token contract.

- #### balance_of <a id="pair-balance-of"></a>
Returns the amount of pool tokens owned by a hash.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key


This method **returns** U256.


- #### nonce <a id="pair-nonce"></a>
Returns the current `nonce` for an address for use in ` permit `.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key


This method **returns** U256.


- #### allowance <a id="pair-allowance"></a>
Returns the amount of liquidity tokens owned by an hash that a spender is allowed to transfer via ` transfer_from `.

Following is the table of parameters.

Parameter Name | Type
---|---
owner | Key
spender | Key


This method **returns** U256.
<br>**Recommendation:**
The exploit is mitigated through use of functions that increase/decrease the allowance relative to its current value, such as `increaseAllowance()` and `decreaseAllowance()`,
Pending community agreement on an ERC standard that would protect against this exploit, we recommend that developers of applications dependent on approve() / transferFrom()
should keep in mind that they have to set allowance to 0 first and verify if it was used before setting the new value.
<br>**Note:**  Teams who decide to wait for such a standard should make these
recommendations to app developers who work with their token contract.

- #### total_supply <a id="pair-total-supply"></a>
Returns the total amount of pool tokens for a pair.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** U256.


- #### mint <a id="pair-mint"></a>
Creates pool tokens.
<br>**Note:** To call this method explicitly, User needs to deploy a `Factory contract` first and call a method `create_pair` which invokes the `initialize` methods of `Pair contract` that's how the `Pair contract` can access the `token0` and `token1`, To call the mint user needs to do all the above steps so he can proceed flawlessly.

Following is the table of parameters.

Parameter Name | Type
---|---
to | Key

This method **returns** U256.

- #### burn <a id="pair-burn"></a>
Destroys pool tokens.
<br>**Note:** User needs to mint tokens before burning them. And user needs to deploy a `Factory contract` first and call a method `create_pair` which invokes the `initialize` method of `Pair contract` that's how the `Pair contract` can access the `token0` and `token1`. To call the burn user needs to do all the above steps so he can proceed flawlessly.


Following is the table of parameters.

Parameter Name | Type
---|---
to | Key

This method **returns** Tuple(U256, U256).


- #### treasury_fee <a id="pair-treasury-fee"></a>
Returns the Treasury Fee for a pair.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** U256.


- #### set_treasury_fee_percent <a id="pair-treasury-fee-percent"></a>
sets the treasury fee for a pair.
<br>**Note:** treasury_fee_percent Cannot be more than `30` and less than 3. If it’s more than `30` it will set it as `30` and if it's less than 3 it will set it as '3'.

Following is the table of parameters.

Parameter Name | Type
---|---
treasury_fee | U256


This method **returns** nothing.


- #### token0 <a id="pair-token0"></a>
Returns the hash of the pair token with the `lower sort order`.

Following is the table of parameters.

Parameter Name | Type
---|---

This method **returns** Key.


- #### token1 <a id="pair-token1"></a>
Returns the address of the pair token with the `higher sort order`.

Following is the table of parameters.

Parameter Name | Type
---|---

This method **returns** Key.


- #### initialize <a id="pair-initialize"></a>
Sets the `token0` and `token1` in pair contract.
<br>**Note:**  This method will be called by `Factory contract` only and the user needs to pass the factory hash to make sure is it a factory or not.

Following is the table of parameters.

Parameter Name | Type
---|---
token0 | Key
token1 | Key
factory_hash | Key

This method **returns** nothing.


- #### get_reserves <a id="pair-get-reserves"></a>
Returns the reserves of token0 and token1 used to price trades and distribute liquidity. Also returns the block_time_stamp `(mod 2**32)` of the last block during which an interaction occured for the pair.

Following is the table of parameters.

Parameter Name | Type
---|---

This method **returns** Tupe3(U128, U128, u64).


- #### erc20_mint <a id="pair-erc20-mint"></a>
This method mints the number of tokens provided by user against the hash provided by user.

Following is the table of parameters.

Parameter Name | Type
---|---
to | Key
amount | U256

This method **returns** nothing.


### Deploying FACTORY contract manually

If you need to deploy the `FACTORY contract` manually you need to pass the some parameters. Following is the command to deploy the `FACTORY contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="fee_to_setter:Key='Account Hash of a user'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="factory-entry-point-methods"></a>

Following are the FACTORY's entry point methods.

- #### create_pair <a id="factory-create-pair"></a>
Creates a pair for `token_a` and `token_b` if one doesn't exist already.
<br>**Note:** `token_a` and `token_b` are interchangeable and The user needs to deploy the pair contract before calling the create pair method so he can pass the `Pair contract` hash as a parameter which allows the `Factory contract` to call the `initialize` methods of `Pair Contract`.
Following is the table of parameters.

Parameter Name | Type
---|---
token_a | Key
token_b | Key
pair_hash | Key


This method **returns** nothing.


- #### get_pair <a id="factory-get-pair"></a>
Returns the hash of the pair for `token0` and `token1`, if it has been created, else `“Hash-0000000000000000000000000000000000000000000000000000000000000000”`.
<br>**Note:** `token0` and `token1` are interchangeable.

Following is the table of parameters.

Parameter Name | Type
---|---
token0 | Key
token1 | Key


This method **returns** Key.


- #### fee_to <a id="factory-fee-to"></a>
Returns the hash of `fee_to`.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** Key.


- #### fee_to_setter <a id="factory-fee-to-setter"></a>
Returns the hash of `fee_to_setter`.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** Key.

- #### all_pairs <a id="factory-all-pairs"></a>
Returns the list of all pairs created.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** list of Keys.


- #### all_pairs_length <a id="factory-all-pairs-length"></a>
Returns the total number of pairs created through the `factory` so far.

Following is the table of parameters.

Parameter Name | Type
---|---


This method **returns** U256.


- #### set_fee_to <a id="factory-set-fee-to"></a>
this will set the hash of `fee_to`
<br>**Note:** Only `fee_to_setter` can set the `fee_to`

Following is the table of parameters.

Parameter Name | Type
---|---
fee_to | Key


This method **returns** nothing.


- #### set_fee_to_setter <a id="factory-set--fee-to-setter"></a>
this will set the Hash of `fee_to_setter`
<br>**Note:** Only `fee_to_setter` can set the `fee_to_setter`

Following is the table of parameters.

Parameter Name | Type
---|---
fee_to_setter | Key


This method **returns** nothing.


### Deploying FLASH SWAPPER contract manually

If you need to deploy the `Flash swapper contract` manually you need to pass the hashes of the other contracts as parameter. Following is the command to deploy the `Flash Swapper contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="uniswap_v2_factory:Key='Hash of factory Contract'" \
    --session-arg="wcspr:Key='Hash of WCSPR Contract'" \
    --session-arg="dai:Key='Hash of DAI Contract'" \
    --session-arg="contract_name:string='contract_name'"
```


Before deploying `Flash Swapper Contract`, you would need to deploy other contracts first and pass hashes of these contracts to the respective parameters above. We have already deployed these contracts and the tables belows displays the hashes of the contracts.

Name | Network | Account info contract hash | Contract owner
---|---|---|---
Factory | Testnet | `hash-13cc83616c3fb4e6ea22ead5e61eb6319d728783ed02eab51b1f442085e605a7` | Casper Association
Wcspr | Testnet | `hash-4f2d1b772147b9ce3706919fe0750af6964249b0931e2115045f97e1e135e80b` | Casper Association
Dai | Testnet | `hash-ffb8fa3073c7623484f76d79bc8baad110b24936b92d5ebc854d401895e88c95` | Casper Association


### Manual Deployment <a id="flash-swapper-manual-deployment"></a>

For manual deployments of these contracts, following are the commands.

#### Factory
```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="fee_to_setter:Key='Hash of fee-to-setter Contract'" \
    --session-arg="contract_name:string='contract_name'"
```

#### Wcspr
```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:string='token-name'" \
    --session-arg="symbol:string='token-symbol'" \
    --session-arg="decimals:u8='unsigned integer value'" \
    --session-arg="contract_name:string='contract_name'"
```

#### Dai
```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:string='token-name'" \
    --session-arg="symbol:string='token-symbol'" \
    --session-arg="decimals:u8='unsigned integer value'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="flash-swapper-entry-point-methods"></a>

Following are the Flash Swapper's entry point methods.

- #### start_swap <a id="flash-swapper-start-swap"></a>
This method will start swap.
<br>Special Instructions: The user needs to call this (start_swap) method first to call the `uniswap_v2_call` method.
`Start_swap` method will further call 3 methods
- simple_flash_loan
This method will be invoked if both tokens (token_borrow and token_pay) are the same.
- simple_flash_swap
This method will be invoked if both tokens (token_borrow and token_pay) are not the same. one of them must be equal to
“Hash-0000000000000000000000000000000000000000000000000000000000000000”
- triangular_flash_swap
This method will be invoked if both tokens (token_borrow and token_pay) are not the same.
The above mthods will invoke the swap methods of `Pair` Contract by using the `permissioned_pair_address`. And then the `swap` method will invoke the `uniswap_v2_call` method.

Following is the table of parameters.

Parameter Name | Type
---|---
token_borrow | Key
amount | U256
token_pay | Key
user_data | String

This method **returns** nothing.


- #### uniswap_v2_call <a id="flash-swapper-uniswap-v2-call"></a>
This method is called by `swap` method of `pair contract`.
<br>the sender must be a `Flash Swapper Contract` hash if user data has some value.
`Uniswap_v2_call` must be called from a contract. Users cannot directly invoke this method.


Following is the table of parameters.

Parameter Name | Type
---|---
sender | Key
amount0 | U256
amount1 | U256
data | String

This method **returns** nothing.
