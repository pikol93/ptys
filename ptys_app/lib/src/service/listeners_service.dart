import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:fpdart/fpdart.dart';
import 'package:ptys_app/src/logger.dart';
import 'package:ptys_app/src/rust/api/network.dart';
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

  TaskEither<void, void> initialize() => TaskEither.tryCatch(
        () async {
          logger.debug("Initializing listeners service");
          await ffi.subscribeListenerAdded(
              dartCallback: _onListenerAddedOrRemoved);
          await ffi.subscribeListenerRemoved(
              dartCallback: _onListenerAddedOrRemoved);
          await ffi.subscribeListenerChangedState(
              dartCallback: _onListenerChangedState);
        },
        (error, _) =>
            logger.error("Could not initialize listeners service: $error"),
      );

  TaskEither<void, void> addListener(int port) => TaskEither.tryCatch(
        () => ffi.addListener(port: port),
        (error, _) =>
            logger.error("Could not add listener for port $port: $error"),
      );

  TaskEither<void, void> removeListener(int id) => TaskEither.tryCatch(
        () => ffi.removeListener(id: id),
        (error, _) => logger.error("Could not remove listener $id: $error"),
      );

  TaskEither<void, void> startListener(int id) => TaskEither.tryCatch(
        () => ffi.startListener(id: id),
        (error, _) {
          logger.error("Could not start listener: $error");
        },
      );

  TaskEither<void, void> stopListener(int id) => TaskEither.tryCatch(
        () => ffi.stopListener(id: id),
        (error, _) => logger.error("Could not stop listener $id: $error"),
      );

  void _onListenerAddedOrRemoved(int listenerId) {
    ref.invalidate(listenersProvider);
  }

  void _onListenerChangedState(int listenerId, ListenerState state) {
    ref.invalidate(listenersProvider);
  }
}
