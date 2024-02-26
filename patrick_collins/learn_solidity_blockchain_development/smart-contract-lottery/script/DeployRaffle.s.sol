// SPDX-License-Identifier: MIT

pragma solidity ^0.8.18;

import {Script} from "forge-std/Script.sol";
import {Raffle} from "../src/Raffle.sol";
import {HelperConfig} from "./HelperConfig.s.sol";
import {CreateSubscription, FundSubscription, AddConsumer} from "./Interactions.s.sol";

contract DeployRaffle is Script {
    function run() external returns (Raffle, HelperConfig) {
        HelperConfig helperConfig = new HelperConfig();
        (
            address vrfCoordinater,
            address link,
            bytes32 gasLane,
            uint32 callbackGasLimit,
            uint64 subscriptionId,
            uint256 entranceFee,
            uint256 interval,
            uint256 deployerKey
        ) = helperConfig.activeNetworkConfig();

        if (subscriptionId == 0) {
            CreateSubscription createSubscription = new CreateSubscription();
            subscriptionId = createSubscription.createSubscription(
                vrfCoordinater,
                deployerKey
            );

            FundSubscription fundSubscription = new FundSubscription();
            fundSubscription.fundSubscription(
                vrfCoordinater,
                link,
                subscriptionId,
                deployerKey
            );
        }

        vm.startBroadcast();

        Raffle raffle = new Raffle(
            vrfCoordinater,
            gasLane,
            callbackGasLimit,
            subscriptionId,
            entranceFee,
            interval
        );

        vm.stopBroadcast();

        AddConsumer addConsumer = new AddConsumer();
        addConsumer.addConsumer(
            address(raffle),
            vrfCoordinater,
            subscriptionId,
            deployerKey
        );

        return (raffle, helperConfig);
    }
}
