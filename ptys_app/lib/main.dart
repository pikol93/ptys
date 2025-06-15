import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:loggy/loggy.dart';
import 'package:ptys_app/src/gui/listeners_section.dart';
import 'package:ptys_app/src/logger.dart';
import 'package:ptys_app/src/rust/frb_generated.dart';
import 'package:ptys_app/src/service/listeners_service.dart';

Future<void> main() async {
  await RustLib.init();
  Loggy.initLoggy(logPrinter: const LoggerPrinter());

  runApp(
    const ProviderScope(
      child: InitializationNode(
        child: MyApp(),
      ),
    ),
  );
}

class InitializationNode extends ConsumerStatefulWidget {
  final Widget child;

  const InitializationNode({super.key, required this.child});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() =>
      _InitializationNodeState();
}

class _InitializationNodeState extends ConsumerState<InitializationNode> {
  @override
  void initState() {
    super.initState();
    ref.read(listenersServiceProvider).initialize().run();
  }

  @override
  Widget build(BuildContext context) => widget.child;
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('PTYS')),
        body: const ListenersSection(),
      ),
    );
  }
}
