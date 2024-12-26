import "package:flutter/material.dart";
import "package:flutter_riverpod/flutter_riverpod.dart";
import "package:fpdart/fpdart.dart";

extension AsyncValueExtension<T> on AsyncValue<T> {
  Widget whenDataOrDefault(
    BuildContext context,
    Widget Function(T data) widgetBuilder,
  ) =>
      when(
        data: widgetBuilder,
        error: (error, stack) => Text("$error $stack"),
        loading: () => const Center(child: CircularProgressIndicator()),
      );

  Option<T> toOption() => when(
        data: Option.of,
        error: (error, stackTrace) => Option.none(),
        loading: Option.none,
      );
}