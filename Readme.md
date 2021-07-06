# Bit-Decomposition of Secret-Shared Values
* This repo implements the naive protocol described in "Linear Round Bit-Decomposition of Secret-Shared Values" by Thijs Veugen, in *IEEE Transactions on Information Forensics and Security, Vol. 10, No. 3, March 2015*.
* It simulates two parties with variables, but I will soon add netcode to facilitate real networking conditions.
* Also I will soon implement the more-efficient protocol described in the paper.

# How it works:
1. Two parties securely generate a random number and shares of its bits. This could be accomplished by an addition operation of secret-shared integers, and their bits.
2. The parties sum up the secret value to be decomposed, and the random number.
3. The parties reveal this sum without revealing anything about the target number.
4. the parties securely subtract the random number from the revealed number, bit by bit, producing shares of the target number's bits.

After running the protocol, binary-based operations such as secure comparisson, or division by a power of two, can be computed on the bit sharesk

Uses [Snips.ai's Secret Sharing library](https://github.com/snipsco/rust-threshold-secret-sharing)
