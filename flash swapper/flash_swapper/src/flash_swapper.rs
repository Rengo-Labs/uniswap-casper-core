use alloc::{format, string::String,vec::Vec};

use casper_contract::{contract_api::{runtime::{self, call_contract}}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{ApiError, ContractHash,Key, RuntimeArgs, U256, runtime_args};
use contract_utils::{ContractContext, ContractStorage};


use crate::data::{self};

use contract_utils::{set_key,get_key};

/// Enum for FailureCode, It represents codes for different smart contract errors.
#[repr(u16)]
enum FailureCode {

      /// 65,536 for (Requested pair is not available)
      Zero= 0,
      /// 65,537 for (Requested borrow token is not available) 
      One,  
      //  65,538 for (Requested pay token is not available)
      Two,
      //  65,539 for (_amount is too big)
      Three

}

#[repr(u16)]
pub enum Error {
    UniswapV2ZeroAddress = 0,
    UniswapV2PairExists = 1,
    UniswapV2PermissionedPairAccess = 2,
    UniswapV2InvalidContractAddress = 3,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait FLASHSWAPPER<Storage: ContractStorage>: ContractContext<Storage> {

    fn init(&mut self,weth: Key, dai: Key, uniswap_v2_factory: Key, contract_hash: Key ) {
        data::set_weth(weth);
        data::set_dai(dai);
        data::set_uniswap_v2_factory(uniswap_v2_factory);
        data::set_hash(contract_hash);
    }

    fn start_swap(&mut self, _token_borrow: Key, _amount: U256, _token_pay: Key, _user_data: String) {

        let mut is_borrowing_eth: bool=false;
        let mut is_paying_eth: bool=false;
        let mut token_borrow: Key = _token_borrow;
        let mut token_pay: Key = _token_pay;
        let eth = data::get_eth();
        let weth = data::get_weth();
        

        if token_borrow == eth {
            is_borrowing_eth = true;
            token_borrow = weth; // we'll borrow WETH from UniswapV2 but then unwrap it for the user
        }
        if token_pay == eth {
            is_paying_eth = true;
            token_pay = weth; // we'll wrap the user's ETH before sending it back to UniswapV2
        }
        if token_borrow == token_pay {
            self.simple_flash_loan(token_borrow, _amount, is_borrowing_eth, is_paying_eth, _user_data);
        } else if token_borrow == weth || token_pay == weth {
            self.simple_flash_swap(token_borrow, _amount, token_pay, is_borrowing_eth, is_paying_eth, _user_data);
        } else {
            self.traingular_flash_swap(token_borrow, _amount, token_pay, _user_data);
        }

    }

    fn uniswap_v2_call(&mut self, _sender: Key, _amount0: U256, _amount1: U256, _data: String) {
        // access control
        let permissioned_pair_address = data::get_permissioned_pair_address();
        if self.get_caller() == permissioned_pair_address{
            runtime::revert(Error::UniswapV2PermissionedPairAccess);
        }
        if _sender != data::get_hash(){
            runtime::revert(Error::UniswapV2InvalidContractAddress);
        }
                
        let decoded_data_without_commas: Vec<&str> = _data.split(',').collect();
        let _token_borrow_string = format!(
            "{}{}",
            "dictionary-",
            decoded_data_without_commas[1]
        );
        let _token_pay_string = format!(
            "{}{}",
            "dictionary-",
            decoded_data_without_commas[3]
        );

        let _swap_type: &str = decoded_data_without_commas[0];
        let _token_borrow :Key= Key::from_formatted_str(&_token_borrow_string).unwrap();
        let _amount: U256 = decoded_data_without_commas[2].parse().unwrap();
        let _token_pay :Key= Key::from_formatted_str(&_token_pay_string).unwrap();
        let _is_borrowing_eth: bool = decoded_data_without_commas[4].parse().unwrap();
        let _is_paying_eth: bool = decoded_data_without_commas[5].parse().unwrap();
        let _triangle_data: &str = decoded_data_without_commas[6];
        let _user_data: &str = decoded_data_without_commas[8];
        if _swap_type == "simple_loan" {
            self.simple_flash_loan_execute(_token_borrow, _amount, self.get_caller(), _is_borrowing_eth, _is_paying_eth, _user_data.into());
        } else if _swap_type == "simple_swap" {
            self.simple_flash_swap_execute(_token_borrow, _amount, _token_pay, self.get_caller(), _is_borrowing_eth, _is_paying_eth, _user_data.into());
        } else {
            self.traingular_flash_swap_execute(_token_borrow, _amount, _token_pay, _triangle_data.into(), _user_data.into());
        }
       
    }

    // @notice This function is used when the user repays with the same token they borrowed
    // @dev This initiates the flash borrow. See `simpleFlashLoanExecute` for the code that executes after the borrow.
    fn simple_flash_loan(&mut self, _token_borrow: Key, _amount: U256, _is_borrowing_eth: bool, _is_paying_eth: bool, _data: String) {
        let mut other_token =  data::get_dai();
        let weth = data::get_weth();
        let uniswap_v2_factory = data::get_uniswap_v2_factory();
        if _token_borrow != weth {
            other_token = weth;
        }
        let uniswap_v2_factory_hash_add_array = match uniswap_v2_factory {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };

        let uniswap_v2_factory_hash_add = ContractHash::new(uniswap_v2_factory_hash_add_array);

        let permissioned_pair_address: Key = call_contract(uniswap_v2_factory_hash_add, "pair", runtime_args!{"token0" => _token_borrow, "token1"  => other_token });
        data::set_permissioned_pair_address(permissioned_pair_address);
        let pair_address: Key = data::get_permissioned_pair_address();
          // in before 0 address was 0x0
        if pair_address == Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap() {
            runtime::revert(Error::UniswapV2ZeroAddress);
        }

        let pair_address_hash_add_array = match pair_address {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };

        let pair_address_hash_add = ContractHash::new(pair_address_hash_add_array);

        let token0: Key = call_contract(pair_address_hash_add, "token0", RuntimeArgs::new());
        let token1: Key = call_contract(pair_address_hash_add, "token1", RuntimeArgs::new());
        let amount0_out: U256;
        let amount1_out: U256;
        if _token_borrow == token0 {
            amount0_out = _amount;
        }
        else{
            amount0_out = 0.into();
        }
        if _token_borrow == token1 {
            amount1_out = _amount;
        }
        else{
            amount1_out = 0.into();
        }
        let data:String=format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}","simple_loan",",",_token_borrow,",",_amount,",",_token_borrow,",",_is_borrowing_eth,",",_is_paying_eth,",",",",_data);


        let _ret:Key = call_contract(pair_address_hash_add, "swap", runtime_args!{"amount0_out" => amount0_out, "amount1_out"  => amount1_out, "to" => data::get_hash(), "data" => data });
        
    }    


    // @notice This is the code that is executed after `simpleFlashLoan` initiated the flash-borrow
    // @dev When this code executes, this contract will hold the flash-borrowed _amount of _token_borrow
    fn simple_flash_loan_execute(&mut self, _token_borrow: Key, _amount: U256, _pair_address: Key, _is_borrowing_eth: bool, _is_paying_eth: bool, _user_data: String){

        let weth = data::get_weth();

        let weth_hash_add_array = match weth {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };

        let weth_hash_add = ContractHash::new(weth_hash_add_array);
        let eth = data::get_eth();
        if _is_borrowing_eth {
            let _ret:bool = call_contract(weth_hash_add, "withdraw", runtime_args!{"amount" => _amount});
        }
        let fee: U256 = ((_amount * 3) / 997) +  1;
        let amount_to_repay: U256 = _amount + fee;
        let token_borrowed: Key;
        let token_to_repay: Key;
        if _is_borrowing_eth {
            token_borrowed = eth;
        } else {
            token_borrowed =_token_borrow;
        }
        if _is_paying_eth {
            token_to_repay = eth;
        } else {
            token_to_repay = _token_borrow;
        }
          // do whatever the user wants
          self.execute(token_borrowed, _amount, token_to_repay, amount_to_repay, _user_data);

        // payback the loan
        // wrap the ETH if necessary

          if _is_paying_eth {
           let _ret:bool = call_contract(weth_hash_add, "deposit", runtime_args!{"amount" => amount_to_repay});
          }

          let _token_borrow_hash_add_array = match _token_borrow {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };

        let _token_borrow_hash_add = ContractHash::new(_token_borrow_hash_add_array);
        let _ret:bool = call_contract(_token_borrow_hash_add, "transfer", runtime_args!{"recipient"=>_pair_address ,"amount" => amount_to_repay});
    }

    /// @notice This function is used when either the _tokenBorrow or _tokenPay is WETH or ETH
    /// @dev Since ~all tokens trade against WETH (if they trade at all), we can use a single UniswapV2 pair to
    /// flash-borrow and repay with the requested tokens.
    /// @dev This initiates the flash borrow. See `simpleFlashSwapExecute` for the code that executes after the borrow.
    /// 
    fn simple_flash_swap( &mut self, token_borrow:Key, amount:U256 , token_pay:Key, is_borrowing_eth:bool , is_paying_eth:bool , user_data: String ) 
    {
        let uniswap_v2_factory_address:Key=get_key("factory_contract_hash").unwrap_or_revert();

        //convert Key to ContractHash
        let uniswap_v2_factory_address_hash_add_array = match uniswap_v2_factory_address {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let uniswap_v2_factory_contract_hash = ContractHash::new(uniswap_v2_factory_address_hash_add_array);

        let token_borrow_token_pay_pair_address:Key=runtime::call_contract(uniswap_v2_factory_contract_hash,"getpair",runtime_args!{"token0" => token_borrow,"token1" => token_pay});
        set_key("permissioned_pair_address",token_borrow_token_pay_pair_address);

        let pair_address:Key = token_borrow_token_pay_pair_address; // gas efficiency

        let address_0 = Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();

        if pair_address != address_0 {

            //convert Key to ContractHash
            let pair_address_hash_add_array = match pair_address {
                Key::Hash(package) => package,
                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
            };
            let pair_contract_hash = ContractHash::new(pair_address_hash_add_array); 

            let token0:Key=runtime::call_contract(pair_contract_hash,"token0",runtime_args!{});
            let token1:Key=runtime::call_contract(pair_contract_hash,"token1",runtime_args!{});
            
            let mut amount0_out:U256=0.into();
            let mut amount1_out:U256=0.into();

            if token_borrow == token0{
                amount0_out=amount;
            }

            if token_borrow == token1{
                amount1_out=amount;
            }

            let data:String=format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}","simple_swap",",",token_borrow,",",amount,",",token_pay,",",is_borrowing_eth,",",is_paying_eth,",",",",user_data);

            let flash_swapper_address:Key=get_key("self_contract_hash").unwrap_or_revert();
            let _result:bool=runtime::call_contract(pair_contract_hash,"swap",runtime_args!{"amount0_out" => amount0_out,"amount1_out" => amount1_out,"to" => flash_swapper_address,"data" => data});
        
        }
        else
        {
            // requested pair is not available
            runtime::revert(ApiError::User(FailureCode::Zero as u16));
        }
        
    }

    /// @notice This is the code that is executed after `simpleFlashSwap` initiated the flash-borrow
    /// @dev When this code executes, this contract will hold the flash-borrowed _amount of _tokenBorrow

    fn simple_flash_swap_execute( &mut self, token_borrow:Key, amount:U256 , token_pay:Key, _pair_address:Key, is_borrowing_eth:bool , is_paying_eth:bool , _user_data: String)
    {

        // unwrap WETH if necessary

        let iweth_address:Key=get_key("iweth_contract_hash").unwrap_or_revert();

        //convert Key to ContractHash
        let iweth_address_hash_add_array = match iweth_address {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let iweth_contract_hash = ContractHash::new(iweth_address_hash_add_array);

        if is_borrowing_eth == true
        {
            
            let _withdraw_result:bool=runtime::call_contract(iweth_contract_hash,"withdraw",runtime_args!{"amount" => amount});
        }

        // compute the amount of _tokenPay that needs to be repaid

        let permissioned_pair_address:Key = get_key("permissioned_pair_address").unwrap_or_revert(); // gas efficiency

        let pair_address:Key = permissioned_pair_address; // gas efficiency

        //convert Key to ContractHash
        let token_borrow_address_hash_add_array = match token_borrow {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token_borrow_contract_hash = ContractHash::new(token_borrow_address_hash_add_array);

        let pair_balance_token_borrow:U256=runtime::call_contract(token_borrow_contract_hash,"balanceof",runtime_args!{"owner" => pair_address});

        //convert Key to ContractHash
        let token_pay_address_hash_add_array = match token_pay {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token_pay_contract_hash = ContractHash::new(token_pay_address_hash_add_array);

        let pair_balance_token_pay:U256=runtime::call_contract(token_pay_contract_hash,"balanceOf",runtime_args!{"owner" => pair_address});

        let amount_1000:U256=1000.into();
        let amount_997:U256=997.into();
        let amount_1:U256=1.into();

        let amount_to_repay:U256 = ((amount_1000 * pair_balance_token_pay * amount) / (amount_997 * pair_balance_token_borrow)) + amount_1;

        // get the orignal tokens the user requested
        let mut _token_borrowed=Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let mut _token_to_repay=Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let eth:Key=get_key("eth").unwrap_or_revert();

        if is_borrowing_eth == true{

            _token_borrowed=eth;
        }
        else{

            _token_borrowed=token_borrow;
        }

        if is_paying_eth == true{

            _token_to_repay=eth;
        }
        else{

            _token_to_repay=token_pay;
        }

        // do whatever the user wants
        self.execute(_token_borrowed, amount, _token_to_repay, amount_to_repay, _user_data);

        // payback loan
        // wrap ETH if necessary

        if is_paying_eth==true 
        {
            let _deposit_result:bool=runtime::call_contract(iweth_contract_hash,"deposit",runtime_args!{"amount" => amount_to_repay});
        }

        let _result:bool=runtime::call_contract(token_pay_contract_hash,"transfer",runtime_args!{"recipient" => _pair_address, "amount" => amount_to_repay});

    }
    
    /// @notice This function is used when neither the _tokenBorrow nor the _tokenPay is WETH
    /// @dev Since it is unlikely that the _tokenBorrow/_tokenPay pair has more liquidaity than the _tokenBorrow/WETH and
    ///     _tokenPay/WETH pairs, we do a triangular swap here. That is, we flash borrow WETH from the _tokenPay/WETH pair,
    ///     Then we swap that borrowed WETH for the desired _tokenBorrow via the _tokenBorrow/WETH pair. And finally,
    ///     we pay back the original flash-borrow using _tokenPay.
    /// @dev This initiates the flash borrow. See `traingularFlashSwapExecute` for the code that executes after the borrow.
    /// 
    fn traingular_flash_swap(&mut self, token_borrow:Key, amount:U256 , token_pay:Key, user_data: String)
    {
        
        let uniswap_v2_factory_address:Key=get_key("factory_contract_hash").unwrap_or_revert();

        //convert Key to ContractHash
        let uniswap_v2_factory_address_hash_add_array = match uniswap_v2_factory_address {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let uniswap_v2_factory_contract_hash = ContractHash::new(uniswap_v2_factory_address_hash_add_array);

        let weth:Key=get_key("weth_contract_hash").unwrap_or_revert();
        let borrow_pair_address:Key=runtime::call_contract(uniswap_v2_factory_contract_hash,"getpair",runtime_args!{"token0" => token_borrow,"token1" => weth});
    
        let address_0:Key = Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap();

        if borrow_pair_address != address_0
        {
            let permissioned_pair_address:Key=runtime::call_contract(uniswap_v2_factory_contract_hash,"getpair",runtime_args!{"token0" => token_pay,"token1" => weth});
            let pay_pair_address:Key = permissioned_pair_address; // gas efficiency
          
            if pay_pair_address != address_0
            {
                // STEP 1: Compute how much WETH will be needed to get _amount of _tokenBorrow out of the _tokenBorrow/WETH pool

                //convert Key to ContractHash
                let token_borrow_address_hash_add_array = match token_borrow {
                    Key::Hash(package) => package,
                    _ => runtime::revert(ApiError::UnexpectedKeyVariant),
                };
                let token_borrow_contract_hash = ContractHash::new(token_borrow_address_hash_add_array);

                let pair_balance_token_borrow_before:U256=runtime::call_contract(token_borrow_contract_hash,"balanceOf",runtime_args!{"owner" => borrow_pair_address});

                if pair_balance_token_borrow_before >= amount 
                {
 
                    let pair_balance_token_borrow_after:U256 = pair_balance_token_borrow_before - amount;

                    //convert Key to ContractHash
                    let weth_address_hash_add_array = match weth {
                        Key::Hash(package) => package,
                        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
                    };
                    let weth_contract_hash = ContractHash::new(weth_address_hash_add_array);

                    let pair_balance_weth:U256=runtime::call_contract(weth_contract_hash,"balanceOf",runtime_args!{"owner" => borrow_pair_address});

                    let amount_1000:U256=1000.into();
                    let amount_997:U256=997.into();
                    let amount_1:U256=1.into();
            
                    let amount_of_weth:U256 = ((amount_1000 * pair_balance_weth * amount) / (amount_997 * pair_balance_token_borrow_after)) + amount_1;
    
                    // using a helper function here to avoid "stack too deep" :(
                    self.traingular_flash_swap_helper(token_borrow, amount, token_pay, borrow_pair_address, pay_pair_address, amount_of_weth, user_data);
                }
                else
                {
                    // _amount is too big
                    runtime::revert(ApiError::User(FailureCode::Three as u16));
                }
               
            }
            else
            {
                // Requested pay token is not available
                runtime::revert(ApiError::User(FailureCode::Two as u16));
            }
           
    
        }
        else
        {
            // Requested borrow token is not available
            runtime::revert(ApiError::User(FailureCode::One as u16));
        }
        
    }

    /// @notice Helper function for `traingularFlashSwap` to avoid `stack too deep` errors
    /// 
    fn traingular_flash_swap_helper(&mut self,token_borrow:Key,amount:U256, token_pay:Key, borrow_pair_address:Key , pay_pair_address:Key,amount_of_weth:U256, user_data:String)
    {
        //convert Key to ContractHash
        let pay_pair_address_hash_add_array = match pay_pair_address {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let pay_pair_contract_hash = ContractHash::new(pay_pair_address_hash_add_array);

        // Step 2: Flash-borrow _amountOfWeth WETH from the _tokenPay/WETH pool
        let token0:Key=runtime::call_contract(pay_pair_contract_hash,"token0",runtime_args!{});
        let token1:Key=runtime::call_contract(pay_pair_contract_hash,"token1",runtime_args!{});

        let mut amount0_out:U256=0.into();
        let mut amount1_out:U256=0.into();
        let weth:Key=get_key("weth_contract_hash").unwrap_or_revert();

        if weth == token0{
            amount0_out=amount_of_weth;
        }

        if weth == token1{
            amount1_out=amount_of_weth;
        }

        let triangle_data :String = format!("{}{}{}",borrow_pair_address,".", amount_of_weth);
        let data:String = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}","triangular_swap",",", token_borrow,",", amount,",", token_pay,",", false,",", false,",", triangle_data,",", user_data);

        // initiate the flash swap from UniswapV2
        let flash_swapper_address:Key=get_key("self_contract_hash").unwrap_or_revert();

        let _result:bool=runtime::call_contract(pay_pair_contract_hash,"swap",runtime_args!{"amount0_out" => amount0_out,"amount1_out" => amount1_out,"to" => flash_swapper_address,"data" => data});
        
    }    

    /// @notice This is the code that is executed after `traingularFlashSwap` initiated the flash-borrow
    /// @dev When this code executes, this contract will hold the amount of WETH we need in order to get _amount
    ///     _tokenBorrow from the _tokenBorrow/WETH pair.
    fn traingular_flash_swap_execute(&mut self, token_borrow:Key, amount:U256 , token_pay:Key, triangle_data:String, user_data:String)
    {

        // decode _triangleData
        let decoded_data_without_fullstop: Vec<&str> = triangle_data.split('.').collect();
        let borrow_pair_address_string = format!(
            "{}{}",
            "dictionary-",
            decoded_data_without_fullstop[0]
        );

        let borrow_pair_address :Key= Key::from_formatted_str(&borrow_pair_address_string).unwrap();
        let amount_of_weth: U256 = decoded_data_without_fullstop[1].parse().unwrap();
     
        //convert Key to ContractHash
        let borrow_pair_address_hash_add_array = match borrow_pair_address {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let borrow_pair_contract_hash = ContractHash::new(borrow_pair_address_hash_add_array);

        // Step 3: Using a normal swap, trade that WETH for _tokenBorrow
        let token0:Key = runtime::call_contract(borrow_pair_contract_hash,"token0",runtime_args!{});
        let token1:Key = runtime::call_contract(borrow_pair_contract_hash,"token1",runtime_args!{});

        let mut amount0_out:U256=0.into();
        let mut amount1_out:U256=0.into();

        if token_borrow == token0{
            amount0_out=amount;
        }

        if token_borrow == token1{
            amount1_out=amount;
        }

        // send our flash-borrowed WETH to the pair
        let weth:Key=get_key("weth_contract_hash").unwrap_or_revert();

        //convert Key to ContractHash
        let weth_address_hash_add_array = match weth {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let weth_contract_hash = ContractHash::new(weth_address_hash_add_array);

        let _weth_transfer_result:bool=runtime::call_contract(weth_contract_hash,"transfer",runtime_args!{"recipient" => borrow_pair_address,"amount" => amount_of_weth});

        let flash_swapper_address:Key=get_key("self_contract_hash").unwrap_or_revert();
        let _result:bool=runtime::call_contract(borrow_pair_contract_hash,"swap",runtime_args!{"amount0_out" => amount0_out,"amount1_out" => amount1_out,"to" => flash_swapper_address,"data" => user_data});
        
        
        // compute the amount of _tokenPay that needs to be repaid
        let permissioned_pair_address:Key=get_key("permissioned_pair_address").unwrap_or_revert();
        let pay_pair_address:Key = permissioned_pair_address; // gas efficiency

        let pair_balance_weth:U256=runtime::call_contract(weth_contract_hash,"balanceof",runtime_args!{"owner" => pay_pair_address});

        //convert Key to ContractHash
        let token_pay_address_hash_add_array = match token_pay {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token_pay_contract_hash = ContractHash::new(token_pay_address_hash_add_array);

        let pair_balance_token_pay:U256=runtime::call_contract(token_pay_contract_hash,"balanceof",runtime_args!{"owner" => pay_pair_address});

        let amount_1000:U256=1000.into();
        let amount_997:U256=997.into();
        let amount_1:U256=1.into();

        let amount_to_repay:U256 = ((amount_1000 * pair_balance_token_pay * amount_of_weth) / (amount_997 * pair_balance_weth)) + amount_1;

      
        // Step 4: Do whatever the user wants (arb, liqudiation, etc)
        // self.execute( token_borrow, amount, token_pay, amount_to_repay, user_data);

        // Step 5: Pay back the flash-borrow to the _tokenPay/WETH pool
        let _token_pay_transfer_result:bool=runtime::call_contract(token_pay_contract_hash,"transfer",runtime_args!{"recipient" => pay_pair_address,"amount" => amount_to_repay});

    }

    // @notice This is where the user's custom logic goes
    // @dev When this function executes, this contract will hold _amount of _token_borrow
    // @dev It is important that, by the end of the execution of this function, this contract holds the necessary
    //     amount of the original _token_pay needed to pay back the flash-loan.
    // @dev Paying back the flash-loan happens automatically by the calling function -- do not pay back the loan in this function
    // @dev If you entered `0x0` for _token_pay when you called `flashSwap`, then make sure this contract holds _amount ETH before this
    //     finishes executing
    // @dev User will override this function on the inheriting contract
    fn execute(&mut self,_token_borrow: Key, _amount: U256, _token_pay: Key, _amount_to_repay: U256, _user_data: String){

    }
}

    