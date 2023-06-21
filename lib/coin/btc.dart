import 'package:flutter/material.dart';

import "coin.dart";

class BTCCoin extends CoinBase {
  int coin = 2;
  bool transparentOnly = true;
  String name = "Bitcoin";
  String symbol = "\u20BF";
  String currency = "bitcoin";
  int coinIndex = 0;
  String ticker = "BTC";
  String dbName = "btc.db";
  String image = 'assets/bitcoin.png';
  List<LWInstance> lwd = [
    LWInstance("Blockstream", "ssl://electrum.blockstream.info:50002")
  ];
  bool supportsUA = false;
  bool supportsMultisig = false;
  bool utxo = true;
  List<double> weights = [0.0001, 0.001, 0.01];
  List<String> blockExplorers = ["https://blockstream.info/tx"];
  bool supportsLedger = false;
}
