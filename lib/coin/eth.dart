import 'package:flutter/material.dart';

import "coin.dart";

class EthCoin extends CoinBase {
  int coin = 3;
  bool transparentOnly = true;
  String name = "Ethereum";
  String symbol = "\u039E";
  String currency = "ethereum";
  int coinIndex = 60;
  String ticker = "ETH";
  String dbName = "eth.db";
  String image = 'assets/eth.png';
  List<LWInstance> lwd = [
    LWInstance("MyEtherWallet", "https://nodes.mewapi.io/rpc/eth")
  ];
  bool supportsUA = false;
  bool supportsMultisig = false;
  bool utxo = false;
  List<double> weights = [0.001, 0.01, 0.1];
  List<String> blockExplorers = ["https://etherscan.io/tx"];
  bool supportsLedger = false;
}
