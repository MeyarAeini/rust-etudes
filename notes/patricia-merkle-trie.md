# [What are Patricia Merkle Tries?](https://docs.alchemy.com/docs/patricia-merkle-tries)
Ethereum makes use of a data structure called a radix trie, also referred to as a Patricia trie or a **radix tree** and combines this data structure with a **Merkle tree** to create a **Patricia Merkle Trie**.

```note
Patricia Trie + Merkle Tree = Patricia Merkle Trie
```

## Patricia??
- P = Practical
- A = Algorithm
- T = To
- R = Retrieve
- I = Information
- C = Coded
- I = In
- A = Alphanumeric

## Why Does Ethereum Use a Merkle Patricia Trie?

It makes sense that permanent data, like mined transactions, and ephemeral data, like Ethereum accounts (balance, nonce, etc), should be stored separately. **Merkle trees**, again, are perfect for **permanent data**. **PMTs** are perfect for **ephemeral data**, which Ethereum is in plenty supply of.

Unlike transaction history, Ethereum account state needs to be frequently updated. The balance and nonce of accounts is often changed, and what’s more, new accounts are frequently inserted, and keys in storage are frequently inserted and deleted.


Initially, the idea was that 100% of the state Merkle-Patricia-trie (MPT) would fit on the RAM of a standard device. This is not the case anymore as the **state has grown to over 900 GB and is expected to grow approx 50–100 GB per year**, which is unreasonably large for anyone to fit in RAM, so most nodes have turned to SSDs to store state. Historically, SSDs do not improve as quickly as the state size is growing, so this cost will continue to impact the decentralization of the network. 