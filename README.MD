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

tx hash: 1D4A433128CAD0BC49CF21A75BD836BC075FF9F0755FB6FE98DF0C0A96DCFB91

code id: 4638

https://www.mintscan.io/juno/tx/1D4A433128CAD0BC49CF21A75BD836BC075FF9F0755FB6FE98DF0C0A96DCFB91/?height=23140032

*Migrate* 

junod tx wasm migrate juno1fucsaa4mukx86z5sfxm3k3445eh8c4vcpejzu93457wufh4s6zms4qz6ra 4638 '{"migrate_msg":{}}' --from deploy --fees 1500000ujuno --gas 20000000 -y

tx hash: A266305153292B054DA5AE7A6BB113C3EEE81C5C04E8D4C59648DB3473747A1C

https://www.mintscan.io/juno/tx/A266305153292B054DA5AE7A6BB113C3EEE81C5C04E8D4C59648DB3473747A1C?height=23140104

# Query Contract

*Query Listing Count*

junod query wasm contract-state smart juno1fucsaa4mukx86z5sfxm3k3445eh8c4vcpejzu93457wufh4s6zms4qz6ra '{"listing_count":{}}'

