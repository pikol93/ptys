import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ptys_app/src/rust/api/network.dart';
import 'package:ptys_app/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(
    const ProviderScope(
      child: MyApp(),
    ),
  );
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Column(
            children: [
              ElevatedButton(
                onPressed: _onAddListenerPressed,
                child: const Text("Add listener"),
              ),
              ElevatedButton(
                onPressed: _onStartListenerPressed,
                child: const Text("Start listener"),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Future<void> _onAddListenerPressed() async {
    await addListener(port: 1234);
  }

  Future<void> _onStartListenerPressed() async {
    await startListener(id: 0);
  }
}
