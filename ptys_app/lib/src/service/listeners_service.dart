import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ptys_app/src/logger.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:ptys_app/src/rust/api/network.dart' as ffi;

part "listeners_service.g.dart";

@Riverpod(keepAlive: true)
ListenersService listenersService(Ref ref) {
  return ListenersService(ref: ref);
}

@riverpod
Future<List<ffi.Listener>> listeners(Ref ref) {
  return ffi.getListeners();
}

class ListenersService with Logger {
  final Ref ref;

  ListenersService({required this.ref});

  Future<void> initialize() async {
    logger.debug("Initializing listeners service");
    ffi.subscribeListenerAdded(dartCallback: _onListenerAdded);
  }

  Future<void> addListener(int port) async {
    final id = await ffi.addListener(port: port);
    logger.debug("Added listener with ID $id");
  }

  Future<void> removeListener(int id) async {
    logger.warning("TODO");
  }

  Future<void> startListener(int id) async {
    logger.warning("TODO");
    await ffi.startListener(id: id);
  }

  Future<void> stopListener(int id) async {
    logger.warning("TODO");
  }

  Future<void> _onListenerAdded(int a) async {
    logger.debug("Listeners: $listeners");
    ref.invalidate(listenersProvider);
  }
}
