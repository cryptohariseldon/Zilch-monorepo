import json
import csv

def Alice_publish_request(inputs_path, program_path, program_hash):
    c={}
    c['inputs']=open(inputs_path).read()
    c['program']=open(program_path).read()
    c['program_hash']=program_hash

    with open("./Alice/compiled_inputs.json", 'w') as writer:
        json.dump(c, writer)


import urllib.request
import json

def prover_get_inputs(cid):
    # content ID
    # cid = "bafkreifacnxyt7p45fat7nirzcqhw45ac4lphyqb5ltyrnpttq3iqm4miu"
    # read .json
    with urllib.request.urlopen(f"https://dweb.link/ipfs/{cid}") as url:
        data = json.loads(url.read())

    with open("./Bob/inputs/fib.inputs" , 'w') as wr:
        wr.write(data['inputs'])

    with open("./Bob/inputs/fib.masm", 'w') as wr:
        wr.write(data['program'])
    print(data['program_hash'])
