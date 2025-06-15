import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:ptys_app/src/extensions/async_value.dart';
import 'package:ptys_app/src/extensions/number.dart';
import 'package:ptys_app/src/logger.dart';
import 'package:ptys_app/src/rust/api/network.dart';
import 'package:ptys_app/src/service/listeners_service.dart';

class ListenersSection extends StatelessWidget {
  const ListenersSection({super.key});

  @override
  Widget build(BuildContext context) => const Column(
        children: [
          AddListenerForm(),
          ListenersList(),
        ],
      );
}

class AddListenerForm extends ConsumerStatefulWidget {
  const AddListenerForm({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() =>
      _AddListenerFormState();
}

class _AddListenerFormState extends ConsumerState<AddListenerForm> with Logger {
  final _formKey = GlobalKey<FormState>();

  late final TextEditingController _portController;

  @override
  void initState() {
    super.initState();
    _portController = TextEditingController();
  }

  @override
  Widget build(BuildContext context) => Form(
        key: _formKey,
        child: Column(
          children: [
            TextFormField(
              controller: _portController,
              validator: _validatePort,
            ),
            ElevatedButton(
              onPressed: _onSubmitPressed,
              child: const Text("Submit"),
            ),
          ],
        ),
      );

  String? _validatePort(String? value) {
    if (value == null || value.isEmpty) {
      return "Port cannot be empty.";
    }

    final parsed = int.tryParse(value);
    if (parsed == null) {
      return "Cannot parse port.";
    }

    if (!parsed.inRangeInclusive(1, 65535)) {
      return "Port out of range.";
    }

    return null;
  }

  Future<void> _onSubmitPressed() async {
    if (!_formKey.currentState!.validate()) {
      logger.debug("Form not validated.");
      return;
    }

    final port = int.parse(_portController.text);
    ref.read(listenersServiceProvider).addListener(port).run();
    _portController.clear();
  }
}

class ListenersList extends ConsumerWidget with Logger {
  const ListenersList({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) =>
      ref.watch(listenersProvider).whenDataOrDefault(
            context,
            (listeners) => Column(
              children: listeners
                  .map(
                    (listener) => Row(
                      children: [
                        Text(listener.id.toString()),
                        Text(listener.port.toString()),
                        Text(listener.state.toString()),
                        if (listener.state == ListenerState.disabled)
                          ElevatedButton(
                            onPressed: () => _onStartPressed(ref, listener.id),
                            child: const Text("Start"),
                          )
                        else
                          ElevatedButton(
                            onPressed: () => _onStopPressed(ref, listener.id),
                            child: const Text("Stop"),
                          ),
                        ElevatedButton(
                          onPressed: () => _onRemovePressed(ref, listener.id),
                          child: const Text("Remove"),
                        ),
                      ],
                    ),
                  )
                  .toList(),
            ),
          );

  Future<void> _onStartPressed(WidgetRef ref, int id) async {
    logger.debug("Start pressed for listener $id");
    ref.read(listenersServiceProvider).startListener(id).run();
  }

  Future<void> _onStopPressed(WidgetRef ref, int id) async {
    logger.debug("Stop pressed for listener $id");
    ref.read(listenersServiceProvider).stopListener(id).run();
  }

  Future<void> _onRemovePressed(WidgetRef ref, int id) async {
    logger.debug("Remove pressed for listener $id");
    ref.read(listenersServiceProvider).removeListener(id).run();
  }
}
