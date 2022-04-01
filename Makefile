uniswap_core_directory = ./

erc20_contract = ${uniswap_core_directory}erc20/
factory_contract = ${uniswap_core_directory}factory/
flash_swapper_contract = ${uniswap_core_directory}flash-swapper/
pair_contract = ${uniswap_core_directory}pair/
wcspr_contract = ${uniswap_core_directory}wcspr/

wasm_src_path = target/wasm32-unknown-unknown/release/
wasm_dest_factory_path = ${uniswap_core_directory}factory/factory-tests/wasm/
wasm_dest_pair_path = ${uniswap_core_directory}pair/pair-tests/wasm/
wasm_dest_flash_swapper_path = ${uniswap_core_directory}flash-swapper/flash_swapper-tests/wasm/


all:
	# Build erc20
	cd ${erc20_contract} && make build-contract && make build-test-contract && make build-test-contract2

	# Build wcspr
	cd ${wcspr_contract} && make build-contract && make build-test-contract && make build-test-contract2

	# Build factory
	cd ${factory_contract} && make build-contract

	# Build flash swapper
	cd ${flash_swapper_contract} && make build-contract

	# Build pair
	cd ${pair_contract} && make build-contract && make build-test-contract && make build-test-contract2

	# copy wasm files
	make copy-wasm-file
clean:
	# clean erc20
	cd ${erc20_contract} && make clean

	# clean wcspr
	cd ${wcspr_contract} && make clean

	# clean factory
	cd ${factory_contract} && make clean

	# clean flash swapper
	cd ${flash_swapper_contract} && make clean

	# clean pair
	cd ${pair_contract} && make clean




# copy wasm to required directory
copy-wasm-file:
	cp ${erc20_contract}${wasm_src_path}*.wasm ${wasm_dest_factory_path}
	cp ${erc20_contract}${wasm_src_path}*.wasm ${wasm_dest_flash_swapper_path}
	cp ${erc20_contract}${wasm_src_path}*.wasm ${wasm_dest_pair_path}
	cp ${wcspr_contract}${wasm_src_path}*.wasm ${wasm_dest_factory_path}
	cp ${wcspr_contract}${wasm_src_path}*.wasm ${wasm_dest_flash_swapper_path}
	cp ${wcspr_contract}${wasm_src_path}*.wasm ${wasm_dest_pair_path}

	cp ${pair_contract}${wasm_src_path}*.wasm ${wasm_dest_factory_path}
	cp ${pair_contract}${wasm_src_path}*.wasm ${wasm_dest_flash_swapper_path}

	cp ${flash_swapper_contract}${wasm_src_path}*.wasm ${wasm_dest_factory_path}
	cp ${flash_swapper_contract}${wasm_src_path}*.wasm ${wasm_dest_pair_path}

	cp ${factory_contract}${wasm_src_path}*.wasm ${wasm_dest_flash_swapper_path}
	cp ${factory_contract}${wasm_src_path}*.wasm ${wasm_dest_pair_path}


# run all tests sequentially
test:
	# Test ERC20
	cd ${erc20_contract} && make test

	# Test WCSPR
	cd ${wcspr_contract} && make test

	# Test Factory
	cd ${factory_contract} && make test

	# Test Flashswapper
	cd ${flash_swapper_contract} && make test

	# Test Pair
	cd ${pair_contract} && make test







