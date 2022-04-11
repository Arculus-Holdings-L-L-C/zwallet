import 'dart:io';

import 'package:ZYWallet/main.dart';
import 'package:flutter/material.dart';
import 'package:path/path.dart';
import 'package:share_plus/share_plus.dart';
import 'package:sqflite/sqflite.dart';

class LWInstance {
  String name;
  String url;

  LWInstance(this.name, this.url);
}

abstract class CoinBase {
  String get app;
  String get symbol;
  String get currency;
  String get ticker;
  String get explorerUrl;
  AssetImage get image;
  String get dbName;
  late Database db;
  List<LWInstance> get lwd;
  bool get supportsUA;
  bool get supportsMultisig;
  List<double> get weights;

  Future<void> open(String dbPath) async {
    final path = join(dbPath, dbName);
    db = await openDatabase(path);
  }

  Future<void> delete(String dbPath) async {
    final path = join(dbPath, dbName);
    await deleteDatabase(path);
  }

  Future<void> export(String dbPath) async {
    if (isMobile()) {
      final path = join(dbPath, dbName);
      db = await openDatabase(path);
      await db.close();
      await Share.shareFiles([path], subject: dbName);
    }
  }
}