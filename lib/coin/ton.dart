import 'package:flutter/material.dart';

import "coin.dart";

class TonCoin extends CoinBase {
  int coin = 4;
  bool transparentOnly = true;
  String name = "Ton";
  String symbol = "\u{1F48E}";
  String currency = "the-open-network";
  int coinIndex = 9999;
  String ticker = "TON";
  String dbName = "ton.db";
  String image = 'assets/ton.png';
  List<LWInstance> lwd = [LWInstance("Ton Center", "https://toncenter.com")];
  bool supportsUA = false;
  bool supportsMultisig = false;
  bool utxo = false;
  List<double> weights = [0.001, 0.01, 0.1];
  List<String> blockExplorers = ["https://ton.cx/tx"];
  bool supportsLedger = false;
}
