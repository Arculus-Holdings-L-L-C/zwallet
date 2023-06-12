import 'coin.dart';
import 'ycash.dart';
import 'zcash.dart';
import 'btc.dart';
import 'eth.dart';

CoinBase ycash = YcashCoin();
CoinBase zcash = ZcashCoin();
CoinBase btc = BTCCoin();
CoinBase eth = EthCoin();

final coins = [zcash, ycash, btc, eth];
