# This is a test contract for a YouTube video.

# Juno Create

*Store*

junod tx wasm store julian.wasm --from deploy --fees 1500000ujuno --gas 20000000 -y

hash: 

code id: 4630



*Instantiate*

junod tx wasm instantiate 4630 '{}' --label test_dep_for_ham --admin deploy --from deploy --fees 1500000ujuno --gas 20000000 -y

tx hash: 

contract address: 


# Juno Migrate

*Store*

junod tx wasm store julian.wasm --from deploy --fees 1500000ujuno --gas 20000000 -y

tx hash: 

code id: 



*Migrate* 

junod tx wasm migrate juno1fucsaa4mukx86z5sfxm3k3445eh8c4vcpejzu93457wufh4s6zms4qz6ra 4638 '{"migrate_msg":{}}' --from deploy --fees 1500000ujuno --gas 20000000 -y

tx hash: 


# Query Contract

*Query Listing Count*

junod query wasm contract-state smart juno1fucsaa4mukx86z5sfxm3k3445eh8c4vcpejzu93457wufh4s6zms4qz6ra '{"listing_count":{}}'

