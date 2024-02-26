# Proveably Random Raffle Contracts

## About

This code is to create a proveably random smart contract lottery.

## What Do We Want It to Do?

1. User can enter by paying for a ticket
    1. the Ticket fees are going to go to the winner during the draw
2. After X period of time, the lottery will automatically draw a winner
    1. And this will be doen programmatically
3. Using Chainlink VRF & Chainlink Automation
    1. Chainlink VRF -> Randomness
    2. Chainlink Automation -> time based trigger

## Tests

1. Write some deploy scripts
2. Write our tests
    1. Work on a local chain
    2. Forked Testnet
    3. Forked Mainnet
