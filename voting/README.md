# Voting Contract

The purpose of this contract is solely for validators to vote on whether to unlock
token transfer. Validators can call `vote` to vote for yes with the amount of stake they wish
to put on the vote. If there are more than 2/3 of the stake at any given moment voting for yes, the voting is done.
After the voting is finished, no one can further modify the contract.




## init 用deploy_voting.sh部署
```bash
near call transfer-vote.gao.test.near new --accountId gao.test.near
```

## ping
```bash
near call dev-1670383319826-58645144100625 ping --accountId gao.test.near
```

## vote
```bash
near call dev-1670383319826-58645144100625 vote '{"is_vote":true}' --accountId gao.test.near
```


## get_result
```bash
near view dev-1670383319826-58645144100625 get_result
```

## get_total_voted_stake
```bash
near view dev-1670383319826-58645144100625 get_total_voted_stake
```


## get_votes
```bash
near view dev-1670383319826-58645144100625 get_votes
```
