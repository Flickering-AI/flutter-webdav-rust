import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_rust_app/src/rust/api/simple.dart';
import 'package:flutter_rust_app/src/rust/frb_generated.dart';
import 'package:path_provider/path_provider.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});
  static const platform = MethodChannel('samples.flutter.dev/battery');

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
            child: Column(
          children: [
            Text(
                'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`'),
            TextButton(
              child: Text("TEST WEBDAV"),
              onPressed: () async {
                final applicationDocumentsDirectory =
                    await getApplicationDocumentsDirectory();
                final externalStorageDirectory =
                    await getExternalStorageDirectory();
                final applicationSupportDirectory =
                    await getApplicationSupportDirectory();
                final temporaryDirectory = await getTemporaryDirectory();
                print("applicationDocumentsDirectory:" +
                    applicationDocumentsDirectory.path +
                    "|externalStorageDirectory:" +
                    (externalStorageDirectory?.path).toString() +
                    "|temporaryDirectory:" +
                    (temporaryDirectory.path).toString());
                webdav(dir: applicationDocumentsDirectory.path + "/../");
                // platform.invokeMethod("test");
              },
            ),
          ],
        )),
      ),
    );
  }
}
